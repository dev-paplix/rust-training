//! # Feature Flags Demo
//!
//! A demonstration library for using feature flags and conditional compilation
//! when publishing Rust crates to crates.io.
//!
//! # Features
//!
//! - `std` (enabled by default): Enable standard library support
//! - `serde_support`: Enable serialization/deserialization with serde
//! - `async_support`: Enable async runtime support with tokio
//! - `crypto`: Enable cryptographic functions (hashing, encoding)
//! - `full`: Enable all optional features
//!
//! # Examples
//!
//! ## Basic Usage (no features)
//!
//! ```
//! use feature_flags_demo::Data;
//!
//! let data = Data::new("Hello");
//! assert_eq!(data.content(), "Hello");
//! ```
//!
//! ## With Serde Support
//!
//! ```toml
//! [dependencies]
//! feature_flags_demo = { version = "0.1", features = ["serde_support"] }
//! ```
//!
//! ```ignore
//! use feature_flags_demo::Data;
//!
//! let data = Data::new("Hello");
//! let json = serde_json::to_string(&data).unwrap();
//! ```
//!
//! ## With All Features
//!
//! ```toml
//! [dependencies]
//! feature_flags_demo = { version = "0.1", features = ["full"] }
//! ```

use thiserror::Error;

/// Error type for this library.
#[derive(Debug, Error)]
pub enum FeatureError {
    /// Feature not enabled error
    #[error("Feature '{0}' is not enabled")]
    FeatureNotEnabled(String),
    
    /// General error
    #[error("Error: {0}")]
    General(String),
}

/// A data container that demonstrates feature-gated functionality.
///
/// # Examples
///
/// ```
/// use feature_flags_demo::Data;
///
/// let data = Data::new("content");
/// assert_eq!(data.content(), "content");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_support", derive(serde::Serialize, serde::Deserialize))]
pub struct Data {
    content: String,
    #[cfg(feature = "std")]
    timestamp: Option<String>,
}

impl Data {
    /// Creates a new `Data` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use feature_flags_demo::Data;
    ///
    /// let data = Data::new("Hello, World!");
    /// assert_eq!(data.content(), "Hello, World!");
    /// ```
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            #[cfg(feature = "std")]
            timestamp: None,
        }
    }

    /// Returns the content.
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Sets a timestamp (only available with `std` feature).
    ///
    /// # Examples
    ///
    /// ```
    /// use feature_flags_demo::Data;
    ///
    /// let mut data = Data::new("content");
    /// data.set_timestamp("2025-01-01");
    /// assert_eq!(data.timestamp(), Some("2025-01-01"));
    /// ```
    #[cfg(feature = "std")]
    pub fn set_timestamp(&mut self, timestamp: impl Into<String>) {
        self.timestamp = Some(timestamp.into());
    }

    /// Gets the timestamp (only available with `std` feature).
    #[cfg(feature = "std")]
    pub fn timestamp(&self) -> Option<&str> {
        self.timestamp.as_deref()
    }
}

// Serde-specific functionality
#[cfg(feature = "serde_support")]
pub mod serde_utils {
    //! Serialization utilities (requires `serde_support` feature).
    
    use super::*;
    
    /// Serializes data to JSON string.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "serde_support")]
    /// # {
    /// use feature_flags_demo::{Data, serde_utils};
    ///
    /// let data = Data::new("test");
    /// let json = serde_utils::to_json(&data).unwrap();
    /// assert!(json.contains("test"));
    /// # }
    /// ```
    pub fn to_json<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
        serde_json::to_string(value)
    }
    
    /// Deserializes data from JSON string.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "serde_support")]
    /// # {
    /// use feature_flags_demo::{Data, serde_utils};
    ///
    /// let json = r#"{"content":"test","timestamp":null}"#;
    /// let data: Data = serde_utils::from_json(json).unwrap();
    /// assert_eq!(data.content(), "test");
    /// # }
    /// ```
    pub fn from_json<'a, T: serde::Deserialize<'a>>(s: &'a str) -> Result<T, serde_json::Error> {
        serde_json::from_str(s)
    }
}

// Async-specific functionality
#[cfg(feature = "async_support")]
pub mod async_utils {
    //! Async utilities (requires `async_support` feature).
    
    /// Processes data asynchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "async_support")]
    /// # {
    /// use feature_flags_demo::async_utils;
    ///
    /// # tokio_test::block_on(async {
    /// let result = async_utils::process_async("data").await;
    /// assert_eq!(result, "Processed: data");
    /// # });
    /// # }
    /// ```
    pub async fn process_async(data: &str) -> String {
        // Simulate async work
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        format!("Processed: {}", data)
    }
    
    /// Fetches data asynchronously.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "async_support")]
    /// # {
    /// use feature_flags_demo::async_utils;
    ///
    /// # tokio_test::block_on(async {
    /// let data = async_utils::fetch_data().await;
    /// assert!(!data.is_empty());
    /// # });
    /// # }
    /// ```
    pub async fn fetch_data() -> String {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        String::from("Fetched data")
    }
}

// Crypto-specific functionality
#[cfg(feature = "crypto")]
pub mod crypto_utils {
    //! Cryptographic utilities (requires `crypto` feature).
    
    use sha2::{Sha256, Digest};
    use base64::{Engine as _, engine::general_purpose};
    
    /// Computes SHA-256 hash of input data.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "crypto")]
    /// # {
    /// use feature_flags_demo::crypto_utils;
    ///
    /// let hash = crypto_utils::hash("hello");
    /// assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex chars
    /// # }
    /// ```
    pub fn hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
    
    /// Encodes data to base64.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "crypto")]
    /// # {
    /// use feature_flags_demo::crypto_utils;
    ///
    /// let encoded = crypto_utils::encode_base64("hello");
    /// assert_eq!(encoded, "aGVsbG8=");
    /// # }
    /// ```
    pub fn encode_base64(data: &str) -> String {
        general_purpose::STANDARD.encode(data.as_bytes())
    }
    
    /// Decodes base64 data.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "crypto")]
    /// # {
    /// use feature_flags_demo::crypto_utils;
    ///
    /// let decoded = crypto_utils::decode_base64("aGVsbG8=").unwrap();
    /// assert_eq!(decoded, "hello");
    /// # }
    /// ```
    pub fn decode_base64(data: &str) -> Result<String, base64::DecodeError> {
        let bytes = general_purpose::STANDARD.decode(data)?;
        Ok(String::from_utf8_lossy(&bytes).to_string())
    }
}

/// Utility function always available.
///
/// # Examples
///
/// ```
/// use feature_flags_demo::always_available;
///
/// assert!(always_available());
/// ```
pub fn always_available() -> bool {
    true
}

/// Checks which features are enabled.
///
/// # Examples
///
/// ```
/// use feature_flags_demo::check_features;
///
/// let features = check_features();
/// assert!(features.contains("std"));
/// ```
pub fn check_features() -> String {
    let mut features: Vec<&str> = Vec::new();
    
    #[cfg(feature = "std")]
    features.push("std");
    
    #[cfg(feature = "serde_support")]
    features.push("serde_support");
    
    #[cfg(feature = "async_support")]
    features.push("async_support");
    
    #[cfg(feature = "crypto")]
    features.push("crypto");
    
    if features.is_empty() {
        "No features enabled (no_std mode)".to_string()
    } else {
        format!("Enabled features: {}", features.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_creation() {
        let data = Data::new("test content");
        assert_eq!(data.content(), "test content");
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_timestamp() {
        let mut data = Data::new("test");
        data.set_timestamp("2025-01-01");
        assert_eq!(data.timestamp(), Some("2025-01-01"));
    }

    #[test]
    fn test_always_available() {
        assert!(always_available());
    }

    #[test]
    fn test_check_features() {
        let features = check_features();
        assert!(!features.is_empty());
    }

    #[test]
    #[cfg(feature = "serde_support")]
    fn test_serde_serialization() {
        let data = Data::new("test");
        let json = serde_utils::to_json(&data).unwrap();
        assert!(json.contains("test"));
        
        let deserialized: Data = serde_utils::from_json(&json).unwrap();
        assert_eq!(deserialized.content(), "test");
    }

    #[test]
    #[cfg(feature = "crypto")]
    fn test_crypto_hash() {
        let hash = crypto_utils::hash("hello");
        assert_eq!(hash.len(), 64);
        // SHA-256 of "hello"
        assert_eq!(hash, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    #[cfg(feature = "crypto")]
    fn test_base64() {
        let encoded = crypto_utils::encode_base64("hello");
        assert_eq!(encoded, "aGVsbG8=");
        
        let decoded = crypto_utils::decode_base64(&encoded).unwrap();
        assert_eq!(decoded, "hello");
    }
}
