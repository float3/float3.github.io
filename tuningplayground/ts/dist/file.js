"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.downloadFile = void 0;
let textFile = null;
const makeTextFile = (text) => {
    const data = new Blob([text], { type: "text/plain" });
    if (textFile !== null) {
        window.URL.revokeObjectURL(textFile);
    }
    textFile = window.URL.createObjectURL(data);
    return textFile;
};
const downloadFile = (name, contents) => {
    const a = document.createElement("a");
    a.style.display = "none";
    a.href = makeTextFile(contents);
    a.download = name;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
};
exports.downloadFile = downloadFile;
