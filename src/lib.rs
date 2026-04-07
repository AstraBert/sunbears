#![deny(clippy::all)]

use std::{
  collections::{HashMap, HashSet},
  fmt,
  fs::File,
  io::BufWriter,
};

use anyhow::{anyhow, Result};

use csv::{Reader, Writer};
use napi_derive::napi;

// ---------------------------------------- DATA MODELS ----------------------------------------

#[napi]
#[derive(Clone)]
pub enum ColumnData {
  String(Vec<Option<String>>),
  Integer(Vec<Option<i64>>),
  Float(Vec<Option<f64>>),
  Boolean(Vec<Option<bool>>),
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
pub fn as_float_array(column: ColumnData) -> Option<Vec<Option<f64>>> {
  match column {
    ColumnData::Float(f) => Some(f),
    _ => None,
  }
}

#[napi]
pub fn as_int_array(column: ColumnData) -> Option<Vec<Option<i64>>> {
  match column {
    ColumnData::Integer(f) => Some(f),
    _ => None,
  }
}

#[napi]
pub fn as_boolean_array(column: ColumnData) -> Option<Vec<Option<bool>>> {
  match column {
    ColumnData::Boolean(f) => Some(f),
    _ => None,
  }
}

#[napi]
pub fn as_string_array(column: ColumnData) -> Option<Vec<Option<String>>> {
  match column {
    ColumnData::String(f) => Some(f),
    _ => None,
  }
}

#[napi]
pub fn to_string_column(data: Vec<Option<String>>) -> ColumnData {
  ColumnData::String(data)
}

#[napi]
pub fn to_float_column(data: Vec<Option<f64>>) -> ColumnData {
  ColumnData::Float(data)
}

#[napi]
pub fn to_int_column(data: Vec<Option<i64>>) -> ColumnData {
  ColumnData::Integer(data)
}

#[napi]
pub fn to_bool_column(data: Vec<Option<bool>>) -> ColumnData {
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

  fn get_dtypes(&self) -> Vec<DataType> {
    let mut dtypes = Vec::with_capacity(self.columns.len());
    for k in self.columns().keys() {
      let dt = self.col_dtype(k.clone()).unwrap();
      dtypes.push(dt);
    }
    dtypes
  }

  #[napi]
  pub fn drop_nan(&mut self) {
    let dtypes = self.get_dtypes();
    if dtypes.iter().all(|d| d != &DataType::Float) {
      return;
    }
    let mut idxs = HashSet::new();
    for i in 0..self.len as usize {
      let mut has_nan = false;
      for col in self.columns.values() {
        match col {
          ColumnData::Float(f) => {
            if f[i].unwrap_or_default().is_nan() {
              has_nan = true;
            }
          }
          _ => continue,
        }
        if has_nan {
          break;
        }
      }
      if has_nan {
        idxs.insert(i);
      }
    }
    if !idxs.is_empty() {
      for col in self.columns.values_mut() {
        match col {
          ColumnData::Boolean(b) => {
            let mut i = 0;
            b.retain(|_| {
              let keep = !idxs.contains(&i);
              i += 1;
              keep
            });
          }
          ColumnData::Float(b) => {
            let mut i = 0;
            b.retain(|_| {
              let keep = !idxs.contains(&i);
              i += 1;
              keep
            });
          }
          ColumnData::Integer(b) => {
            let mut i = 0;
            b.retain(|_| {
              let keep = !idxs.contains(&i);
              i += 1;
              keep
            });
          }
          ColumnData::String(b) => {
            let mut i = 0;
            b.retain(|_| {
              let keep = !idxs.contains(&i);
              i += 1;
              keep
            });
          }
        }
      }
      self.len -= idxs.len() as u32;
    }
  }

  #[napi]
  pub fn fill_nan(&mut self, fill_value: Option<f64>) {
    let dtypes = self.get_dtypes();
    if dtypes.iter().all(|d| d != &DataType::Float) {
      return;
    }
    let val = fill_value.unwrap_or_default();
    for col in self.columns.values_mut() {
      match col {
        ColumnData::Float(f) => {
          for item in f.iter_mut().take(self.len as usize) {
            if item.unwrap_or_default().is_nan() {
              *item = Some(val);
            }
          }
        }
        _ => continue,
      }
    }
  }

  #[napi]
  pub fn drop_null(&mut self) {
    let mut idxs = HashSet::new();
    for i in 0..self.len as usize {
      let mut has_none = false;
      for col in self.columns.values() {
        match col {
          ColumnData::Boolean(b) => {
            if b[i].is_none() {
              has_none = true;
            }
          }
          ColumnData::Float(b) => {
            if b[i].is_none() {
              has_none = true;
            }
          }
          ColumnData::String(b) => {
            if b[i].is_none() {
              has_none = true;
            }
          }
          ColumnData::Integer(b) => {
            if b[i].is_none() {
              has_none = true;
            }
          }
        }
        if has_none {
          break;
        }
      }
      if has_none {
        idxs.insert(i);
      }
    }
    if !idxs.is_empty() {
      for col in self.columns.values_mut() {
        match col {
          ColumnData::Boolean(b) => {
            let mut i = 0;
            b.retain(|_| {
              let keep = !idxs.contains(&i);
              i += 1;
              keep
            });
          }
          ColumnData::Float(b) => {
            let mut i = 0;
            b.retain(|_| {
              let keep = !idxs.contains(&i);
              i += 1;
              keep
            });
          }
          ColumnData::Integer(b) => {
            let mut i = 0;
            b.retain(|_| {
              let keep = !idxs.contains(&i);
              i += 1;
              keep
            });
          }
          ColumnData::String(b) => {
            let mut i = 0;
            b.retain(|_| {
              let keep = !idxs.contains(&i);
              i += 1;
              keep
            });
          }
        }
      }
      self.len -= idxs.len() as u32;
    }
  }

  #[napi]
  pub fn fill_null(&mut self) {
    for col in self.columns.values_mut() {
      match col {
        ColumnData::Boolean(b) => {
          for item in b.iter_mut().take(self.len as usize) {
            if item.is_none() {
              *item = Some(false);
            }
          }
        }
        ColumnData::Float(f) => {
          for item in f.iter_mut().take(self.len as usize) {
            if item.is_none() {
              *item = Some(0_f64);
            }
          }
        }
        ColumnData::String(s) => {
          for item in s.iter_mut().take(self.len as usize) {
            if item.is_none() {
              *item = Some(String::new());
            }
          }
        }
        ColumnData::Integer(j) => {
          for item in j.iter_mut().take(self.len as usize) {
            if item.is_none() {
              *item = Some(0_i64);
            }
          }
        }
      }
    }
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
            let v = if b[i].unwrap_or_default() {
              "true".to_string()
            } else {
              "false".to_string()
            };
            row.push(v);
          }
          ColumnData::Float(f) => {
            let v = float_buf.format(f[i].unwrap_or_default()).to_string();
            row.push(v);
          }
          ColumnData::Integer(j) => {
            let v = int_buf.format(j[i].unwrap_or_default()).to_string();
            row.push(v);
          }
          ColumnData::String(s) => {
            row.push(s[i].clone().unwrap_or_default());
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
        let mut v: Vec<Option<bool>> = Vec::with_capacity(vc.len());
        for el in vc {
          if el.is_empty() {
            v.push(None);
            continue;
          }
          match str_to_bool(el) {
            Some(val) => v.push(Some(val)),
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
        let mut v: Vec<Option<f64>> = Vec::with_capacity(vc.len());
        for s in vc {
          if s.is_empty() {
            v.push(None);
            continue;
          }
          match s.parse::<f64>() {
            Ok(val) => v.push(Some(val)),
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
        let mut v: Vec<Option<String>> = Vec::with_capacity(vc.len());
        for s in vc {
          if s.is_empty() {
            v.push(None);
            continue;
          }
          v.push(Some(s.to_string()));
        }
        ColumnData::String(v)
      }
      DataType::Integer => {
        let mut v: Vec<Option<i64>> = Vec::with_capacity(vc.len());
        for s in vc {
          if s.is_empty() {
            v.push(None);
            continue;
          }
          match s.parse::<i64>() {
            Ok(val) => v.push(Some(val)),
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
