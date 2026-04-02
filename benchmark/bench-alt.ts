import { Bench } from 'tinybench'
import { parse } from 'csv-parse'
import fs from 'node:fs'
import { finished } from 'stream/promises'

const b = new Bench()

// const processFile = async () => {
//     const records = [];
//     const parser = fs.createReadStream(`${os.tmpdir()}/input.csv`).pipe(
//       parse({
//         // CSV options if any
//       }),
//     );
//     parser.on("readable", function () {
//       let record;
//       while ((record = parser.read()) !== null) {
//         // Work with each record
//         records.push(record);
//       }
//     });
//     await finished(parser);
//     return records;
//   };

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

await b.run()

console.table(b.table())
