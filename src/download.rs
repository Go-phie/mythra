use cursive::utils::Counter;
use reqwest::Url;

use std::fs;
use std::io::{self, copy, Read};
use std::path::Path;

fn append_frag(text: &mut String, frag: &mut String) {
    if !frag.is_empty() {
        let encoded = frag
            .chars()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|ch| u8::from_str_radix(&ch.iter().collect::<String>(), 16).unwrap())
            .collect::<Vec<u8>>();
        text.push_str(&std::str::from_utf8(&encoded).unwrap());
        frag.clear();
    }
}

fn decode_uri(text: &str) -> String {
    let mut output = String::new();
    let mut encoded_ch = String::new();
    let mut iter = text.chars();
    while let Some(ch) = iter.next() {
        if ch == '%' {
            encoded_ch.push_str(&format!("{}{}", iter.next().unwrap(), iter.next().unwrap()));
        } else {
            append_frag(&mut output, &mut encoded_ch);
            output.push(ch);
        }
    }
    append_frag(&mut output, &mut encoded_ch);
    output
}

pub struct DownloadProgress<R> {
    inner: R,
    progress_bar: Counter,
}

impl<R: Read> Read for DownloadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf).map(|n| {
            &self.progress_bar.tick(n as usize);
            n
        })
    }
}

pub fn download_size(url: &str) -> Result<u64, String> {
    let url = Url::parse(url).unwrap();
    let resp = ureq::head(url.as_str()).call();
    let total_size = {
        if resp.ok() {
            resp.header("Content-Length")
                .unwrap()
                .parse::<u64>()
                .unwrap()
        } else {
            return Err(
                format!("Couldn't download URL: {}. Error: {:?}", url, resp.status(),).into(),
            );
        }
    };
    Ok(total_size)
}

pub fn download_from_url(counter: Counter, url: String) {
    let parsed_url = Url::parse(&url[..]).unwrap();
    let mut request = ureq::get(url.as_str());

    let segment = parsed_url
        .path_segments()
        .and_then(|segments| {
            let output = decode_uri(&segments.last().unwrap().to_owned());
            Some(output)
        })
        .unwrap_or("tmp.bin".to_owned());

    let file = Path::new(&segment);

    // if file already exists
    if file.exists() {
        let size = file.metadata().unwrap().len() - 1;
        request = request
            .set("Content-Length", &(format!("bytes={}-", size))[..])
            .build();
        &counter.set(size as usize);
    }

    let mut source = DownloadProgress {
        progress_bar: counter,
        inner: request.call().into_reader(),
    };
    let mut dest = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file)
        .unwrap();

    copy(&mut source, &mut dest).unwrap();
}
