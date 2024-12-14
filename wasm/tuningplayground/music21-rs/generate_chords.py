#!/usr/bin/env python3
import itertools
import json
import sys

if __name__ == "__main__":

    sys.path.append("./music21")
    from music21 import chord

    data = {}
    for r in range(1, 13):
        combinations = itertools.combinations(range(12), r)
        for combination in combinations:
            c = chord.Chord(combination).pitchedCommonName
            bitmask = sum(1 << i for i in combination)
            data[bitmask] = c

    # sort the dictionary by key
    data = dict(sorted(data.items()))

    with open("../../content/misc/plaintext/chords.txt", "w") as outfile:
        for key, value in data.items():
            outfile.write(f"{value};")

    with open("../../content/misc/plaintext/chords.json", "w") as outfile:
        json.dump(data, outfile, separators=(",", ":"))
