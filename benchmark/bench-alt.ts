/* eslint-disable */

import { Bench } from 'tinybench'
import { parse } from 'csv-parse'
import fs, { unlinkSync, writeFileSync } from 'node:fs'
import { finished } from 'stream/promises'
import { stringify } from 'csv-stringify/sync'

const b = new Bench()

b.add('Read a 100-lines CSV', async () => {
  console.log('Started 100 lines bench')
  const parser = fs.createReadStream(`testfiles/generated-100.csv`).pipe(parse({ toLine: 100 }))
  parser.on('readable', function () {
    while (parser.read() !== null) {
      continue
    }
  })
  await finished(parser)
  console.log('Ended 100 lines bench')
})

b.add('Read a 1000-lines CSV', async () => {
  console.log('Started 1000 lines bench')
  const parser = fs.createReadStream(`testfiles/generated-1000.csv`).pipe(parse({ toLine: 1000 }))
  parser.on('readable', function () {
    while (parser.read() !== null) {
      continue
    }
  })
  await finished(parser)

  console.log('Ended 1000 lines bench')
})

b.add('Read a 100000-lines CSV', async () => {
  console.log('Started 100.000 lines bench')
  const parser = fs.createReadStream(`testfiles/generated-100000.csv`).pipe(parse({ toLine: 100_000 }))
  parser.on('readable', function () {
    while (parser.read() !== null) {
      continue
    }
  })
  await finished(parser)

  console.log('Ended 100.000 lines bench')
})

b.add('Read a 1000000-lines CSV', async () => {
  console.log('Started 1.000.000 lines bench')
  const parser = fs.createReadStream(`testfiles/generated-1000000.csv`).pipe(parse({ toLine: 1_000_000 }))
  parser.on('readable', function () {
    while (parser.read() !== null) {
      continue
    }
  })
  await finished(parser)
  console.log('Ended 1.000.000 lines bench')
})

let records_100: any[][] | undefined
let records_1000: any[][] | undefined
let records_100000: any[][] | undefined
let records_1000000: any[][] | undefined

b.add(
  'Write a 100-lines CSV',
  async () => {
    console.log('Started writing 100 lines bench')
    const output = stringify(records_100!)
    writeFileSync('testfiles/written-alt-100.csv', output)
    console.log('Ended writing 100 lines bench')
  },
  {
    beforeAll: () => {
      const col1 = new Array(100).fill('something')
      const col2 = new Array(100).fill(1.0)
      const col3 = new Array(100).fill(3)
      const col4 = new Array(100).fill(true)
      const cols = [col1, col2, col3, col4]
      let i = 0
      const matrix: any[][] = []
      while (i < 100) {
        let row = []
        for (const col of cols) {
          row.push(col[i])
        }
        matrix.push(row)
        i++
      }
      records_100 = matrix
    },
    afterEach: () => {
      unlinkSync('testfiles/written-alt-100.csv')
    },
  },
)

b.add(
  'Write a 1000-lines CSV',
  async () => {
    console.log('Started writing 1000 lines bench')
    const output = stringify(records_1000!)
    writeFileSync('testfiles/written-alt-1000.csv', output)
    console.log('Ended writing 1000 lines bench')
  },
  {
    beforeAll: () => {
      const col1 = new Array(1000).fill('something')
      const col2 = new Array(1000).fill(1.0)
      const col3 = new Array(1000).fill(3)
      const col4 = new Array(1000).fill(true)
      const cols = [col1, col2, col3, col4]
      let i = 0
      const matrix: any[][] = []
      while (i < 1000) {
        const row = []
        for (const col of cols) {
          row.push(col[i])
        }
        matrix.push(row)
        i++
      }
      records_1000 = matrix
    },
    afterEach: () => {
      unlinkSync('testfiles/written-alt-1000.csv')
    },
  },
)

b.add(
  'Write a 100.000-lines CSV',
  async () => {
    console.log('Started writing 100.000 lines bench')
    const output = stringify(records_100000!)
    writeFileSync('testfiles/written-alt-100000.csv', output)
    console.log('Ended writing 100.000 lines bench')
  },
  {
    beforeAll: () => {
      const col1 = new Array(100000).fill('something')
      const col2 = new Array(100000).fill(1.0)
      const col3 = new Array(100000).fill(3)
      const col4 = new Array(100000).fill(true)
      const cols = [col1, col2, col3, col4]
      let i = 0
      const matrix: any[][] = []
      while (i < 100000) {
        const row = []
        for (const col of cols) {
          row.push(col[i])
        }
        matrix.push(row)
        i++
      }
      records_100000 = matrix
    },
    afterEach: () => {
      unlinkSync('testfiles/written-alt-100000.csv')
    },
  },
)

b.add(
  'Write a 1.000.000-lines CSV',
  async () => {
    console.log('Started writing 1.000.000 lines bench')
    const output = stringify(records_1000000!)
    writeFileSync('testfiles/written-alt-1000000.csv', output)
    console.log('Ended writing 1.000.000 lines bench')
  },
  {
    beforeAll: () => {
      const col1 = new Array(1000000).fill('something')
      const col2 = new Array(1000000).fill(1.0)
      const col3 = new Array(1000000).fill(3)
      const col4 = new Array(1000000).fill(true)
      const cols = [col1, col2, col3, col4]
      let i = 0
      const matrix: any[][] = []
      while (i < 1000000) {
        const row = []
        for (const col of cols) {
          row.push(col[i])
        }
        matrix.push(row)
        i++
      }
      records_1000000 = matrix
    },
    afterEach: () => {
      unlinkSync('testfiles/written-alt-1000000.csv')
    },
  },
)

await b.run()

console.table(b.table())
