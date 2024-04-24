use std::sync::Arc;

use anyhow::Result;

use crate::Downloader;

pub struct BitTorrentDownloadState;

impl Downloader<BitTorrentDownloadState> {
    pub fn bittorrent_client(url: &str) -> Self {
        let opt = BitTorrentDownloadState {};
        Self {
            url: Arc::new(url.into()),
            opt,
        }
    }

    pub async fn download(&self) -> Result<()> {
        let client = rbittorrent::TorrentClientBuilder::new()
            .add_path(&*self.url)?
            .build();
        client.send_request().await?;
        Ok(())
    }
}
