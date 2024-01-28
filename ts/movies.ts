function getRandomMovie(id: string): void {
    const heading = document.getElementById(id) as HTMLElement;
    const headingContent: string = heading.innerText;
    const moviesToWatch: string[] = [];

    const movieListPattern: RegExp = /^- \[ \] (.+)$/gm;
    let match;

    while ((match = movieListPattern.exec(headingContent)) !== null) {
        moviesToWatch.push(match[1].trim());
    }

    const url: string = "https://tools-unite.com/tools/random-picker-wheel?inputs=" +
        moviesToWatch.map(movie => `:1,${encodeURIComponent(movie)}`).join("");

    window.open(url, '_blank');
}

document.addEventListener("DOMContentLoaded", () => {
    let headings: NodeListOf<HTMLHeadingElement> = document.querySelectorAll('h1');

    headings.forEach(heading => {
        const headingText: string = heading.innerText.trim();
        if (heading.id !== "index" && heading.id !== "dropped-movies") {
            const button = document.createElement("button");
            button.textContent = "I'm feelin lucky";
            button.addEventListener("click", () => {
                getRandomMovie(heading.id);
            });
            heading.insertAdjacentElement('afterend', button);
        }
    })
});