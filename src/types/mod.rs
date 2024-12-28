pub mod fixed_window;
pub mod leaky_bucket;
pub mod sliding_window;
pub mod token_bucket;

use crate::backend::BackendError;
use fixed_window::FixedWindowInstance;
use leaky_bucket::LeakyBucketInstance;
use serde::{Deserialize, Serialize};
use sliding_window::SlidingWindowInstance;
use std::{
    error::Error,
    fmt::{self, Debug, Display},
};
use token_bucket::TokenBucketInstance;

pub trait LimiterType: Clone {
    fn is_ratelimited(&self, value: Option<Vec<u8>>) -> Result<Vec<u8>, RateLimiterError>;
    fn window_instance(&self, value: Vec<u8>) -> Result<LimiterInstance, RateLimiterError>;
}

#[derive(Debug)]
pub enum LimiterInstance {
    FixedWindowInstance(FixedWindowInstance),
    SlidingWindowInstance(SlidingWindowInstance),
    TokenBucketInstance(TokenBucketInstance),
    LeakyBucketInstance(LeakyBucketInstance),
}

impl LimiterInstance {
    pub fn as_fixed_window_instance(self) -> Result<FixedWindowInstance, RateLimiterError> {
        match self {
            Self::FixedWindowInstance(i) => Ok(i),
            _ => Err(RateLimiterError::MalformedValue(None)),
        }
    }

    pub fn as_sliding_window_instance(self) -> Result<SlidingWindowInstance, RateLimiterError> {
        match self {
            Self::SlidingWindowInstance(i) => Ok(i),
            _ => Err(RateLimiterError::MalformedValue(None)),
        }
    }

    pub fn as_token_bucket_instance(self) -> Result<TokenBucketInstance, RateLimiterError> {
        match self {
            Self::TokenBucketInstance(i) => Ok(i),
            _ => Err(RateLimiterError::MalformedValue(None)),
        }
    }

    pub fn as_leaky_bucket_instance(self) -> Result<LeakyBucketInstance, RateLimiterError> {
        match self {
            Self::LeakyBucketInstance(i) => Ok(i),
            _ => Err(RateLimiterError::MalformedValue(None)),
        }
    }
}

trait SerializableInstance: Debug + Serialize + for<'de> Deserialize<'de> {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, RateLimiterError> {
        bincode::deserialize(&bytes).map_err(|e| RateLimiterError::MalformedValue(Some(e)))
    }
    fn to_bytes(self) -> Result<Vec<u8>, RateLimiterError> {
        bincode::serialize(&self).map_err(|e| RateLimiterError::MalformedValue(Some(e)))
    }
}

#[derive(Debug)]
pub enum RateLimiterError {
    MalformedValue(Option<bincode::Error>),
    RateExceeded,
    BackendError(BackendError),
    BackendConflict,
}

impl Display for RateLimiterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RateLimiterError::MalformedValue(Some(e)) => std::fmt::Display::fmt(&e, f),
            RateLimiterError::MalformedValue(None) => write!(f, "malformed value"),
            RateLimiterError::RateExceeded => write!(f, "rate exceeded"),
            RateLimiterError::BackendError(e) => std::fmt::Display::fmt(&e, f),
            RateLimiterError::BackendConflict => write!(f, "backend value conflict"),
        }
    }
}

impl Error for RateLimiterError {}
