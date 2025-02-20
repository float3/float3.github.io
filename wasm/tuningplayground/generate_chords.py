#!/usr/bin/env python3
import itertools
import json
import sys

if __name__ == "__main__":

    sys.path.append("./music21")
    from music21 import chord

    data = {}
    data2 = {}
    data3 = {}

    for r in range(1, 13):
        combinations = itertools.combinations(range(12), r)
        for combination in combinations:
            c = chord.Chord(combination)
            bitmask = sum(1 << i for i in combination)
            data[bitmask] = c.pitchedCommonName
            data2[bitmask] = c.commonName
            data3[bitmask] = c.fullName

    # sort the dictionary by key
    data = dict(sorted(data.items()))
    data2 = dict(sorted(data2.items()))
    data3 = dict(sorted(data3.items()))

    with open("../../content/misc/plaintext/chords.txt", "w") as outfile:
        for key, value in data.items():
            outfile.write(f"{value};")

    with open("../../content/misc/plaintext/chords.json", "w") as outfile:
        json.dump(data, outfile, separators=(",", ":"))

    with open("../../content/misc/plaintext/chords_unpitched.txt", "w") as outfile:
        for key, value in data2.items():
            outfile.write(f"{value};")

    with open("../../content/misc/plaintext/chords_unpitched.json", "w") as outfile:
        json.dump(data2, outfile, separators=(",", ":"))
    
    with open("../../content/misc/plaintext/chords_unpitched.txt", "w") as outfile:
        for key, value in data3.items():
            outfile.write(f"{value};")

    with open("../../content/misc/plaintext/chords_unpitched.json", "w") as outfile:
        json.dump(data3, outfile, separators=(",", ":"))
