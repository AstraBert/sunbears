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
  toIntColumn,
  toFloatColumn,
  toStringColumn,
  toBoolColumn,
} from '../index'
import { unlinkSync } from 'fs'

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

test('readCsv throws expected type-dependent errors', (t) => {
  t.throws(
    () => {
      readCsv('testfiles/err-typed-col-1.csv')
    },
    undefined,
    'Expecting integer at line 2 for column colErr',
  )
  t.throws(
    () => {
      readCsv('testfiles/err-typed-col-2.csv')
    },
    undefined,
    'Expecting integer at line 2 for column colErr',
  )
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

test('Column to array functions work correctly', (t) => {
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

test('Array to column functions work correctly', (t) => {
  const intArr = [0, 1, 2, 3]
  const floatArr = [0.1, 1.2, 2.3, 3.4]
  const stringArr = ['hello', 'world', 'how', 'are']
  const boolArr = [true, true, false, false]
  const intCol = toIntColumn(intArr)
  t.deepEqual(intCol.field0, intArr)
  t.is(intCol.type, 'Integer')
  const floatCol = toFloatColumn(floatArr)
  t.deepEqual(floatCol.field0, floatArr)
  t.is(floatCol.type, 'Float')
  const stringCol = toStringColumn(stringArr)
  t.deepEqual(stringCol.field0, stringArr)
  t.is(stringCol.type, 'String')
  const boolCol = toBoolColumn(boolArr)
  t.deepEqual(boolCol.field0, boolArr)
  t.is(boolCol.type, 'Boolean')
})

test('DataFrame constructor and factory work correctly', (t) => {
  const columns: Record<string, ColumnData> = {
    name: { type: 'String', field0: ['hello', 'world'] },
    other: { type: 'Boolean', field0: [false, true] },
    another: { type: 'Integer', field0: [1, 2] },
    last_one: { type: 'Float', field0: [0.3, 2.6] },
  }
  const df = new DataFrame(columns, 2)
  t.is(df.len, 2)
  for (const k of Object.keys(columns)) {
    t.deepEqual(df.get(k), columns[k])
  }
  const df1 = DataFrame.fromColumns(columns)
  t.is(df1.len, 2)
  for (const k of Object.keys(columns)) {
    t.deepEqual(df1.get(k), columns[k])
  }
})

test('DataFrame constructor and factory throw expected errors', (t) => {
  const columns: Record<string, ColumnData> = {
    name: { type: 'String', field0: ['hello', 'world'] },
    other: { type: 'Boolean', field0: [false, true] },
    another: { type: 'Integer', field0: [1, 2] },
    last_one: { type: 'Float', field0: [0.3, 2.6] },
  }
  const columns1: Record<string, ColumnData> = {
    name: { type: 'String', field0: ['hello'] },
    other: { type: 'Boolean', field0: [false, true] },
    another: { type: 'Integer', field0: [1, 2] },
    last_one: { type: 'Float', field0: [0.3, 2.6] },
  }
  t.throws(
    () => {
      new DataFrame(columns, 3)
    },
    undefined,
    'Not all columns are of the declared length',
  )
  t.throws(
    () => {
      DataFrame.fromColumns(columns1)
    },
    undefined,
    'Not all columns are the same length',
  )
})

test('Write to CSV works correctly', (t) => {
  const col1 = toStringColumn(['hello', 'world'])
  const col2 = toFloatColumn([1.2, 2.3])
  const col3 = toIntColumn([4, 5])
  const col4 = toBoolColumn([true, false])
  const dfw = DataFrame.fromColumns({
    col1: col1,
    col2: col2,
    col3: col3,
    col4: col4,
  })
  dfw.writeCsv('testfiles/write-test.csv')
  const dfr = readCsv('testfiles/write-test.csv')
  const colw = dfw.columns
  for (const k of Object.keys(colw)) {
    t.deepEqual(dfr.get(k), colw[k])
  }
  unlinkSync('testfiles/write-test.csv')
})

test('Dropping null values works correctly', (t) => {
  const col1 = toStringColumn(['hello', 'world', null, 'bye', 'moon', 'hey'])
  const col2 = toFloatColumn([1.2, 2.3, 0.3, null, 0.5, 8.9])
  const col3 = toIntColumn([4, 5, 6, 7, null, 8])
  const col4 = toBoolColumn([true, false, true, false, true, null])
  const df = DataFrame.fromColumns({
    col1: col1,
    col2: col2,
    col3: col3,
    col4: col4,
  })
  t.is(df.len, 6)
  df.dropNull()
  t.is(df.len, 2)
  t.deepEqual(asStringArray(df.get('col1')!)!, ['hello', 'world'])
  t.deepEqual(asFloatArray(df.get('col2')!)!, [1.2, 2.3])
  t.deepEqual(asIntArray(df.get('col3')!)!, [4, 5])
  t.deepEqual(asBooleanArray(df.get('col4')!)!, [true, false])
})

test('Reading null values works correctly', (t) => {
  const df = readCsv('testfiles/with-nulls.csv')
  t.is(df.len, 5)
  const col1 = asStringArray(df.get('col1')!)!
  t.is(col1.filter((x) => x === null).length, 1)
  t.deepEqual(col1, ['hello', null, 'bye', 'test', 'ciao'])
  const col2 = asIntArray(df.get('col2')!)!
  t.is(col2.filter((x) => x === null).length, 1)
  t.deepEqual(col2, [1, 2, null, 3, 4])
  const col3 = asFloatArray(df.get('col3')!)!
  t.is(col3.filter((x) => x === null).length, 1)
  t.deepEqual(col3, [0.1, 0.2, 0.3, null, 0.4])
  const col4 = asBooleanArray(df.get('col4')!)!
  t.is(col4.filter((x) => x === null).length, 1)
  t.deepEqual(col4, [true, false, true, false, null])
})

test('Filling null values works correctly', (t) => {
  const df = readCsv('testfiles/with-nulls.csv')
  df.fillNull()
  t.is(df.len, 5)
  const col1 = asStringArray(df.get('col1')!)!
  t.is(col1.filter((x) => x === null).length, 0)
  t.deepEqual(col1, ['hello', '', 'bye', 'test', 'ciao'])
  const col2 = asIntArray(df.get('col2')!)!
  t.is(col2.filter((x) => x === null).length, 0)
  t.deepEqual(col2, [1, 2, 0, 3, 4])
  const col3 = asFloatArray(df.get('col3')!)!
  t.is(col3.filter((x) => x === null).length, 0)
  t.deepEqual(col3, [0.1, 0.2, 0.3, 0.0, 0.4])
  const col4 = asBooleanArray(df.get('col4')!)!
  t.is(col4.filter((x) => x === null).length, 0)
  t.deepEqual(col4, [true, false, true, false, false])
})

test('Dropping NaN values works correctly', (t) => {
  const col1 = toStringColumn(['hello', 'world', null, 'bye', 'moon', 'hey'])
  const col2 = toFloatColumn([1.2, 2.3, 0.3, NaN, 0.5, 8.9])
  const col3 = toIntColumn([4, 5, 6, 7, null, 8])
  const col4 = toBoolColumn([true, false, true, false, true, null])
  const df = DataFrame.fromColumns({
    col1: col1,
    col2: col2,
    col3: col3,
    col4: col4,
  })
  t.is(df.len, 6)
  df.dropNan()
  t.is(df.len, 5)
  t.deepEqual(asStringArray(df.get('col1')!)!, ['hello', 'world', null, 'moon', 'hey'])
  t.deepEqual(asFloatArray(df.get('col2')!)!, [1.2, 2.3, 0.3, 0.5, 8.9])
  t.deepEqual(asIntArray(df.get('col3')!)!, [4, 5, 6, null, 8])
  t.deepEqual(asBooleanArray(df.get('col4')!)!, [true, false, true, true, null])
})

test('Filling NaN values with a specific value works correctly', (t) => {
  const col1 = toFloatColumn([1.0, 2.0, NaN, 3.0, 5.0, null])
  const col2 = toFloatColumn([1.2, 2.3, 0.3, NaN, 0.5, 8.9])
  const df = DataFrame.fromColumns({
    col1: col1,
    col2: col2,
  })
  t.is(df.len, 6)
  df.fillNan(99.3)
  t.is(df.len, 6)
  t.deepEqual(asFloatArray(df.get('col1')!)!, [1.0, 2.0, 99.3, 3.0, 5.0, null])
  t.deepEqual(asFloatArray(df.get('col2')!)!, [1.2, 2.3, 0.3, 99.3, 0.5, 8.9])
})

test('Filling NaN values with the zero value works correctly', (t) => {
  const col1 = toFloatColumn([1.0, 2.0, NaN, 3.0, 5.0, null])
  const col2 = toFloatColumn([1.2, 2.3, 0.3, NaN, 0.5, 8.9])
  const df = DataFrame.fromColumns({
    col1: col1,
    col2: col2,
  })
  t.is(df.len, 6)
  df.fillNan()
  t.is(df.len, 6)
  t.deepEqual(asFloatArray(df.get('col1')!)!, [1.0, 2.0, 0.0, 3.0, 5.0, null])
  t.deepEqual(asFloatArray(df.get('col2')!)!, [1.2, 2.3, 0.3, 0.0, 0.5, 8.9])
})
