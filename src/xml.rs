use axum::http::{header, Response, StatusCode};
use axum::response::IntoResponse;
use bytes::Bytes;
use http_body::{Body, Frame};
use std::fmt::Display;
use std::{
    convert::Infallible,
    pin::Pin,
    task::{Context, Poll},
};

pub struct Xml<T>(pub T);

impl<T: Display> Body for Xml<T> {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let body = self.as_ref().get_ref().0.to_string();
        let bytes = Bytes::from(body);
        let frame = Frame::data(bytes);
        Poll::Ready(Some(Ok(frame)))
    }
}

impl<T: Display> IntoResponse for Xml<T> {
    fn into_response(self) -> Response<axum::body::Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/xml")
            .body(axum::body::Body::from(self.0.to_string()))
            .unwrap()
    }
}
