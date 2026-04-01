#![deny(clippy::all)]

use std::{collections::HashMap, fmt};

use anyhow::{anyhow, Result};

use csv::Reader;
use napi_derive::napi;

#[napi]
pub enum CsvValue {
  String(String),
  Integer(i64),
  Float(f64),
  Boolean(bool),
}

#[napi]
#[derive(Clone)]
pub enum ColumnData {
  String(Vec<String>),
  Integer(Vec<i64>),
  Float(Vec<f64>),
  Boolean(Vec<bool>),
}

#[napi]
#[derive(PartialEq, Debug, Clone)]
pub enum DataType {
  String,
  Float,
  Integer,
  Boolean,
}

impl fmt::Display for DataType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = match self {
      Self::Boolean => "bool".to_string(),
      Self::Float => "float".to_string(),
      Self::Integer => "integer".to_string(),
      Self::String => "string".to_string(),
    };
    write!(f, "{}", s)
  }
}

#[napi]
impl ColumnData {
  #[napi]
  pub fn to_float_array(&self) -> Option<Vec<f64>> {
    match self {
      ColumnData::Float(v) => Some(v.to_owned()),
      _ => None,
    }
  }

  #[napi]
  pub fn to_string_array(&self) -> Option<Vec<String>> {
    match self {
      ColumnData::String(v) => Some(v.to_owned()),
      _ => None,
    }
  }

  #[napi]
  pub fn to_int_array(&self) -> Option<Vec<i64>> {
    match self {
      ColumnData::Integer(v) => Some(v.to_owned()),
      _ => None,
    }
  }

  #[napi]
  pub fn to_bool_array(&self) -> Option<Vec<bool>> {
    match self {
      ColumnData::Boolean(v) => Some(v.to_owned()),
      _ => None,
    }
  }
}

#[napi(js_name = "DataFrame")]
pub struct DataFrame {
  columns: HashMap<String, ColumnData>,
  pub len: u32,
}

#[napi]
impl DataFrame {
  #[napi(constructor)]
  pub fn new(columns: HashMap<String, ColumnData>, len: u32) -> Self {
    return Self { columns, len };
  }

  #[napi]
  pub fn col_dtype(&self, col: String) -> Option<DataType> {
    if let Some(d) = self.columns.get(&col) {
      match d {
        ColumnData::Boolean(_) => return Some(DataType::Boolean),
        ColumnData::Float(_) => return Some(DataType::Float),
        ColumnData::Integer(_) => return Some(DataType::Integer),
        ColumnData::String(_) => return Some(DataType::String),
      }
    }
    None
  }

  #[napi]
  pub fn get(&self, col: String) -> Option<ColumnData> {
    if let Some(opt) = self.columns.get(&col) {
      return Some(opt.clone());
    }
    None
  }

  #[napi(getter)]
  pub fn columns(&self) -> HashMap<String, ColumnData> {
    self.columns.clone()
  }
}

fn infer_dtype(s: &str) -> DataType {
  if s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("false") {
    return DataType::Boolean;
  }
  let ires = s.parse::<i64>();
  if ires.is_ok() {
    return DataType::Integer;
  }
  let fres = s.parse::<f64>();
  if fres.is_ok() {
    return DataType::Float;
  }

  DataType::String
}

fn str_to_bool(s: &str) -> Option<bool> {
  if s.eq_ignore_ascii_case("true") {
    return Some(true);
  } else if s.eq_ignore_ascii_case("false") {
    return Some(false);
  }
  None
}

#[napi]
pub fn read_csv(path: String) -> Result<DataFrame> {
  let mut reader = Reader::from_path(&path)?;
  let header = reader.headers()?.to_owned();
  let header_len = header.len();
  let mut i = 0 as u32;
  let mut col_to_vec: Vec<Vec<Box<str>>> = (0..header_len).map(|_| Vec::new()).collect();
  let col_idx: Vec<&str> = header.iter().collect();
  let mut dtypes: HashMap<usize, DataType> = HashMap::new();

  for record in reader.records() {
    i += 1;
    let rec = record?;
    if rec.len() != header_len {
      return Err(anyhow!(
        "Expected length {:?} at row {:?}, got {:?}",
        header_len,
        i,
        rec.len()
      ));
    }
    for (i, s) in rec.iter().enumerate() {
      if !dtypes.contains_key(&i) {
        let dtype = infer_dtype(s);
        dtypes.insert(i, dtype);
      }
      col_to_vec[i].push(s.into());
    }
  }

  let mut cols: HashMap<String, ColumnData> = HashMap::new();
  for (c, d) in &dtypes {
    let vc = &col_to_vec[*c];
    let data: ColumnData = match d {
      DataType::Boolean => {
        let v = vc
          .iter()
          .enumerate()
          .map(|(i, s)| {
            str_to_bool(s).expect(&format!("Expecting bool at line {:?} for col {}", i, c))
          })
          .collect();
        ColumnData::Boolean(v)
      }
      DataType::Float => {
        let v = vc
          .iter()
          .enumerate()
          .map(|(i, s)| {
            s.parse::<f64>()
              .expect(&format!("Expecting float at line {:?} for col {}", i, c))
          })
          .collect();

        ColumnData::Float(v)
      }
      DataType::String => {
        let v = vc.iter().map(|s| s.to_string()).collect();
        ColumnData::String(v)
      }
      DataType::Integer => {
        let v = vc
          .iter()
          .enumerate()
          .map(|(i, s)| {
            s.parse::<i64>()
              .expect(&format!("Expecting integer at line {:?} for col {}", i, c))
          })
          .collect();

        ColumnData::Integer(v)
      }
    };
    cols.insert(col_idx[*c].to_owned(), data);
  }

  let df = DataFrame::new(cols, i);

  Ok(df)
}
