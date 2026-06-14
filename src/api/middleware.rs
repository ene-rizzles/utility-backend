use axum::{http::Request, middleware::Next, response::Response};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::warn;

pub struct TokenBucket {
    tokens: AtomicU64,
    max_tokens: u64,
    refill_rate: u64,
}

impl TokenBucket {
    pub fn new(max_tokens: u64, refill_rate: u64) -> Self {
        Self {
            tokens: AtomicU64::new(max_tokens),
            max_tokens,
            refill_rate,
        }
    }

    pub fn try_consume(&self, count: u64) -> bool {
        loop {
            let current = self.tokens.load(Ordering::Acquire);
            if current < count {
                return false;
            }
            if self
                .tokens
                .compare_exchange(current, current - count, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                return true;
            }
        }
    }
}

pub async fn rate_limit_layer<B>(
    req: Request<B>,
    next: Next<B>,
) -> Response {
    let bucket = req.extensions().get::<Arc<TokenBucket>>();
    match bucket {
        Some(b) if !b.try_consume(1) => {
            warn!("rate limit exceeded for request");
            http::Response::builder()
                .status(429)
                .body(axum::body::Body::from("rate limit exceeded"))
                .unwrap()
        }
        _ => next.run(req).await,
    }
}
