use std::task::{Context, Poll};

use axum::{
    body::{Body, BoxBody},
    http::{Request, Response, StatusCode},
    response::IntoResponse,
};
use futures::future::BoxFuture;
use tower::Service;

#[derive(Clone)]
pub struct Authorization<S> {
    pub(crate) inner: S,
}

impl<S> Service<Request<Body>> for Authorization<S>
where
    S: Service<Request<Body>, Response = Response<BoxBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            match req.headers().get("Authorization") {
                None => Ok((StatusCode::UNAUTHORIZED, "Empty token").into_response()),
                Some(ref token) => {
                    if token.to_str().unwrap() != dotenv::var("TOKEN").unwrap() {
                        Ok((StatusCode::FORBIDDEN, "Bad token").into_response())
                    } else {
                        Ok(inner.call(req).await?)
                    }
                }
            }
        })
    }
}

pub async fn get_login() -> impl IntoResponse {}
