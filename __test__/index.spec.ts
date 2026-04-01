import test from 'ava'

import { readCsv, DataType } from '../index'

test('readCsv reads a CSV and returns a DataFrame', (t) => {
  const csvPath = 'testfiles/customers-100.csv'
  const df = readCsv(csvPath)
  t.is(df.len, 100)
  t.is(Object.keys(df.columns).length, 12)
  t.is(Object.keys(df.dtypes).length, 12)
  t.true(Object.keys(df.dtypes).includes('Index'))
  t.true(Object.keys(df.dtypes).includes('City'))
  t.is(df.dtypes['Index'], DataType.Integer)
  t.is(df.dtypes['City'], DataType.String)
})
