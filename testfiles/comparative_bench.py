#!/usr/bin/env -S uv run --script
#
# /// script
# requires-python = ">=3.12"
# dependencies = ["polars", "pandas"]
# ///


import os
import timeit
from glob import glob

import pandas as pd
import polars as pl


def generate_df_with_n_rows(n: int, pandas: bool) -> pl.DataFrame | pd.DataFrame:
    data = {
        "col1": ["something"] * n,
        "col2": [1.0] * n,
        "col3": [3] * n,
        "col4": [True] * n,
    }
    if pandas:
        return pd.DataFrame(data)
    return pl.DataFrame(data)


pl_100_df = generate_df_with_n_rows(100, False)
pl_1000_df = generate_df_with_n_rows(1000, False)
pl_100000_df = generate_df_with_n_rows(100000, False)
pl_1000000_df = generate_df_with_n_rows(1000000, False)
pd_100_df = generate_df_with_n_rows(100, True)
pd_1000_df = generate_df_with_n_rows(1000, True)
pd_100000_df = generate_df_with_n_rows(100000, True)
pd_1000000_df = generate_df_with_n_rows(1000000, True)


def read_100_with_polars() -> None:
    """
    Read a 100-rows CSV with Polars.
    """

    pl.read_csv("testfiles/generated-100.csv")


def read_1000_with_polars() -> None:
    """
    Read a 1000-rows CSV with Polars.
    """
    pl.read_csv("testfiles/generated-1000.csv")


def read_100000_with_polars() -> None:
    """
    Read a 100.000-rows CSV with Polars.
    """
    pl.read_csv("testfiles/generated-100000.csv")


def read_1000000_with_polars() -> None:
    """
    Read a 1.000.000-rows CSV with Polars.
    """
    pl.read_csv("testfiles/generated-1000000.csv")


def read_100_with_pandas() -> None:
    """
    Read a 100-rows CSV with Pandas.
    """
    pd.read_csv("testfiles/generated-100.csv")


def read_1000_with_pandas() -> None:
    """
    Read a 1000-rows CSV with Pandas.
    """
    pd.read_csv("testfiles/generated-1000.csv")


def read_100000_with_pandas() -> None:
    """
    Read a 100.000-rows CSV with Pandas.
    """
    pd.read_csv("testfiles/generated-100000.csv")


def read_1000000_with_pandas() -> None:
    """
    Read a 1.000.000-rows CSV with Pandas.
    """
    pd.read_csv("testfiles/generated-1000000.csv")


def write_100_with_pandas() -> None:
    pd_100_df.to_csv("testfiles/pd-100.csv", index=False)


def write_1000_with_pandas() -> None:
    pd_1000_df.to_csv("testfiles/pd-1000.csv", index=False)


def write_100000_with_pandas() -> None:
    pd_100000_df.to_csv("testfiles/pd-100000.csv", index=False)


def write_1000000_with_pandas() -> None:
    pd_1000000_df.to_csv("testfiles/pd-1000000.csv", index=False)


def write_100_with_polars() -> None:
    pl_100_df.write_csv("testfiles/pl-100.csv")


def write_1000_with_polars() -> None:
    pl_1000_df.write_csv("testfiles/pl-1000.csv")


def write_100000_with_polars() -> None:
    pl_100000_df.write_csv("testfiles/pl-100000.csv")


def write_1000000_with_polars() -> None:
    pl_1000000_df.write_csv("testfiles/pl-1000000.csv")


def cleanup() -> None:
    files = glob("testfiles/p?-1*.csv")
    for file in files:
        os.remove(file)


def main() -> None:
    t100_pd = timeit.timeit(read_100_with_pandas, number=200)
    t1000_pd = timeit.timeit(read_1000_with_pandas, number=100)
    t100000_pd = timeit.timeit(read_100000_with_pandas, number=20)
    t1000000_pd = timeit.timeit(read_1000000_with_pandas, number=20)
    t100_pl = timeit.timeit(read_100_with_polars, number=200)
    t1000_pl = timeit.timeit(read_1000_with_polars, number=100)
    t100000_pl = timeit.timeit(read_100000_with_polars, number=20)
    t1000000_pl = timeit.timeit(read_1000000_with_polars, number=20)

    wt100_pd = timeit.timeit(write_100_with_pandas, number=200)
    wt1000_pd = timeit.timeit(write_1000_with_pandas, number=100)
    wt100000_pd = timeit.timeit(write_100000_with_pandas, number=20)
    wt1000000_pd = timeit.timeit(write_1000000_with_pandas, number=20)
    wt100_pl = timeit.timeit(write_100_with_polars, number=200)
    wt1000_pl = timeit.timeit(write_1000_with_polars, number=100)
    wt100000_pl = timeit.timeit(write_100000_with_polars, number=20)
    wt1000000_pl = timeit.timeit(write_1000000_with_polars, number=20)

    rows = [
        ("Read 100 lines", t100_pd, t100_pl),
        ("Read 1000 lines", t1000_pd, t1000_pl),
        ("Read 100000 lines", t100000_pd, t100000_pl),
        ("Read 1000000 lines", t1000000_pd, t1000000_pl),
        ("Write 100 lines", wt100_pd, wt100_pl),
        ("Write 1000 lines", wt1000_pd, wt1000_pl),
        ("Write 100000 lines", wt100000_pd, wt100000_pl),
        ("Write 1000000 lines", wt1000000_pd, wt1000000_pl),
    ]

    col0, col1, col2 = "Dataset", "Pandas (s)", "Polars (s)"
    w0 = max(len(col0), max(len(r[0]) for r in rows))
    w1 = max(len(col1), 10)
    w2 = max(len(col2), 10)

    sep = f"+{'-' * (w0 + 2)}+{'-' * (w1 + 2)}+{'-' * (w2 + 2)}+"
    header = f"| {col0:<{w0}} | {col1:<{w1}} | {col2:<{w2}} |"

    print(sep)
    print(header)
    print(sep)
    for name, pd_t, pl_t in rows:
        print(f"| {name:<{w0}} | {pd_t:<{w1}.6f} | {pl_t:<{w2}.6f} |")
    print(sep)

    cleanup()


if __name__ == "__main__":
    main()
