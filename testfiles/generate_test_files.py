#!/usr/bin/env -S uv run --script
#
# /// script
# requires-python = ">=3.12"
# dependencies = ["polars"]
# ///

import random

import polars as pl

NAMES = ["Anne", "Bob", "Charles", "Dylan", "Enid", "Francis"]
PHONES = ["+1 1234", "0043-1234"]


def generate_data(n: int) -> None:
    fields = {
        "name": [],
        "age": [],
        "is_working": [],
        "height": [],
        "phone_number": [],
        "years_of_experience": [],
    }
    for _ in range(n):
        name = random.choice(NAMES)
        age = random.randint(20, 50)
        is_working = random.randint(0, 1) == 1
        height = random.random() + random.randint(150, 180)
        phone_number = random.choice(PHONES)
        yeo = random.randint(0, 3) + random.random() * 2
        fields["name"].append(name)
        fields["age"].append(age)
        fields["height"].append(height)
        fields["phone_number"].append(phone_number)
        fields["years_of_experience"].append(yeo)
        fields["is_working"].append(is_working)
    df = pl.DataFrame(fields, strict=False)
    df.write_csv(f"testfiles/generated-{n}.csv")


def main() -> None:
    sizes = [100, 1000, 100_000, 1_000_000]
    for s in sizes:
        print(f"Generating data for testfiles/generated-{s}.csv")
        generate_data(s)


if __name__ == "__main__":
    main()
