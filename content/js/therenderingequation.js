"use strict";
document.addEventListener("nav", () => {
    func();
    console.log("nav event");
});
console.log("therenderingequation.ts");
function func() {
    const elements = document.querySelectorAll("#interactiveSvg path, #interactiveSvg rect");
    const groupMap = {};
    elements.forEach((element) => {
        const fillColor = window.getComputedStyle(element).fill;
        if (!groupMap[fillColor]) {
            groupMap[fillColor] = [];
        }
        groupMap[fillColor].push(element);
        if (fillColor === "rgb(123, 233, 255)" || fillColor === "#7BE9FF") {
            return;
        }
        const frequency = Math.random() * 3 + 2;
        const amplitude = Math.random() * 5 + 8;
        element.style.animation = `moveUpDown ${frequency}s ease-in-out infinite alternate`;
        element.style.setProperty("--amplitude", `${amplitude}px`);
    });
    elements.forEach((element) => {
        const fillColor = window.getComputedStyle(element).fill;
        element.addEventListener("mouseenter", () => {
            var _a;
            (_a = groupMap[fillColor]) === null || _a === void 0 ? void 0 : _a.forEach((el) => el.classList.add("hovered"));
        });
        element.addEventListener("mouseleave", () => {
            var _a;
            (_a = groupMap[fillColor]) === null || _a === void 0 ? void 0 : _a.forEach((el) => el.classList.remove("hovered"));
        });
    });
}
