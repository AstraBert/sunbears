# `sunbears`

A CSV data loader for TypeScript with an API similar to Polars and Pandas, written in pure Rust.

## Installation

Install the package with your favorite package manager:

```bash
npm install @cle-does-things/sunbears
```

## Usage

The main function for `sunbears` is `readCsv`, which loads the data contained in a CSV file as a `DataFrame`, a columnar data format.

```typescript
import { readCsv } from '@cle-does-things/sunbears'

const df = readCsv('test.csv')
```

The `DataFrame` class exposes two methods:

- `colDtype`: retrieve the data type of the records contained within a column (integer, float, boolean or string)
- `get`: get a column

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

## Benchmarking

`sunbears` was benchmarked using the `tinybench`-based script you can find [here](./benchmark/bench.ts). The script reports latency statistics related to the `readCsv` function reading increasingly large CSV files (100, 1000, 100.000 and 1.000.000 rows).

The latest benchmark run was:

| Task                     | Latency avg (s)  | Latency med (s)      | Throughput avg (ops/s) | Throughput med (ops/s) | Samples |
| ------------------------ | ---------------- | -------------------- | ---------------------- | ---------------------- | ------- |
| Read a 100-lines CSV     | 0.000073 ± 0.20% | 0.000073 ± 0.0000015 | 13814 ± 0.11%          | 13841 ± 285            | 13727   |
| Read a 1000-lines CSV    | 0.000423 ± 0.47% | 0.000412 ± 0.000010  | 2383 ± 0.31%           | 2427 ± 60              | 2362    |
| Read a 100000-lines CSV  | 0.041591 ± 2.07% | 0.040895 ± 0.001126  | 24 ± 1.86%             | 24 ± 1                 | 64      |
| Read a 1000000-lines CSV | 0.424404 ± 0.73% | 0.421294 ± 0.008276  | 2 ± 0.70%              | 2 ± 0                  | 64      |

Here is how the tool compares to the `read_csv` function in Pandas and Polars ([script](./testfiles/comparative_bench.py)):

| Dataset       | Pandas (s) | Polars (s) |
| ------------- | ---------- | ---------- |
| 100 lines     | 0.030637   | 0.022748   |
| 1000 lines    | 0.033386   | 0.017368   |
| 100000 lines  | 0.453596   | 0.027075   |
| 1000000 lines | 3.986837   | 0.198708   |

And here it how it compares with `csv-parse` ([script](./benchmark/bench-alt.ts)):

| Task                       | Latency avg (s)  | Latency med (s)     | Throughput avg (ops/s) | Throughput med (ops/s) | Samples |
| -------------------------- | ---------------- | ------------------- | ---------------------- | ---------------------- | ------- |
| Read a 100-lines CSV       | 0.000188 ± 0.98% | 0.000179 ± 0.000003 | 5,442 ± 0.23%          | 5,580 ± 94             | 5,312   |
| Read a 1,000-lines CSV     | 0.001235 ± 0.69% | 0.001189 ± 0.000032 | 816 ± 0.58%            | 841 ± 23               | 810     |
| Read a 100,000-lines CSV   | 0.11656 ± 0.26%  | 0.11648 ± 0.000642  | 9 ± 0.25%              | 9 ± 0                  | 64      |
| Read a 1,000,000-lines CSV | 1.19114 ± 0.37%  | 1.18707 ± 0.007191  | 1 ± 0.36%              | 1 ± 0                  | 64      |

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
