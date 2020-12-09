use cursive::utils::Counter;
use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
use reqwest::StatusCode;
use std::fs::OpenOptions;
use std::borrow::Cow;
//use std::io::Read;
use std::str::FromStr;

pub static MAX_DOWNLOAD:u64  = 1000000000;

fn basename<'a>(path: &'a String, sep: char) -> Cow<'a, str> {
    let mut pieces = path.rsplit(sep);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

struct PartialRangeIter {
  start: u64,
  end: u64,
  buffer_size: u32,
}

impl PartialRangeIter {
  pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self, &'static str> {
    if buffer_size == 0 {
      Err("invalid buffer_size, give a value greater than zero.")?;
    }
    Ok(PartialRangeIter {
      start,
      end,
      buffer_size,
    })
  }
}

impl Iterator for PartialRangeIter {
  type Item = HeaderValue;
  fn next(&mut self) -> Option<Self::Item> {
    if self.start > self.end {
      None
    } else {
      let prev_start = self.start;
      self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
      Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).expect("string provided by format!"))
    }
  }
}

pub fn download_from_url(counter: &Counter, url:String){
  const CHUNK_SIZE: u32 = 10240;
    
  let client = reqwest::blocking::Client::new();
  let response = client.head(&url).send().expect("failed to get head");
  let length = response
    .headers()
    .get(CONTENT_LENGTH)
    .ok_or("response doesn't include the content length")
    .unwrap();
  let mut length = u64::from_str(length.to_str().unwrap()).map_err(|_| "invalid Content-Length header").unwrap();
  length = length/100;
  let mut output_file = OpenOptions::new().append(true).create(true).open(basename(&url, '/').to_string()).unwrap();
  let increment:u64;
  if length > CHUNK_SIZE as u64 {
      increment = MAX_DOWNLOAD/(length/CHUNK_SIZE as u64);
  } else {
      increment = MAX_DOWNLOAD/length;
  }
  let range_iter = PartialRangeIter::new(0, length -1, CHUNK_SIZE).unwrap();
  for range in range_iter{
    let mut response = client.get(&url)
        .header(RANGE, range).send().unwrap();
    let status = response.status();
    if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
      panic!("Unexpected server response: {}", status)
    }
  // TODO: Fix blocking nature of copy
//    let out = std::io::copy(&mut response.take(CHUNK_SIZE as u64), &mut output_file)
//        .unwrap();
//    debug!("{:?}", out);
    std::io::copy(&mut response, &mut output_file).unwrap();
    counter.tick(increment as usize);
  }
    
  let content = response.text().unwrap();
  std::io::copy(&mut content.as_bytes(), &mut output_file)
      .unwrap();

  counter.set(MAX_DOWNLOAD as usize);
}
