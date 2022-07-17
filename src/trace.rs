use std::{
    error::Error,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{http::Request, response::Response};
use pin_project::pin_project;
use tokio::time::Instant;
use tower::{Layer, Service};
use tracing::{error, error_span, info, Span};
use uuid::Uuid;

pub struct TraceLayer;

impl<S> Layer<S> for TraceLayer {
    type Service = Trace<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Trace { inner }
    }
}

#[derive(Clone)]
pub struct Trace<S> {
    inner: S,
}

impl<ReqB, ResB, Serv, Err, Fut> Service<Request<ReqB>> for Trace<Serv>
where
    Serv: Service<Request<ReqB>, Future = Fut, Error = Err>,
    Fut: Future<Output = Result<Response<ResB>, Err>>,
    Err: Error,
{
    type Response = Response<ResB>;
    type Error = Err;
    type Future = TraceFuture<Fut>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqB>) -> Self::Future {
        let start = Instant::now();
        let id = Uuid::new_v4();
        let method = req.method();
        let uri = req.uri();
        let span = error_span!("request", %id, %method, %uri);

        let inner = {
            let _guard = span.enter();
            info!("started processing request");
            self.inner.call(req)
        };

        TraceFuture { inner, start, span }
    }
}

#[pin_project]
pub struct TraceFuture<F> {
    #[pin]
    inner: F,
    start: Instant,
    span: Span,
}

impl<F, B, E> Future for TraceFuture<F>
where
    F: Future<Output = Result<Response<B>, E>>,
    E: Error,
{
    type Output = Result<Response<B>, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _guard = this.span.enter();
        let result = if let Poll::Ready(r) = this.inner.poll(cx) {
            r
        } else {
            return Poll::Pending;
        };

        match &result {
            Ok(res) => {
                let is_error = res.status().is_server_error();
                let elapsed = this.start.elapsed().as_micros();
                if is_error {
                    error!(
                        status_code = %res.status(),
                        latency = format!("{:?}μs elapsed", elapsed),
                        "response failed"
                    );
                } else {
                    info!(
                        status_code = %res.status(),
                        latency = format!("{:?}μs elapsed", elapsed),
                        "finished processing request"
                    );
                }
            }
            Err(error) => {
                let elapsed = this.start.elapsed().as_micros();
                error!(
                    %error,
                    latency = format!("{:?}μs elapsed", elapsed),
                    "response failed"
                );
            }
        }

        Poll::Ready(result)
    }
}
