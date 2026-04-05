/* eslint-disable */

import { Bench } from 'tinybench'

import { DataFrame, readCsv, toBoolColumn, toFloatColumn, toIntColumn, toStringColumn } from '../index.js'
import { unlinkSync } from 'fs'

const b = new Bench()

b.add('Read a 100-lines CSV', () => {
  console.log('Started 100 lines bench')
  readCsv('testfiles/generated-100.csv')
  console.log('Ended 100 lines bench')
})

b.add('Read a 1000-lines CSV', () => {
  console.log('Started 1000 lines bench')
  readCsv('testfiles/generated-1000.csv')
  console.log('Ended 1000 lines bench')
})

b.add('Read a 100000-lines CSV', () => {
  console.log('Started 100.000 lines bench')
  readCsv('testfiles/generated-100000.csv')
  console.log('Ended 100.000 lines bench')
})

b.add('Read a 1000000-lines CSV', () => {
  console.log('Started 1.000.000 lines bench')
  readCsv('testfiles/generated-1000000.csv')
  console.log('Ended 1.000.000 lines bench')
})

let df: DataFrame | undefined = undefined
let df_100: DataFrame | undefined = undefined
let df_1000: DataFrame | undefined = undefined
let df_100000: DataFrame | undefined = undefined

b.add(
  'Write a 100-lines CSV',
  () => {
    console.log('Started writing 100 lines bench')
    df_100!.writeCsv('testfiles/written-100.csv')
    console.log('Ended writing 100 lines bench')
  },
  {
    beforeAll: () => {
      const col1 = toStringColumn(new Array(100).fill('something'))
      const col2 = toFloatColumn(new Array(100).fill(1.0))
      const col3 = toIntColumn(new Array(100).fill(3))
      const col4 = toBoolColumn(new Array(100).fill(true))
      df_100 = DataFrame.fromColumns({
        col1: col1,
        col2: col2,
        col3: col3,
        col4: col4,
      })
    },
    afterEach: () => {
      unlinkSync('testfiles/written-100.csv')
    },
  },
)

b.add(
  'Write a 1000-lines CSV',
  () => {
    console.log('Started writing 1000 lines bench')
    df_1000!.writeCsv('testfiles/written-1000.csv')
    console.log('Ended writing 1000 lines bench')
  },
  {
    beforeAll: () => {
      const col1 = toStringColumn(new Array(1000).fill('something'))
      const col2 = toFloatColumn(new Array(1000).fill(1.0))
      const col3 = toIntColumn(new Array(1000).fill(3))
      const col4 = toBoolColumn(new Array(1000).fill(true))
      df_1000 = DataFrame.fromColumns({
        col1: col1,
        col2: col2,
        col3: col3,
        col4: col4,
      })
    },
    afterEach: () => {
      unlinkSync('testfiles/written-1000.csv')
    },
  },
)

b.add(
  'Write a 100000-lines CSV',
  () => {
    console.log('Started writing 100.000 lines bench')
    df_100000!.writeCsv('testfiles/written-100000.csv')
    console.log('Ended writing 100.000 lines bench')
  },
  {
    beforeAll: () => {
      const col1 = toStringColumn(new Array(100_000).fill('something'))
      const col2 = toFloatColumn(new Array(100_000).fill(1.0))
      const col3 = toIntColumn(new Array(100_000).fill(3))
      const col4 = toBoolColumn(new Array(100_000).fill(true))
      df_100000 = DataFrame.fromColumns({
        col1: col1,
        col2: col2,
        col3: col3,
        col4: col4,
      })
    },
    afterEach: () => {
      unlinkSync('testfiles/written-100000.csv')
    },
  },
)

b.add(
  'Write a 1000000-lines CSV',
  () => {
    console.log('Started writing 1.000.000 lines bench')
    df!.writeCsv('testfiles/written-1000000.csv')
    console.log('Ended writing 1.000.000 lines bench')
  },
  {
    beforeAll: () => {
      const col1 = toStringColumn(new Array(1_000_000).fill('something'))
      const col2 = toFloatColumn(new Array(1_000_000).fill(1.0))
      const col3 = toIntColumn(new Array(1_000_000).fill(3))
      const col4 = toBoolColumn(new Array(1_000_000).fill(true))
      df = DataFrame.fromColumns({
        col1: col1,
        col2: col2,
        col3: col3,
        col4: col4,
      })
    },
    afterEach: () => {
      unlinkSync('testfiles/written-1000000.csv')
    },
  },
)

await b.run()

console.table(b.table())
