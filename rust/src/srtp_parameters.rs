//! Collection of SRTP-related data structures that are used to specify SRTP encryption/decryption
//! parameters.

use serde::{Deserialize, Serialize};

/// SRTP parameters.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SrtpParameters {
    /// Encryption and authentication transforms to be used.
    pub crypto_suite: SrtpCryptoSuite,
    /// SRTP keying material (master key and salt) in Base64.
    pub key_base64: String,
}

/// SRTP crypto suite.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum SrtpCryptoSuite {
    /// AEAD_AES_256_GCM
    #[serde(rename = "AEAD_AES_256_GCM")]
    AeadAes256Gcm,
}

impl Default for SrtpCryptoSuite {
    fn default() -> Self {
        Self::AesCm128HmacSha180
    }
}
