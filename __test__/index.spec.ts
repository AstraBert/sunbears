import test from 'ava'

import {
  readCsv,
  DataType,
  DataFrame,
  ColumnData,
  asBooleanArray,
  asFloatArray,
  asIntArray,
  asStringArray,
} from '../index'

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

test('DataFrame class methods work correctly', (t) => {
  const columns: Record<string, ColumnData> = {
    test: { type: 'String', field0: ['hello', 'world'] },
    other: { type: 'Boolean', field0: [false, true] },
    another: { type: 'Integer', field0: [1, 2] },
    last_one: { type: 'Float', field0: [0.3, 2.6] },
  }
  const df = new DataFrame(columns, 2)
  t.is(df.len, 2)
  t.deepEqual(df.columns, columns)
  t.is(df.colDtype('test'), DataType.String)
  t.is(df.colDtype('other'), DataType.Boolean)
  t.is(df.colDtype('another'), DataType.Integer)
  t.is(df.colDtype('last_one'), DataType.Float)
  t.deepEqual(df.get('test'), columns['test'])
  t.deepEqual(df.get('other'), columns['other'])
  t.deepEqual(df.get('another'), columns['another'])
  t.deepEqual(df.get('last_one'), columns['last_one'])
})

test('Column to array functions work', (t) => {
  const columns: Record<string, ColumnData> = {
    name: { type: 'String', field0: ['hello', 'world'] },
    other: { type: 'Boolean', field0: [false, true] },
    another: { type: 'Integer', field0: [1, 2] },
    last_one: { type: 'Float', field0: [0.3, 2.6] },
  }
  const name = columns['name']
  const nameArray = asStringArray(name as ColumnData)
  t.truthy(nameArray)
  t.deepEqual(nameArray, name.field0)
  t.falsy(asIntArray(name as ColumnData))
  t.falsy(asFloatArray(name as ColumnData))
  t.falsy(asBooleanArray(name as ColumnData))
  const otherArray = asBooleanArray(columns['other'])
  t.truthy(otherArray)
  t.deepEqual(otherArray, columns['other'].field0)
  const anotherArray = asIntArray(columns['another'])
  t.truthy(anotherArray)
  t.deepEqual(anotherArray, columns['another'].field0)
  const lastArray = asFloatArray(columns['last_one'])
  t.truthy(lastArray)
  t.deepEqual(lastArray, columns['last_one'].field0)
})
