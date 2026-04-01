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
impl CsvValue {
  #[napi]
  pub fn new_from(s: String) -> Self {
    let ures = s.parse::<i64>();
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

  #[napi]
  pub fn dtype(&self) -> DataType {
    match self {
      Self::Boolean(_) => DataType::Boolean,
      Self::Float(_) => DataType::Float,
      Self::String(_) => DataType::String,
      Self::Integer(_) => DataType::Integer,
    }
  }

  #[napi]
  pub fn as_float(&self) -> Option<f64> {
    match self {
      Self::Float(f) => Some(f.to_owned()),
      Self::Integer(i) => Some(i.to_owned() as f64),
      _ => None,
    }
  }

  #[napi]
  pub fn as_int(&self) -> Option<i64> {
    match self {
      Self::Integer(i) => Some(i.to_owned()),
      _ => None,
    }
  }

  #[napi]
  pub fn as_bool(&self) -> Option<bool> {
    match self {
      Self::Boolean(b) => Some(b.to_owned()),
      _ => None,
    }
  }

  #[napi]
  pub fn as_string(&self) -> Option<String> {
    match self {
      Self::String(s) => Some(s.to_owned()),
      _ => None,
    }
  }
}

#[napi(object)]
pub struct DataFrame {
  pub columns: HashMap<String, Vec<CsvValue>>,
  pub len: u16,
  pub dtypes: HashMap<String, DataType>,
}

#[napi]
impl DataFrame {
  fn new(
    columns: HashMap<String, Vec<CsvValue>>,
    len: u16,
    dtypes: HashMap<String, DataType>,
  ) -> Self {
    return Self {
      columns,
      len,
      dtypes,
    };
  }

  #[napi]
  pub fn col_dtype(&self, col: String) -> Option<DataType> {
    if let Some(d) = self.dtypes.get(&col) {
      return Some(d.clone());
    }
    None
  }

  #[napi]
  pub fn get_as_string_array(&self, col: String) -> Result<Vec<String>> {
    if let Some(d) = self.dtypes.get(&col) {
      if d.to_owned() == DataType::String {
        let v: Vec<String> = self
          .columns
          .get(&col)
          .unwrap()
          .iter()
          .map(|val| val.as_string().unwrap())
          .collect();
        return Ok(v);
      }
      return Err(anyhow!("Column {} has type {}, not string", col, d));
    }

    Err(anyhow!("Could not find column {}", col))
  }

  #[napi]
  pub fn get_as_float_array(&self, col: String) -> Result<Vec<f64>> {
    if let Some(d) = self.dtypes.get(&col) {
      if d.to_owned() == DataType::String {
        let v: Vec<f64> = self
          .columns
          .get(&col)
          .unwrap()
          .iter()
          .map(|val| val.as_float().unwrap())
          .collect();
        return Ok(v);
      }
      return Err(anyhow!("Column {} has type {}, not float", col, d));
    }

    Err(anyhow!("Could not find column {}", col))
  }

  #[napi]
  pub fn get_as_int_array(&self, col: String) -> Result<Vec<i64>> {
    if let Some(d) = self.dtypes.get(&col) {
      if d.to_owned() == DataType::String {
        let v: Vec<i64> = self
          .columns
          .get(&col)
          .unwrap()
          .iter()
          .map(|val| val.as_int().unwrap())
          .collect();
        return Ok(v);
      }
      return Err(anyhow!("Column {} has type {}, not integer", col, d));
    }

    Err(anyhow!("Could not find column {}", col))
  }

  #[napi]
  pub fn get_as_bool_array(&self, col: String) -> Result<Vec<bool>> {
    if let Some(d) = self.dtypes.get(&col) {
      if d.to_owned() == DataType::String {
        let v: Vec<bool> = self
          .columns
          .get(&col)
          .unwrap()
          .iter()
          .map(|val| val.as_bool().unwrap())
          .collect();
        return Ok(v);
      }
      return Err(anyhow!("Column {} has type {}, not integer", col, d));
    }

    Err(anyhow!("Could not find column {}", col))
  }
}

#[napi]
pub fn read_csv(path: String) -> Result<DataFrame> {
  let mut reader = Reader::from_path(&path)?;
  let header = reader.headers()?.to_owned();
  let mut i = 0 as u16;
  let mut columns: HashMap<String, Vec<CsvValue>> = header
    .iter()
    .map(|e| {
      let v: Vec<CsvValue> = Vec::new();
      (e.to_string(), v)
    })
    .collect();
  let col_idx: HashMap<usize, String> = header
    .iter()
    .enumerate()
    .map(|(i, s)| (i, s.to_string()))
    .collect();
  let mut dtypes: HashMap<String, DataType> = HashMap::new();
  for result in reader.records() {
    i += 1;
    let res = result?.to_owned();
    if res.len() != header.len() {
      return Err(anyhow!(
        "Line {:?} has {:?} records, expected {:?}",
        i,
        res.len(),
        header.len()
      ));
    }
    for (i, s) in res.iter().enumerate() {
      let col = &col_idx[&i];
      let mut value = CsvValue::new_from(s.to_string());
      if let Some(d) = dtypes.get(col) {
        if d == &DataType::Integer && value.dtype() == DataType::Float {
          dtypes.insert(col.to_owned(), DataType::Float);
        } else if d != &DataType::String && value.dtype() == DataType::String {
          dtypes.insert(col.to_owned(), DataType::String);
        } else if d == &DataType::String && value.dtype() != DataType::String {
          value = CsvValue::String(s.to_string());
        } else if d != &value.dtype() && d != &DataType::String && value.dtype() != DataType::String
        {
          return Err(anyhow!(
            "Line {:?} has type {}, expected {} for {}",
            i,
            value.dtype(),
            d,
            col,
          ));
        }
      } else {
        dtypes.insert(col.to_owned(), value.dtype());
      }
      let vec = columns.get_mut(col).unwrap();
      vec.push(value);
    }
  }

  let df = DataFrame::new(columns, i, dtypes);

  Ok(df)
}
