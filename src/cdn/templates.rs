use askama::Template;
use axum::body::{Bytes, Full};
use axum::http::{Response, StatusCode};
use axum::response::{Html, IntoResponse};
use std::convert::Infallible;

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;

    fn into_response(self) -> Response<Self::Body> {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "").into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "dir.html")]
pub struct DirTemplate {
    pub entries: Vec<String>,
    pub prefix: String,
    pub suffix: String,
}
