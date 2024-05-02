use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
    sync::Arc,
};

use anyhow::{Context, Result};
use crossbeam::sync::WaitGroup;
use indicatif::{ProgressBar, ProgressStyle};

use crate::Downloader;

pub struct HttpDownloadState {
    user_agent: Arc<String>,
    threads: usize,
}

impl Downloader<HttpDownloadState> {
    pub fn http_client(src: &str, user_agent: &str, threads: usize) -> Self {
        let opt = HttpDownloadState {
            user_agent: Arc::new(user_agent.into()),
            threads,
        };
        Self {
            src: Arc::new(src.into()),
            opt,
        }
    }

    async fn get_content_length(&self) -> Result<u64> {
        let res = reqwest::ClientBuilder::new()
            .user_agent(&*self.opt.user_agent)
            .build()?
            .head(&*self.src)
            .send()
            .await?;
        let content_length = res
            .headers()
            .get("content-length")
            .unwrap()
            .to_str()?
            .parse::<u64>()?;
        Ok(content_length)
    }

    fn range_of_bytes(&self, i: usize, content_length: u64) -> String {
        let piece_size = content_length / self.opt.threads as u64;
        if i == self.opt.threads - 1 {
            format!("bytes={}-", i as u64 * piece_size)
        } else {
            format!(
                "bytes={}-{}",
                i as u64 * piece_size,
                (i as u64 + 1) * piece_size - 1
            )
        }
    }

    fn filename(&self) -> Result<String> {
        let path = PathBuf::from(&*self.src);
        let name = path
            .file_name()
            .context("cannot infer file name from url")?
            .to_string_lossy()
            .to_string();
        Ok(name)
    }

    fn piece_name(&self, i: usize) -> Result<PathBuf> {
        let dir = PathBuf::from(format!("{}.cache", self.filename()?));
        let name = dir.join(format!("{}-cache-{}", self.filename()?, i));
        Ok(name)
    }

    fn create_progress_bar(&self, total: u64) -> ProgressBar {
        let pb = ProgressBar::new(total);
        pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap());
        pb
    }

    async fn download_pieces(&self) -> Result<Vec<PathBuf>> {
        let content_length = self.get_content_length().await?;
        let wg = WaitGroup::new();
        let mut pieces = Vec::with_capacity(self.opt.threads);
        let pb = self.create_progress_bar(content_length);

        let dir = PathBuf::from(format!("{}.cache", self.filename()?));
        create_dir_all(dir)?;

        (0..self.opt.threads).try_for_each(|i| {
            let range = self.range_of_bytes(i, content_length);
            let url = self.src.clone();
            let user_agent = self.opt.user_agent.clone();
            let name = self.piece_name(i)?;
            pieces.push(name.clone());
            let mut file = File::create(name)?;
            let pb = pb.clone();
            let wg = wg.clone();
            tokio::spawn(async move {
                let mut res = reqwest::ClientBuilder::new()
                    .user_agent(&*user_agent)
                    .build()?
                    .get(&*url)
                    .header("Range", &range)
                    .send()
                    .await?;
                while let Ok(Some(chunk)) = res.chunk().await {
                    pb.inc(chunk.len() as _);
                    file.write_all(&chunk)?;
                }
                drop(wg);
                Ok::<(), anyhow::Error>(())
            });
            Ok::<(), anyhow::Error>(())
        })?;
        wg.wait();
        pb.finish();
        Ok(pieces)
    }

    fn concat_pieces(&self, pieces: Vec<PathBuf>) -> Result<()> {
        let mut file = File::create(self.filename()?)?;
        pieces.into_iter().try_for_each(|name| {
            let mut piece = File::open(name)?;
            std::io::copy(&mut piece, &mut file)?;
            anyhow::Ok(())
        })?;

        let dir = PathBuf::from(format!("{}.cache", self.filename()?));
        std::fs::remove_dir_all(dir)?;
        Ok(())
    }

    pub async fn download(&self) -> Result<()> {
        let pieces = self.download_pieces().await?;
        self.concat_pieces(pieces)?;
        Ok(())
    }
}
