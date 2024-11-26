"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.visibilityChange = visibilityChange;
exports.onload = onload;
exports.keydown = keydown;
exports.keyup = keyup;
const tslib_1 = require("tslib");
const wasm = tslib_1.__importStar(require("wasm"));
const _1 = require(".");
const _2 = require(".");
const UI_1 = require("./UI");
function visibilityChange() {
    if (document.hidden) {
        (0, _2.stopAllTones)();
    }
}
function onload() {
    const hash = window.location.hash.substring(1);
    if (hash) {
        const notes = hash.split(",");
        UI_1.markedButtons.style.display = "flex";
        notes.forEach((note) => {
            const index = parseInt(note);
            (0, UI_1.markKey)(index);
        });
    }
    else {
        UI_1.markedButtons.style.display = "none";
    }
}
function keydown(event) {
    if (!document.hasFocus())
        return;
    if (event.repeat)
        return;
    if (event.code in _1.heldKeys)
        return;
    if (document.activeElement?.tagName === "BODY") {
        const tone_index = wasm.from_keymap(event.code);
        if (tone_index === -1)
            return;
        (0, _1.noteOn)(tone_index);
        _1.heldKeys[event.code] = true;
    }
}
function keyup(event) {
    const tone_index = wasm.from_keymap(event.code);
    if (tone_index === -1)
        return;
    (0, _1.noteOff)(tone_index);
    delete _1.heldKeys[event.code];
}
