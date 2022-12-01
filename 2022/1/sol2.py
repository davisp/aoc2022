#!/usr/bin/env python3

import sys


def find_max_cals(handle):
    max_cals = []
    cur_cals = 0
    for line in handle:
        if not line.strip():
            max_cals.append(cur_cals)
            if len(max_cals) > 3:
                max_cals.sort()
                max_cals.pop(0)
            cur_cals = 0
        else:
            cur_cals += int(line.strip())
    return sum(max_cals)


def main():
    if len(sys.argv) != 2:
        print("usage: %s filename" % sys.argv[0])

    with open(sys.argv[1]) as handle:
        print(find_max_cals(handle))


if __name__ == "__main__":
    main()
