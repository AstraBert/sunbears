# `sunbears`

A CSV data loader for TypeScript with an API similar to Polars and Pandas, written in pure Rust.

## Installation

Install the package with your favorite package manager:

```bash
npm install @cle-does-things/sunbears
```

## Usage

### `readCsv`

The main function for `sunbears` is `readCsv`, which loads the data contained in a CSV file as a `DataFrame`, a columnar data format.

```typescript
import { readCsv } from '@cle-does-things/sunbears'

const df = readCsv('test.csv')
```

The `DataFrame` class exposes two methods:

- `colDtype`: retrieve the data type of the records contained within a column (integer, float, boolean or string)
- `get`: get a column
- `writeCsv`: write the dataframe to CSV (see the next paragraph)

```typescript
const dt = df.colDtype('name')
const colData = df.get('name')
```

Based on the data type of the column, you can use one of the following helper functions to extract the associated array of data (as `string[]`, `boolean[]` or `number[]`):

```typescript
import { DataType, asBooleanArray, asFloatArray, asIntArray, asStringArray } from '@cle-does-things/sunbears'

let arr
switch (dt) {
  case DataType.Float:
    arr = asFloatArray(colData)
    break
  case DataType.Integer:
    arr = asIntArray(colData)
    break
  case DataType.Boolean:
    arr = asBooleanArray(colData)
    break
  default:
    arr = asStringArray(colData)
    break
}
```

If the helper function is used on the wrong data type, it will return `null`.

You can then chain these methods and functions to perform `filter` or `map` operations (natively supported by TypeScript arrays):

```typescript
const filteredNames = asStringArray(readCsv('test.csv').get('name'))?.filter((n) => n === 'John Doe')
const mappedNames = asStringArray(readCsv('test.csv').get('name'))?.map((n) => n.toUpperCase())
```

### `DataFrame.writeCsv`

The `writeCsv` method writes a DataFrame to CSV.

You can construct a DataFrame simply starting from arrays, using the following helper functions:

```typescript
import { DataFrame, toIntColumn, toFloatColumn, toStringColumn, toBoolColumn } from '@cle-does-things/sunbears'

const col1 = toStringColumn(['hello', 'world'])
const col2 = toFloatColumn([1.2, 2.3])
const col3 = toIntColumn([4, 5])
const col4 = toBoolColumn([true, false])
```

You can then use the `fromColumns` factory for the `DataFrame` class to turn column data into a DataFrame: if the columns do not have the same length, an error will be thrown.

```typescript
const df = DataFrame.fromColumns({
  col1: col1,
  col2: col2,
  col3: col3,
  col4: col4,
})
```

Writing to the CSV file is then trivial:

```typescript
df.writeCsv('test.csv')
```

The file will look like this:

```csv
col1,col2,col3,col4
hello,1.2,4,true
world,2.3,5,false
```

## Benchmarking

`sunbears` was benchmarked using the `tinybench`-based script you can find [here](./benchmark/bench.ts). The script reports latency statistics related to the `readCsv` and `writeCsv` functions reading/writing increasingly large CSV files (100, 1000, 100.000 and 1.000.000 rows).

The latest benchmark run was:

| Task                      | Latency avg (s)  | Latency med (s)      | Throughput avg (ops/s) | Throughput med (ops/s) | Samples |
| ------------------------- | ---------------- | -------------------- | ---------------------- | ---------------------- | ------- |
| Read a 100-lines CSV      | 0.000054 ± 0.24% | 0.000050 ± 0.0000023 | 18964 ± 0.16%          | 19967 ± 921            | 18654   |
| Read a 1000-lines CSV     | 0.000289 ± 0.54% | 0.000279 ± 0.0000090 | 3518 ± 0.32%           | 3583 ± 116             | 3464    |
| Read a 100000-lines CSV   | 0.028000 ± 1.38% | 0.027537 ± 0.000254  | 36 ± 1.19%             | 36 ± 0                 | 64      |
| Read a 1000000-lines CSV  | 0.310751 ± 0.75% | 0.308330 ± 0.004228  | 3 ± 0.70%              | 3 ± 0                  | 64      |
| Write a 100-lines CSV     | 0.000076 ± 0.46% | 0.000069 ± 0.0000052 | 13665 ± 0.28%          | 14467 ± 1148           | 13140   |
| Write a 1000-lines CSV    | 0.000213 ± 0.43% | 0.000209 ± 0.0000056 | 4724 ± 0.16%           | 4785 ± 130             | 4700    |
| Write a 100000-lines CSV  | 0.013886 ± 0.86% | 0.013756 ± 0.000171  | 72 ± 0.78%             | 73 ± 1                 | 73      |
| Write a 1000000-lines CSV | 0.146282 ± 1.39% | 0.146108 ± 0.005752  | 7 ± 1.31%              | 7 ± 0                  | 64      |

Here is how the tool compares to the `read_csv` and `to_csv`/`write_csv` functions in Pandas and Polars ([script](./testfiles/comparative_bench.py)):

| Dataset             | Pandas (s) | Polars (s) |
| ------------------- | ---------- | ---------- |
| Read 100 lines      | 0.038291   | 0.033831   |
| Read 1000 lines     | 0.037794   | 0.016517   |
| Read 100000 lines   | 0.471109   | 0.029076   |
| Read 1000000 lines  | 4.153507   | 0.216254   |
| Write 100 lines     | 0.035926   | 0.043052   |
| Write 1000 lines    | 0.067816   | 0.017617   |
| Write 100000 lines  | 0.892885   | 0.031329   |
| Write 1000000 lines | 8.549390   | 0.331897   |

And here it how it compares with `csv-parse` and `csv-stringify`+`writeFileSync` ([script](./benchmark/bench-alt.ts)):

| Task                      | Latency avg (s)  | Latency med (s)      | Throughput avg (ops/s) | Throughput med (ops/s) | Samples |
| ------------------------- | ---------------- | -------------------- | ---------------------- | ---------------------- | ------- |
| Read a 100-lines CSV      | 0.000207 ± 2.31% | 0.000191 ± 0.0000087 | 5086 ± 0.31%           | 5224 ± 244             | 4842    |
| Read a 1000-lines CSV     | 0.001244 ± 0.42% | 0.001233 ± 0.0000229 | 806 ± 0.33%            | 811 ± 15               | 805     |
| Read a 100000-lines CSV   | 0.120565 ± 0.63% | 0.119515 ± 0.001141  | 8 ± 0.60%              | 8 ± 0                  | 64      |
| Read a 1000000-lines CSV  | 1.216019 ± 0.46% | 1.209978 ± 0.006709  | 1 ± 0.44%              | 1 ± 0                  | 64      |
| Write a 100-lines CSV     | 0.000087 ± 0.52% | 0.000080 ± 0.0000078 | 12010 ± 0.31%          | 12526 ± 1267           | 11503   |
| Write a 1000-lines CSV    | 0.000290 ± 1.14% | 0.000275 ± 0.0000192 | 3555 ± 0.42%           | 3635 ± 258             | 3451    |
| Write a 100000-lines CSV  | 0.027303 ± 2.96% | 0.027014 ± 0.000900  | 37 ± 1.87%             | 37 ± 1                 | 64      |
| Write a 1000000-lines CSV | 0.273814 ± 2.08% | 0.265154 ± 0.005824  | 4 ± 1.77%              | 4 ± 0                  | 64      |

## Development

**Requirements:**

- Install the latest `Rust`
- Install `Node.js@10+` which fully supported `Node-API`
- Install `yarn@1.x`

### Test locally

- yarn (install)
- yarn build (build package based on `src/lib.rs`)
- yarn test

And you will see something along the lines of:

```bash
$ ava

  ✔ readCsv reads a CSV and returns a DataFrame with correct datatypes
  ✔ DataFrame class methods work correctly
  ✔ Column to array functions work
  ─

  3 tests passed
```

### Benchmarks

> _To run benchmarks, you will need [`uv`](<[https://](https://docs.astral.sh/uv/)>) installed (for benchmark data generation)_

Run benchmarks with:

```bash
yarn bench
yarn bench:comp-py # compare with python libraries
yarn bench:comp-ts # compare with csv-parse
```

The commands will generate `testfiles/generated-*.csv` files (with 100, 1000, 100.000 and 1.000.000 rows), and will perform time-based benchmarks for the `readCsv` (sunbears), `read_csv` (Pandas/Polars) and `parse` (csv-parse) functions.

### Linting and Formatting

You can run formatting for TypeScript, TOML and Rust code with one command:

```bash
yarn format
```

You can also run specific formatting checks:

```bash
yarn format:rs
yarn format:rs-check # checks formatting, without modifying files
yarn format:prettier
yarn format:toml
```

For linting, you need to run both `oxlint` and `clippy`:

```bash
yarn lint # oxlint
yarn clippy # clippy
```

### Release package

> _Only necessary for maintainers_

> ![NOTE]
>
> Ensure you have set your **NPM_TOKEN** in the `GitHub` project setting.

When you want to release the package:

```bash
yarn version [<newversion> | major | minor | patch | premajor | preminor | prepatch | prerelease [--preid=<prerelease-id>] | from-git]

git push
```

GitHub actions will do the rest job for you.

> WARN: Don't run `npm publish` manually.

## License

MIT
