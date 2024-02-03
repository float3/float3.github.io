function getRandomMovie(id: string): void {
    const heading = document.getElementById(id) as HTMLElement;
    const headingContent: string = (heading.nextElementSibling?.nextElementSibling as HTMLElement).innerText;

    let moviesToWatch = headingContent.split("\n").filter(x => !x.includes("BREAK")).filter(x => !/\(\d{4}-\d{2}-\d{2}\)$/.test(x));

    const url: string = "https://tools-unite.com/tools/random-picker-wheel?inputs=" +
        moviesToWatch.map(movie => `${encodeURIComponent(movie)}:1,`).join("");

    window.open(url, '_blank');
}

document.addEventListener("DOMContentLoaded", () => {
    let headings: NodeListOf<HTMLHeadingElement> = document.querySelectorAll('h1');

    headings.forEach(heading => {
        const headingText: string = heading.innerText.trim();
        if (heading.innerHTML != "movies" && heading.id !== "index" && heading.id !== "dropped-movies") {
            const button = document.createElement("button");
            button.textContent = "I'm feelin lucky";
            button.addEventListener("click", () => {
                getRandomMovie(heading.id);
            });
            heading.insertAdjacentElement('afterend', button);
        }
    })
});