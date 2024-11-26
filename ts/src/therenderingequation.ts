document.addEventListener("nav", () => {
    func();
});

function func(): void {
    const elements: NodeListOf<SVGPathElement | SVGRectElement> = document.querySelectorAll("#interactiveSvg path, #interactiveSvg rect");
    const groupMap: Record<string, (SVGPathElement | SVGRectElement)[]> = {};

    elements.forEach((element) => {
        const fillColor = window.getComputedStyle(element).fill;

        if (!groupMap[fillColor]) {
            groupMap[fillColor] = [];
        }

        groupMap[fillColor].push(element);

        if (fillColor === "rgb(123, 233, 255)" || fillColor === "#7BE9FF") {
            return;
        }

        const frequency = Math.random() * 3 + 2; // 2-5 seconds duration
        const amplitude = Math.random() * 5 + 8; // 5-10 pixels vertical movement

        element.style.animation = `moveUpDown ${frequency}s ease-in-out infinite alternate`;
        element.style.setProperty("--amplitude", `${amplitude}px`);
    });

    elements.forEach((element) => {
        const fillColor = window.getComputedStyle(element).fill;

        element.addEventListener("mouseenter", () => {
            groupMap[fillColor]?.forEach((el) => el.classList.add("hovered"));
        });

        element.addEventListener("mouseleave", () => {
            groupMap[fillColor]?.forEach((el) => el.classList.remove("hovered"));
        });
    });
}
