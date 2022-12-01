#!/usr/bin/env python3

import sys


def find_max_cals(handle):
    max_cals = 0
    cur_cals = 0
    for line in handle:
        if not line.strip():
            max_cals = max(max_cals, cur_cals)
            cur_cals = 0
        else:
            cur_cals += int(line.strip())
    return max_cals


def main():
    if len(sys.argv) != 2:
        print("usage: %s filename" % sys.argv[0])

    with open(sys.argv[1]) as handle:
        print(find_max_cals(handle))


if __name__ == "__main__":
    main()
