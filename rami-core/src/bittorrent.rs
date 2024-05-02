use std::sync::Arc;

use anyhow::Result;

use crate::Downloader;

enum SourceTyp {
    Url,
    Path,
}

pub struct BitTorrentDownloadState {
    source_typ: SourceTyp,
}

impl Downloader<BitTorrentDownloadState> {
    pub fn bittorrent_client(src: &str) -> Self {
        let source_typ = if src.starts_with("http") {
            SourceTyp::Url
        } else {
            SourceTyp::Path
        };
        let opt = BitTorrentDownloadState { source_typ };
        Self {
            src: Arc::new(src.into()),
            opt,
        }
    }

    pub async fn download(&self) -> Result<()> {
        let client = match self.opt.source_typ {
            SourceTyp::Path => rbittorrent::TorrentClientBuilder::new()
                .add_torrent_path(&*self.src)?
                .build(),
            SourceTyp::Url => rbittorrent::TorrentClientBuilder::new()
                .add_torrent_url(&self.src)
                .await?
                .build(),
        };
        client.send_request().await?;
        Ok(())
    }
}
