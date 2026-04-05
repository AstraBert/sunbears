#![deny(clippy::all)]

use std::{collections::HashMap, fmt, fs::File, io::BufWriter};

use anyhow::{anyhow, Result};

use csv::{Reader, Writer};
use napi_derive::napi;

// ---------------------------------------- DATA MODELS ----------------------------------------

#[napi]
#[derive(Clone)]
pub enum ColumnData {
  String(Vec<String>),
  Integer(Vec<i64>),
  Float(Vec<f64>),
  Boolean(Vec<bool>),
}

impl ColumnData {
  fn len(&self) -> usize {
    match self {
      Self::Boolean(b) => b.len(),
      Self::Float(f) => f.len(),
      Self::Integer(i) => i.len(),
      Self::String(s) => s.len(),
    }
  }
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

// ---------------------------------------- HELPER FUNCTIONS FOR DATA MODELS ----------------------------------------

#[napi]
pub fn as_float_array(column: ColumnData) -> Option<Vec<f64>> {
  match column {
    ColumnData::Float(f) => Some(f),
    _ => None,
  }
}

#[napi]
pub fn as_int_array(column: ColumnData) -> Option<Vec<i64>> {
  match column {
    ColumnData::Integer(f) => Some(f),
    _ => None,
  }
}

#[napi]
pub fn as_boolean_array(column: ColumnData) -> Option<Vec<bool>> {
  match column {
    ColumnData::Boolean(f) => Some(f),
    _ => None,
  }
}

#[napi]
pub fn as_string_array(column: ColumnData) -> Option<Vec<String>> {
  match column {
    ColumnData::String(f) => Some(f),
    _ => None,
  }
}

#[napi]
pub fn to_string_column(data: Vec<String>) -> ColumnData {
  ColumnData::String(data)
}

#[napi]
pub fn to_float_column(data: Vec<f64>) -> ColumnData {
  ColumnData::Float(data)
}

#[napi]
pub fn to_int_column(data: Vec<i64>) -> ColumnData {
  ColumnData::Integer(data)
}

#[napi]
pub fn to_bool_column(data: Vec<bool>) -> ColumnData {
  ColumnData::Boolean(data)
}

// ---------------------------------------- DATAFRAME ----------------------------------------

#[napi(js_name = "DataFrame")]
pub struct DataFrame {
  columns: HashMap<String, ColumnData>,
  pub len: u32,
}

#[napi]
impl DataFrame {
  #[napi(constructor)]
  pub fn new(columns: HashMap<String, ColumnData>, len: u32) -> Result<Self> {
    if !columns.iter().all(|(_, c)| c.len() as u32 == len) {
      return Err(anyhow!("Not all columns are of the declared length"));
    }
    Ok(Self { columns, len })
  }

  #[napi(factory)]
  pub fn from_columns(columns: HashMap<String, ColumnData>) -> Result<Self> {
    let keys: Vec<&String> = columns.keys().collect();
    let col = &columns[keys[0]];
    let l = col.len();
    if !columns.iter().all(|(_, c)| c.len() == l) {
      return Err(anyhow!("Not all columns are the same length"));
    }

    Ok(Self {
      columns,
      len: l as u32,
    })
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

  #[napi]
  pub fn write_csv(&self, path: String) -> Result<()> {
    let file = File::create(&path)?;
    let buf = BufWriter::with_capacity(1 << 20, file); // 1 MB buffer size
    let mut writer = Writer::from_writer(buf);
    let mut float_buf = ryu::Buffer::new();
    let mut int_buf = itoa::Buffer::new();
    let keys: Vec<&String> = self.columns.keys().collect();
    writer.write_record(keys.iter().map(|k| k.as_str()))?;
    for i in 0..self.len as usize {
      let mut row = Vec::with_capacity(keys.len());
      for key in &keys {
        let col = &self.columns[*key];
        match col {
          ColumnData::Boolean(b) => {
            let v = if b[i] {
              "true".to_string()
            } else {
              "false".to_string()
            };
            row.push(v);
          }
          ColumnData::Float(f) => {
            let v = float_buf.format(f[i]).to_string();
            row.push(v);
          }
          ColumnData::Integer(j) => {
            let v = int_buf.format(j[i]).to_string();
            row.push(v);
          }
          ColumnData::String(s) => {
            row.push(s[i].clone());
          }
        }
      }
      writer.write_record(&row)?;
    }
    writer.flush()?;

    Ok(())
  }
}

// ---------------------------------------- CSV READING ----------------------------------------

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
  let mut i = 0_u32;
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
      dtypes.entry(i).or_insert_with(|| infer_dtype(s));
      col_to_vec[i].push(s.into());
    }
  }

  let mut cols: HashMap<String, ColumnData> = HashMap::new();
  for (c, d) in &dtypes {
    let vc = &col_to_vec[*c];
    let data: ColumnData = match d {
      DataType::Boolean => {
        let mut v: Vec<bool> = vec![];
        for el in vc {
          match str_to_bool(el) {
            Some(val) => v.push(val),
            None => {
              return Err(anyhow!(
                "Expecting bool at line {:?} for col {}",
                i + 1,
                col_idx[*c]
              ))
            }
          }
        }
        ColumnData::Boolean(v)
      }
      DataType::Float => {
        let mut v: Vec<f64> = vec![];
        for s in vc {
          match s.parse::<f64>() {
            Ok(val) => v.push(val),
            Err(_) => {
              return Err(anyhow!(
                "Expecting float at line {:?} for col {}",
                i + 1,
                col_idx[*c]
              ))
            }
          }
        }

        ColumnData::Float(v)
      }
      DataType::String => {
        let v = vc.iter().map(|s| s.to_string()).collect();
        ColumnData::String(v)
      }
      DataType::Integer => {
        let mut v: Vec<i64> = vec![];
        for s in vc {
          match s.parse::<i64>() {
            Ok(val) => v.push(val),
            Err(_) => {
              return Err(anyhow!(
                "Expecting integer at line {:?} for col {}",
                i + 1,
                col_idx[*c]
              ))
            }
          }
        }

        ColumnData::Integer(v)
      }
    };
    cols.insert(col_idx[*c].to_owned(), data);
  }

  let df = DataFrame::new(cols, i)?;

  Ok(df)
}
