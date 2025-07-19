#[cfg(feature = "simd")]
use simd_json;

use crate::types::{Response, LazyResponse};
use crate::error::{OpenAIError, Result};

/// SIMD-optimized JSON parsing utilities
#[cfg(feature = "simd")]
pub struct SimdJsonParser;

#[cfg(feature = "simd")]
impl SimdJsonParser {
    /// Parse JSON using SIMD optimizations when available
    pub fn parse_response(mut json_bytes: Vec<u8>) -> Result<Response> {
        simd_json::from_slice(&mut json_bytes).map_err(|e| {
            OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("SIMD JSON error: {}", e)
            )))
        })
    }

    /// Parse lazy response with SIMD
    pub fn parse_lazy_response(mut json_bytes: Vec<u8>) -> Result<LazyResponse<'static>> {
        // For SIMD parsing, we need to ensure the buffer is mutable and owned
        let json_str = unsafe {
            // SAFETY: We're converting Vec<u8> to String, which is safe if it's valid UTF-8
            // simd_json will validate this for us
            String::from_utf8_unchecked(json_bytes.clone())
        };
        
        // Parse with SIMD
        let mut bytes = json_bytes;
        let parsed: serde_json::Value = simd_json::from_slice(&mut bytes).map_err(|e| {
            OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("SIMD JSON error: {}", e)
            )))
        })?;
        
        // Convert back to LazyResponse using standard serde
        // Note: In a real implementation, you'd want to implement SIMD support directly
        // in the LazyResponse deserialization
        let json_str = simd_json::to_string(&parsed).map_err(|e| {
            OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("SIMD JSON serialize error: {}", e)
            )))
        })?;
        
        serde_json::from_str(&json_str).map_err(OpenAIError::Json)
    }

    /// Parse JSON array with SIMD optimization
    pub fn parse_response_list(mut json_bytes: Vec<u8>) -> Result<crate::types::ResponseList> {
        simd_json::from_slice(&mut json_bytes).map_err(|e| {
            OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("SIMD JSON error: {}", e)
            )))
        })
    }

    /// Check if the input is valid JSON without full parsing
    pub fn is_valid_json(mut json_bytes: Vec<u8>) -> bool {
        simd_json::from_slice::<serde_json::Value>(&mut json_bytes).is_ok()
    }

    /// Fast JSON minification using SIMD
    pub fn minify(mut json_bytes: Vec<u8>) -> Result<Vec<u8>> {
        let parsed: serde_json::Value = simd_json::from_slice(&mut json_bytes).map_err(|e| {
            OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("SIMD JSON error: {}", e)
            )))
        })?;
        
        simd_json::to_vec(&parsed).map_err(|e| {
            OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("SIMD JSON serialize error: {}", e)
            )))
        })
    }

    /// Pretty print JSON using SIMD parsing
    pub fn pretty_print(mut json_bytes: Vec<u8>) -> Result<String> {
        let parsed: serde_json::Value = simd_json::from_slice(&mut json_bytes).map_err(|e| {
            OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("SIMD JSON error: {}", e)
            )))
        })?;
        
        serde_json::to_string_pretty(&parsed).map_err(OpenAIError::Json)
    }

    /// Benchmark comparison between SIMD and standard JSON parsing
    #[cfg(test)]
    pub fn benchmark_parsing(json_data: &[u8], iterations: usize) -> (std::time::Duration, std::time::Duration) {
        use std::time::Instant;
        
        // Standard serde_json benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let _: serde_json::Value = serde_json::from_slice(json_data).unwrap();
        }
        let standard_duration = start.elapsed();
        
        // SIMD benchmark
        let start = Instant::now();
        for _ in 0..iterations {
            let mut data = json_data.to_vec();
            let _: serde_json::Value = simd_json::from_slice(&mut data).unwrap();
        }
        let simd_duration = start.elapsed();
        
        (standard_duration, simd_duration)
    }
}

/// Extension trait for SIMD JSON operations
#[cfg(feature = "simd")]
pub trait SimdJsonExt {
    /// Parse using SIMD optimizations
    fn parse_simd(data: Vec<u8>) -> Result<Self>
    where
        Self: Sized;
}

#[cfg(feature = "simd")]
impl SimdJsonExt for Response {
    fn parse_simd(data: Vec<u8>) -> Result<Self> {
        SimdJsonParser::parse_response(data)
    }
}

#[cfg(feature = "simd")]
impl SimdJsonExt for crate::types::ResponseList {
    fn parse_simd(data: Vec<u8>) -> Result<Self> {
        SimdJsonParser::parse_response_list(data)
    }
}

/// Auto-detection of best JSON parser based on data size
pub struct AdaptiveJsonParser;

impl AdaptiveJsonParser {
    /// Automatically choose between standard and SIMD parsing based on data size
    pub fn parse_response(json_bytes: Vec<u8>) -> Result<Response> {
        #[cfg(feature = "simd")]
        {
            // Use SIMD for larger payloads where the overhead is worth it
            if json_bytes.len() > 1024 {
                SimdJsonParser::parse_response(json_bytes)
            } else {
                serde_json::from_slice(&json_bytes).map_err(OpenAIError::Json)
            }
        }
        
        #[cfg(not(feature = "simd"))]
        {
            serde_json::from_slice(&json_bytes).map_err(OpenAIError::Json)
        }
    }

    /// Adaptive parsing for lazy responses
    pub fn parse_lazy_response(json_bytes: Vec<u8>) -> Result<LazyResponse<'static>> {
        #[cfg(feature = "simd")]
        {
            if json_bytes.len() > 2048 {
                SimdJsonParser::parse_lazy_response(json_bytes)
            } else {
                let json_str = String::from_utf8(json_bytes).map_err(|e| {
                    OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("UTF-8 error: {}", e)
            )))
                })?;
                serde_json::from_str(&json_str).map_err(OpenAIError::Json)
            }
        }
        
        #[cfg(not(feature = "simd"))]
        {
            let json_str = String::from_utf8(json_bytes).map_err(|e| {
                OpenAIError::Json(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("UTF-8 error: {}", e)
            )))
            })?;
            serde_json::from_str(&json_str).map_err(OpenAIError::Json)
        }
    }
}

/// Feature detection for JSON parsing capabilities
pub struct JsonCapabilities;

impl JsonCapabilities {
    /// Check if SIMD JSON parsing is available
    pub const fn has_simd() -> bool {
        cfg!(feature = "simd")
    }

    /// Get the best available JSON parser name
    pub fn best_parser() -> &'static str {
        if Self::has_simd() {
            "simd-json"
        } else {
            "serde_json"
        }
    }

    /// Get recommended data size threshold for SIMD parsing
    pub const fn simd_threshold() -> usize {
        1024 // bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adaptive_parser() {
        let small_json = br#"{"id": "test", "status": "completed"}"#;
        let response = AdaptiveJsonParser::parse_response(small_json.to_vec());
        assert!(response.is_ok());
    }

    #[test]
    fn test_json_capabilities() {
        println!("Best JSON parser: {}", JsonCapabilities::best_parser());
        println!("Has SIMD: {}", JsonCapabilities::has_simd());
        println!("SIMD threshold: {} bytes", JsonCapabilities::simd_threshold());
    }

    #[cfg(feature = "simd")]
    #[test]
    fn test_simd_parser() {
        let json_data = br#"{
            "id": "resp_123",
            "object": "response",
            "status": "completed",
            "output": [{"type": "message", "content": [{"type": "text", "text": "Hello World"}]}],
            "usage": {"total_tokens": 10}
        }"#;

        let response = SimdJsonParser::parse_response(json_data.to_vec()).unwrap();
        assert_eq!(response.id, "resp_123");
        assert!(response.is_completed());
    }

    #[cfg(feature = "simd")]
    #[test]
    fn test_simd_validation() {
        let valid_json = br#"{"valid": true}"#;
        let invalid_json = br#"{"invalid": true"#; // Missing closing brace

        assert!(SimdJsonParser::is_valid_json(valid_json.to_vec()));
        assert!(!SimdJsonParser::is_valid_json(invalid_json.to_vec()));
    }

    #[cfg(feature = "simd")]
    #[test]
    fn test_simd_minify() {
        let pretty_json = br#"{
            "id": "test",
            "nested": {
                "value": 123
            }
        }"#;

        let minified = SimdJsonParser::minify(pretty_json.to_vec()).unwrap();
        let minified_str = String::from_utf8(minified).unwrap();
        
        // Should not contain extra whitespace
        assert!(!minified_str.contains('\n'));
        assert!(!minified_str.contains("    ")); // 4 spaces
        assert!(minified_str.contains("\"id\":\"test\""));
    }

    #[cfg(feature = "simd")]
    #[test]
    fn test_simd_benchmark() {
        let json_data = br#"{
            "id": "resp_123",
            "object": "response", 
            "status": "completed",
            "output": [{"type": "message", "content": [{"type": "text", "text": "Hello World"}]}],
            "usage": {"total_tokens": 10, "prompt_tokens": 5, "completion_tokens": 5}
        }"#;

        let (standard_time, simd_time) = SimdJsonParser::benchmark_parsing(json_data, 1000);
        
        println!("Standard JSON parsing: {:?}", standard_time);
        println!("SIMD JSON parsing: {:?}", simd_time);
        println!("SIMD speedup: {:.2}x", standard_time.as_nanos() as f64 / simd_time.as_nanos() as f64);
        
        // SIMD should be at least as fast (though for small payloads it might be slower due to overhead)
        // This is more of an informational test
    }

    #[test]
    fn test_large_payload_adaptive_parsing() {
        // Create a large JSON payload to test adaptive parsing
        let large_json = format!(r#"{{
            "id": "resp_large",
            "object": "response",
            "status": "completed",
            "output": [{{
                "type": "message",
                "content": [{{
                    "type": "text",
                    "text": "{}"
                }}]
            }}],
            "usage": {{"total_tokens": 1000}}
        }}"#, "A".repeat(2048)); // Large text content

        let response = AdaptiveJsonParser::parse_response(large_json.into_bytes());
        assert!(response.is_ok());
        
        let response = response.unwrap();
        assert_eq!(response.id, "resp_large");
        assert!(response.is_completed());
    }
}