import { Bench } from 'tinybench'

import { readCsv } from '../index.js'

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

await b.run()

console.table(b.table())
