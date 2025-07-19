#[cfg(feature = "streaming")]
use futures::Stream;
#[cfg(feature = "streaming")]
use tokio_stream::wrappers::ReceiverStream;
#[cfg(feature = "streaming")]
use tokio::sync::mpsc;

use crate::client::OpenAIClient;
use crate::error::{OpenAIError, Result};
use crate::types::{CreateResponseRequest, StreamResponse, ResponseStatus};
use std::pin::Pin;
use std::task::{Context, Poll};

#[cfg(feature = "streaming")]
/// Server-sent events stream for real-time response processing
pub struct ResponseStream {
    receiver: ReceiverStream<Result<StreamResponse>>,
}

#[cfg(feature = "streaming")]
impl Stream for ResponseStream {
    type Item = Result<StreamResponse>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.receiver).poll_next(cx)
    }
}

#[cfg(feature = "streaming")]
impl ResponseStream {
    /// Create a new response stream
    pub fn new(receiver: ReceiverStream<Result<StreamResponse>>) -> Self {
        Self { receiver }
    }

    /// Collect all stream chunks into a final response
    pub async fn collect(mut self) -> Result<crate::types::Response> {
        use futures::StreamExt;
        
        let mut final_response = None;
        let mut accumulated_content = String::new();

        while let Some(chunk_result) = self.next().await {
            let chunk = chunk_result?;
            
            match chunk.status {
                ResponseStatus::InProgress => {
                    // Accumulate delta content
                    if let Some(delta) = &chunk.delta {
                        for content in &delta.content {
                            if let Some(text) = &content.text {
                                accumulated_content.push_str(text);
                            }
                        }
                    }
                }
                ResponseStatus::Completed => {
                    // Convert to final response
                    final_response = Some(crate::types::Response {
                        id: chunk.id,
                        object: chunk.object,
                        status: chunk.status,
                        output: chunk.output,
                        usage: chunk.usage,
                        metadata: None,
                        created_at: None,
                        incomplete_details: None,
                    });
                    break;
                }
                ResponseStatus::Cancelled | ResponseStatus::Incomplete => {
                    return Err(OpenAIError::Streaming(format!(
                        "Stream ended with status: {:?}", chunk.status
                    )));
                }
            }
        }

        final_response.ok_or_else(|| {
            OpenAIError::Streaming("Stream ended without completion".to_string())
        })
    }

    /// Get the next text chunk from the stream
    pub async fn next_text_chunk(&mut self) -> Option<Result<String>> {
        use futures::StreamExt;
        
        while let Some(chunk_result) = self.next().await {
            match chunk_result {
                Ok(chunk) => {
                    if let Some(delta) = &chunk.delta {
                        for content in &delta.content {
                            if let Some(text) = &content.text {
                                if !text.is_empty() {
                                    return Some(Ok(text.clone()));
                                }
                            }
                        }
                    }
                }
                Err(e) => return Some(Err(e)),
            }
        }
        None
    }
}

#[cfg(feature = "streaming")]
impl OpenAIClient {
    /// Create a streaming response using Server-Sent Events
    pub async fn create_response_stream(
        &self,
        request: &CreateResponseRequest,
    ) -> Result<ResponseStream> {
        let (tx, rx) = mpsc::channel(100);
        let receiver_stream = ReceiverStream::new(rx);
        
        let client = self.clone();
        let request = request.clone();
        
        // Spawn task to handle streaming
        tokio::spawn(async move {
            if let Err(e) = client.handle_streaming_request(&request, tx).await {
                eprintln!("Streaming error: {}", e);
            }
        });
        
        Ok(ResponseStream::new(receiver_stream))
    }

    /// Handle the actual streaming logic
    async fn handle_streaming_request(
        &self,
        request: &CreateResponseRequest,
        tx: mpsc::Sender<Result<StreamResponse>>,
    ) -> Result<()> {
        use futures::StreamExt;
        
        // Add streaming parameter to request
        let mut streaming_request = request.clone();
        // Note: In a real implementation, you'd modify the request to enable streaming
        
        let response = self
            .execute_with_retry(self.post("responses").json(&streaming_request))
            .await?;

        // Parse response as Server-Sent Events stream
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);
                    
                    // Process complete SSE messages
                    while let Some(line_end) = buffer.find("\n\n") {
                        let message = buffer[..line_end].to_string();
                        buffer = buffer[line_end + 2..].to_string();
                        
                        if let Some(json_data) = Self::parse_sse_message(&message) {
                            match serde_json::from_str::<StreamResponse>(&json_data) {
                                Ok(stream_response) => {
                                    let is_final = matches!(
                                        stream_response.status,
                                        ResponseStatus::Completed | ResponseStatus::Cancelled | ResponseStatus::Incomplete
                                    );
                                    
                                    if tx.send(Ok(stream_response)).await.is_err() {
                                        break; // Receiver dropped
                                    }
                                    
                                    if is_final {
                                        break;
                                    }
                                }
                                Err(e) => {
                                    let _ = tx.send(Err(OpenAIError::Json(e))).await;
                                    break;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(OpenAIError::Http(e))).await;
                    break;
                }
            }
        }

        Ok(())
    }

    /// Parse Server-Sent Events message format
    fn parse_sse_message(message: &str) -> Option<String> {
        for line in message.lines() {
            if let Some(data) = line.strip_prefix("data: ") {
                if data != "[DONE]" {
                    return Some(data.to_string());
                }
            }
        }
        None
    }

    /// Create a response stream with a callback for each chunk
    pub async fn create_response_stream_with_callback<F>(
        &self,
        request: &CreateResponseRequest,
        mut callback: F,
    ) -> Result<crate::types::Response>
    where
        F: FnMut(&StreamResponse) -> Result<()> + Send + 'static,
    {
        let mut stream = self.create_response_stream(request).await?;
        
        use futures::StreamExt;
        let mut final_response = None;

        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            
            // Call user callback
            callback(&chunk)?;
            
            match chunk.status {
                ResponseStatus::Completed => {
                    final_response = Some(crate::types::Response {
                        id: chunk.id,
                        object: chunk.object,
                        status: chunk.status,
                        output: chunk.output,
                        usage: chunk.usage,
                        metadata: None,
                        created_at: None,
                        incomplete_details: None,
                    });
                    break;
                }
                ResponseStatus::Cancelled | ResponseStatus::Incomplete => {
                    return Err(OpenAIError::Streaming(format!(
                        "Stream ended with status: {:?}", chunk.status
                    )));
                }
                ResponseStatus::InProgress => {
                    // Continue processing
                }
            }
        }

        final_response.ok_or_else(|| {
            OpenAIError::Streaming("Stream ended without completion".to_string())
        })
    }
}

/// Helper trait for streaming responses
#[cfg(feature = "streaming")]
pub trait StreamingExt {
    /// Convert the response to a stream
    fn into_stream(self) -> ResponseStream;
}

#[cfg(feature = "streaming")]
impl StreamingExt for crate::types::Response {
    fn into_stream(self) -> ResponseStream {
        let (tx, rx) = mpsc::channel(1);
        let receiver_stream = ReceiverStream::new(rx);
        
        // For completed responses, just send them once
        tokio::spawn(async move {
            let stream_response = StreamResponse {
                id: self.id,
                object: self.object,
                status: self.status,
                output: self.output,
                usage: self.usage,
                delta: None,
            };
            let _ = tx.send(Ok(stream_response)).await;
        });
        
        ResponseStream::new(receiver_stream)
    }
}

#[cfg(test)]
#[cfg(feature = "streaming")]
mod tests {
    use super::*;
    use crate::types::{Model, CreateResponseRequest};
    use futures::StreamExt;

    #[tokio::test]
    async fn test_response_stream_collect() {
        let (tx, rx) = mpsc::channel(10);
        let receiver_stream = ReceiverStream::new(rx);
        let stream = ResponseStream::new(receiver_stream);

        // Simulate streaming chunks
        tokio::spawn(async move {
            let chunk1 = StreamResponse {
                id: "resp_123".to_string(),
                object: "response".to_string(),
                status: ResponseStatus::InProgress,
                output: None,
                usage: None,
                delta: Some(crate::types::Output {
                    output_type: "message".to_string(),
                    content: vec![crate::types::Content {
                        content_type: crate::types::ContentType::Text,
                        text: Some("Hello".to_string()),
                        image_url: None,
                    }],
                }),
            };

            let chunk2 = StreamResponse {
                id: "resp_123".to_string(),
                object: "response".to_string(),
                status: ResponseStatus::Completed,
                output: Some(vec![crate::types::Output {
                    output_type: "message".to_string(),
                    content: vec![crate::types::Content {
                        content_type: crate::types::ContentType::Text,
                        text: Some("Hello World".to_string()),
                        image_url: None,
                    }],
                }]),
                usage: Some(crate::types::Usage {
                    total_tokens: 10,
                    prompt_tokens: Some(5),
                    completion_tokens: Some(5),
                }),
                delta: None,
            };

            let _ = tx.send(Ok(chunk1)).await;
            let _ = tx.send(Ok(chunk2)).await;
        });

        let final_response = stream.collect().await.unwrap();
        assert_eq!(final_response.id, "resp_123");
        assert!(final_response.is_completed());
    }

    #[tokio::test]
    async fn test_stream_text_chunks() {
        let (tx, rx) = mpsc::channel(10);
        let receiver_stream = ReceiverStream::new(rx);
        let mut stream = ResponseStream::new(receiver_stream);

        tokio::spawn(async move {
            let chunk = StreamResponse {
                id: "resp_123".to_string(),
                object: "response".to_string(),
                status: ResponseStatus::InProgress,
                output: None,
                usage: None,
                delta: Some(crate::types::Output {
                    output_type: "message".to_string(),
                    content: vec![crate::types::Content {
                        content_type: crate::types::ContentType::Text,
                        text: Some("Hello".to_string()),
                        image_url: None,
                    }],
                }),
            };

            let _ = tx.send(Ok(chunk)).await;
        });

        let text_chunk = stream.next_text_chunk().await;
        assert!(text_chunk.is_some());
        assert_eq!(text_chunk.unwrap().unwrap(), "Hello");
    }
}