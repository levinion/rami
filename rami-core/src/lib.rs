use std::sync::Arc;

mod bittorrent;
mod http;

pub struct Downloader<T> {
    src: Arc<String>,
    opt: T,
}
