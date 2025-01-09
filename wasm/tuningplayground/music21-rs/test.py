# test_profiler.py
import sys
import os

from call_tree_profiler import CallTreeProfiler

sys.path.append("./music21")
from music21 import chord

def main():
    project_root = os.path.abspath(os.path.dirname(__file__))
    profiler = CallTreeProfiler(project_root)
    sys.setprofile(profiler.profile)
    try:
        c = chord.Chord("C E G")
        print(f"Pitched Common Name: {c.pitchedCommonName}")
    finally:
        sys.setprofile(None)

    profiler.save_json('full_call_tree.json')
    profiler.save_tree('full_call_tree.txt')
    print("Full call tree has been saved to 'full_call_tree.json' and 'full_call_tree.txt'.")

if __name__ == "__main__":
    main()
