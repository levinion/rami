use std::sync::Arc;

mod bittorrent;
mod http;

pub struct Downloader<T> {
    url: Arc<String>,
    opt: T,
}
