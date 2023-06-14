use std::{borrow::Cow, fs::{read, canonicalize}, path::PathBuf};

use wry::{http::{Request, Response, header::CONTENT_TYPE}, Result};

pub(crate) fn server(req: &Request<Vec<u8>>) -> Result<Response<Cow<'static, [u8]>>> {
    let path = req.uri().path();

    // `1..` for removing leading slash
    let content = read(canonicalize(PathBuf::from("html").join(&path[1..]))?)?.into();


    // Return asset contents and mime types based on file extentions
    // If you don't want to do this manually, there are some crates for you.
    // Such as `infer` and `mime_guess`.
    let mimetype = if path.ends_with(".html") || path == "/" {
        "text/html"
    } else if path.ends_with(".js") {
        "text/javascript"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else {
        unimplemented!();
    };

    Response::builder()
        .header(CONTENT_TYPE, mimetype)
        .body(content)
        .map_err(Into::into)
}
