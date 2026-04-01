import test from 'ava'

import { readCsv, DataType } from '../index'

test('readCsv reads a CSV and returns a DataFrame with correct datatypes', (t) => {
  const csvPath = 'testfiles/generated-100.csv'
  const df = readCsv(csvPath)
  t.is(df.len, 100)
  t.is(Object.keys(df.columns).length, 6)
  t.true(Object.keys(df.columns).includes('name'))
  t.true(Object.keys(df.columns).includes('age'))
  t.true(Object.keys(df.columns).includes('is_working'))
  t.true(Object.keys(df.columns).includes('height'))
  t.true(Object.keys(df.columns).includes('phone_number'))
  t.true(Object.keys(df.columns).includes('years_of_experience'))
  t.is(df.colDtype('name'), DataType.String)
  t.is(df.colDtype('age'), DataType.Integer)
  t.is(df.colDtype('is_working'), DataType.Boolean)
  t.is(df.colDtype('height'), DataType.Float)
  t.is(df.colDtype('phone_number'), DataType.String)
  t.is(df.colDtype('years_of_experience'), DataType.Float)
  t.truthy(df.get('name'))
  t.truthy(df.get('age'))
  t.truthy(df.get('is_working'))
  t.truthy(df.get('height'))
  t.truthy(df.get('phone_number'))
  t.truthy(df.get('years_of_experience'))
})
