use axum::http::{Request, Response, StatusCode};
use futures::future::{self, Either, Ready};
use tower::{BoxError, Layer, Service};

use crate::core::SharedState;

#[derive(Debug, Clone)]
pub struct OpenCheckService<T> {
    inner: T,
}

impl<S, ResBody, ReqBody> Service<Request<ReqBody>> for OpenCheckService<S>
where
    ResBody: Default,
    S: Service<Request<ReqBody>, Response = Response<ResBody>>,
    S::Error: Into<BoxError>,
{
    type Response = Response<ResBody>;

    type Error = S::Error;

    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let shared_state = req.extensions().get::<SharedState>().cloned();

        if let Some(state) = shared_state {
            let state = state.lock().unwrap();

            if !state.open {
                let mut res = Response::default();

                *res.status_mut() = StatusCode::FORBIDDEN;

                Either::Right(future::ok(res))
            } else {
                Either::Left(self.inner.call(req))
            }
        } else {
            let mut res = Response::default();

            *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            Either::Right(future::ok(res))
        }
    }
}

pub struct OpenCheckLayer;

impl<S> Layer<S> for OpenCheckLayer {
    type Service = OpenCheckService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        Self::Service { inner }
    }
}
