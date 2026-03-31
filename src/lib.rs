#![deny(clippy::all)]

use anyhow::{anyhow, Result};

use csv::Reader;
use napi_derive::napi;

#[napi]
pub enum CsvValue {
  String(String),
  Integer(u32),
  Float(f64),
  Boolean(bool),
}

#[napi]
impl CsvValue {
  #[napi]
  pub fn new_from(s: String) -> Self {
    let ures = s.parse::<u32>();
    if ures.is_ok() {
      return Self::Integer(ures.unwrap());
    }
    let fres = s.parse::<f64>();
    if fres.is_ok() {
      return Self::Float(fres.unwrap());
    }
    if s.to_lowercase() == "false" {
      return Self::Boolean(false);
    }
    if s.to_lowercase() == "true" {
      return Self::Boolean(true);
    }

    Self::String(s)
  }
}

#[napi]
pub fn plus_100(x: u32, y: u32) -> bool {
  x > y
}

#[napi]
pub fn read_csv(path: String) -> Result<()> {
  let mut reader = Reader::from_path(&path)?;
  for result in reader.records() {
    let res = result?;
  }

  Ok(())
}
