# Rami

Rami is a multi-threaded downloader written in Rust.

## Usage

```sh
rami [OPTIONS] <URL>
```

## Arguments

- `<URL>`: The URL from which to download the file.

## Options

- `-t, --threads <THREADS>`: Number of threads to use for downloading. Default is 0 (Auto).
- `-a, --agent <AGENT>`: User agent string to use for the download request. Default is "rami/0.1.0".
- `-h, --help`: Print help information.
