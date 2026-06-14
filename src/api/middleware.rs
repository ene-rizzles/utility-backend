use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tracing::warn;

#[allow(dead_code)]
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
                .compare_exchange(
                    current,
                    current - count,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return true;
            }
        }
    }
}

pub async fn rate_limit_layer(req: Request<axum::body::Body>, next: Next) -> Response {
    let bucket = req.extensions().get::<Arc<TokenBucket>>();
    match bucket {
        Some(b) if !b.try_consume(1) => {
            warn!("rate limit exceeded for request");
            Response::builder()
                .status(StatusCode::TOO_MANY_REQUESTS)
                .body(Body::from("rate limit exceeded"))
                .unwrap()
        }
        _ => next.run(req).await,
    }
}
