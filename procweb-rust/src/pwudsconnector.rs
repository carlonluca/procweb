use actix_rt::net::UnixStream;
use actix_service::Service;
use actix_tls::connect::{ConnectError, ConnectInfo, Connection};
use awc::http::Uri;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::task::{Context, Poll};

type Fut<R, E> = Pin<Box<dyn Future<Output = Result<R, E>>>>;

#[derive(Clone)]
pub struct UdsConnector(PathBuf);

impl UdsConnector {
    pub fn new(path: impl AsRef<Path>) -> Self {
        UdsConnector(path.as_ref().to_path_buf())
    }
}

impl Service<ConnectInfo<Uri>> for UdsConnector {
    type Response = Connection<Uri, UnixStream>;
    type Error = ConnectError;
    type Future = Fut<Self::Response, Self::Error>;

    fn poll_ready(&self, _ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ConnectInfo<Uri>) -> Self::Future {
        let uri = req.request().clone();
        let path = self.0.clone();
        let fut = async {
            let stream = UnixStream::connect(path).await.map_err(ConnectError::Io)?;
            Ok(Connection::new(uri, stream))
        };
        Box::pin(fut)
    }
}