"use strict";
function getRandomMovie(id) {
    let moviesToWatch = collectMovies(id);
    const url = "https://tools-unite.com/tools/random-picker-wheel?inputs=" + moviesToWatch.map(movie => `${encodeURIComponent(movie)}:1,`).join("");
    window.open(url, '_blank');
}
document.addEventListener("DOMContentLoaded", () => {
    let headings = document.querySelectorAll('h1');
    headings.forEach(heading => {
        const headingText = heading.innerText.trim();
        if (heading.innerHTML != "movies" && heading.id !== "index" && heading.id !== "dropped-movies") {
            const button = document.createElement("button");
            button.textContent = "I'm feelin lucky";
            button.addEventListener("click", () => {
                getRandomMovie(heading.id);
            });
            heading.insertAdjacentElement('afterend', button);
        }
    });
});
function collectMovies(id) {
    var _a;
    const heading = document.getElementById(id);
    const headingContent = ((_a = heading.nextElementSibling) === null || _a === void 0 ? void 0 : _a.nextElementSibling).innerText;
    let moviesToWatch = headingContent.split("\n").filter(x => !x.includes("BREAK")).filter(x => !/\(\d{4}-\d{2}-\d{2}\)$/.test(x));
    return moviesToWatch;
}
