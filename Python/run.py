import argparse
from datetime import datetime

from aocd.utils import AOC_TZ

from Python import solve, solveAndSubmit


def main():
    aoc_now = datetime.now(tz=AOC_TZ)
    years = range(2015, aoc_now.year + int(aoc_now.month >= 11))
    days = range(1, 26)
    parser = argparse.ArgumentParser(description="run current day")
    parser.add_argument(
        "level",
        nargs="?",
        choices=("a", "b", "ab", "none"),
        default="a",
        help="Which part of the puzzle to solve (default: %(default)s)",
    )
    parser.add_argument(
        "day",
        nargs="?",
        type=int,
        default=min(aoc_now.day, 25) if aoc_now.month == 12 else 1,
        help="1-25 (default: %(default)s)",
    )
    parser.add_argument(
        "year",
        nargs="?",
        type=int,
        default=years[-1],
        help="2015-%(default)s (default: %(default)s)",
    )
    args = parser.parse_args()
    if args.day in years and args.year in days:
        # be forgiving
        args.day, args.year = args.year, args.day
    if args.day not in days or args.year not in years:
        parser.print_usage()
        parser.exit(1)
    year = args.year
    day = args.day
    level = args.level

    if level == "none":
        solve(year, day)
    else:
        solveAndSubmit(year, day, level)


if __name__ == "__main__":
    main()
