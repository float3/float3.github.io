import webbrowser

def main():

    with open("content/movies.md", "r", encoding="utf-8") as file:
        lines = file.readlines()

    start_index = lines.index("# Red Mage:\n") + 1
    end_index = len(lines) if "# " in lines[start_index:] else start_index + lines[start_index:].index("\n")

    movies_to_watch = [line.strip()[6:].strip() for line in lines[start_index:end_index] if line.startswith("- [ ]") and "- [ ] BREAK" not in line]


    url = "https://tools-unite.com/tools/random-picker-wheel?inputs="

    for movie in movies_to_watch:
        url += ":1," + movie

    webbrowser.open(url)

if __name__ == "__main__":
    main()


