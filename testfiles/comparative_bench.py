#!/usr/bin/env -S uv run --script
#
# /// script
# requires-python = ">=3.12"
# dependencies = ["polars", "pandas"]
# ///


import timeit

import pandas as pd
import polars as pl


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


def main() -> None:
    t100_pd = timeit.timeit(read_100_with_pandas, number=200)
    t1000_pd = timeit.timeit(read_1000_with_pandas, number=100)
    t100000_pd = timeit.timeit(read_100000_with_pandas, number=20)
    t1000000_pd = timeit.timeit(read_1000000_with_pandas, number=20)
    t100_pl = timeit.timeit(read_100_with_polars, number=200)
    t1000_pl = timeit.timeit(read_1000_with_polars, number=100)
    t100000_pl = timeit.timeit(read_100000_with_polars, number=20)
    t1000000_pl = timeit.timeit(read_1000000_with_polars, number=20)

    rows = [
        ("100 lines", t100_pd, t100_pl),
        ("1000 lines", t1000_pd, t1000_pl),
        ("100000 lines", t100000_pd, t100000_pl),
        ("1000000 lines", t1000000_pd, t1000000_pl),
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


if __name__ == "__main__":
    main()
