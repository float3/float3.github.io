"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.volumeValue = exports.tranposeValue = exports.output = exports.volumeSlider = exports.tuningSelect = exports.playMarked = exports.shareMarked = exports.playButton = exports.markedButtons = exports.soundMethod = void 0;
exports.play = play;
exports.DOMContentLoaded = DOMContentLoaded;
exports.handleTuningSelectChange = handleTuningSelectChange;
exports.playingTonesChanged = playingTonesChanged;
exports.logToDiv = logToDiv;
exports.keyActive = keyActive;
exports.markKey = markKey;
exports.unmarkKey = unmarkKey;
exports.markOrUnmarkKey = markOrUnmarkKey;
exports.addEvents = addEvents;
const tslib_1 = require("tslib");
const wasm = tslib_1.__importStar(require("wasm"));
const abcjs = tslib_1.__importStar(require("abcjs"));
const _1 = require(".");
const MIDI_1 = require("./MIDI");
const octaveSize = document.getElementById("octaveSize");
const stepSize = document.getElementById("stepSize");
const fileInput = document.getElementById("fileInput");
exports.soundMethod = document.getElementById("soundMethod");
const logContainer = document.getElementById("logContainer");
const stepSizeParent = stepSize.parentElement;
exports.markedButtons = document.getElementById("markedButtons");
exports.playButton = document.getElementById("playButton");
exports.shareMarked = document.getElementById("shareMarked");
exports.playMarked = document.getElementById("playMarked");
const stopButton = document.getElementById("stopButton");
exports.tuningSelect = document.getElementById("tuningSelect");
exports.volumeSlider = document.getElementById("volumeSlider");
const transpose = document.getElementById("transpose");
exports.output = document.getElementById("output");
octaveSize.onchange = handleTuningSelectChange;
exports.tuningSelect.onchange = handleTuningSelectChange;
stepSize.onchange = handleTuningSelectChange;
fileInput.onchange = fileInputChange;
transpose.onchange = transposeChange;
exports.volumeSlider.onchange = volumeChange;
stopButton.onclick = stop;
exports.playMarked.onclick = playMarkedKeys;
exports.shareMarked.onclick = sharedMarkedKeys;
exports.tranposeValue = 0;
function transposeChange() {
    exports.tranposeValue = parseInt(transpose.value);
}
exports.volumeValue = 0.25;
function volumeChange() {
    exports.volumeValue = parseFloat(exports.volumeSlider.value);
}
let midiFile;
let midiFilePromise = null;
function initOrGetMidiFile() {
    if (!midiFilePromise) {
        midiFilePromise = fetch("sample.mid")
            .then((response) => response.arrayBuffer())
            .then((buffer) => {
            midiFile = buffer;
            return midiFile;
        })
            .catch((error) => {
            console.error(error);
            throw error;
        });
    }
    return midiFilePromise;
}
function fileInputChange(event) {
    return new Promise((resolve, reject) => {
        const files = event.target.files;
        if (files && files.length > 0) {
            const reader = new FileReader();
            reader.onload = (e) => {
                midiFile = e.target.result;
                midiFilePromise = Promise.resolve(midiFile);
                resolve();
            };
            reader.onerror = reject;
            reader.readAsArrayBuffer(files[0]);
        }
        else {
            reject(new Error("No file selected"));
        }
    });
}
function playMarkedKeys() {
    _1.markedKeys.forEach((note) => (0, _1._noteOn)(note, undefined, true));
    playingTonesChanged;
}
function sharedMarkedKeys() {
    createAndCopyUrl(_1.markedKeys)();
}
function stop() {
    (0, MIDI_1.stopMIDIFile)();
}
function play() {
    initOrGetMidiFile().then(MIDI_1.playMIDIFile).catch(console.error);
}
function DOMContentLoaded() {
    handleTuningSelectChange;
}
function handleTuningSelectChange() {
    switch (exports.tuningSelect.value) {
        case "StepMethod":
            stepSizeParent.hidden = false;
            stepSize.readOnly = false;
            octaveSize.readOnly = false;
            break;
        case "EqualTemperament":
            stepSizeParent.hidden = true;
            stepSize.readOnly = true;
            octaveSize.readOnly = false;
            break;
        default:
            wasm.set_tuning_system(exports.tuningSelect.value, parseInt(octaveSize.value), parseInt(stepSize.value));
            octaveSize.value = wasm.get_tuning_size().toString();
            octaveSize.readOnly = true;
            stepSize.hidden = true;
            stepSize.readOnly = true;
            break;
    }
    (0, _1.stopAllTones)();
}
function adjustOutputSize() {
    exports.output.style.width = "300px";
    exports.output.style.height = "200px";
}
function playingTonesChanged() {
    const notes = Object.keys(_1.playingTones).map(Number);
    if (notes.length === 0) {
        abcjs.renderAbc("output", 'X: 1\nL: 1/1\n|""[u]|');
        adjustOutputSize();
        return;
    }
    let chordName;
    const tones = Object.values(_1.playingTones)
        .map((tone) => tone.name)
        .join(" ");
    if (octaveSize.value === "12") {
        const formatted_notes = wasm.convert_notes(tones.split(" "));
        chordName = wasm.get_chord_name();
        abcjs.renderAbc("output", formatted_notes);
        adjustOutputSize();
    }
    logToDiv(`${tones} | ${chordName}`, notes);
}
function createAndCopyUrl(keys) {
    const hash = generateHash(keys);
    const url = `${window.location.origin + window.location.pathname}#${hash}`;
    return function () {
        navigator.clipboard.writeText(url).catch(console.error);
    };
}
function generateHash(keys) {
    const hash = keys.join(",");
    return hash;
}
function logToDiv(message, notes) {
    const p = document.createElement("p");
    p.textContent = message;
    const shareButton = document.createElement("button");
    shareButton.textContent = "Share";
    shareButton.onclick = createAndCopyUrl(notes);
    shareButton.style.marginRight = "10px";
    p.style.marginLeft = "10px";
    const div = document.createElement("div");
    div.style.display = "flex";
    div.style.justifyContent = "left";
    div.style.alignItems = "center";
    div.appendChild(shareButton);
    div.appendChild(p);
    logContainer.insertBefore(div, logContainer.firstChild);
}
function keyActive(tone_index, active) {
    const keyElement = document.querySelector(`div[data-note="${tone_index}"]`);
    if (keyElement) {
        if (active)
            keyElement.classList.add("key-active");
        else
            keyElement.classList.remove("key-active");
    }
}
function markKey(tone_index) {
    if (_1.markedKeys.includes(tone_index))
        return;
    _1.markedKeys.push(tone_index);
    const keyElement = document.querySelector(`div[data-note="${tone_index}"]`);
    if (keyElement) {
        keyElement.classList.add("key-marked");
    }
    exports.markedButtons.style.display = "block";
}
function unmarkKey(tone_index) {
    const index = _1.markedKeys.indexOf(tone_index);
    if (index > -1) {
        _1.markedKeys.splice(index, 1);
    }
    const keyElement = document.querySelector(`div[data-note="${tone_index}"]`);
    if (keyElement) {
        keyElement.classList.remove("key-marked");
    }
    if (_1.markedKeys.length === 0) {
        exports.markedButtons.style.display = "none";
    }
}
function markOrUnmarkKey(tone_index) {
    const index = _1.markedKeys.indexOf(tone_index);
    if (index > -1) {
        unmarkKey(tone_index);
    }
    else {
        markKey(tone_index);
    }
    _1.markedKeys.sort((a, b) => a - b);
    window.location.hash = generateHash(_1.markedKeys);
}
function addEvents(key) {
    const note = parseInt(key.getAttribute("data-note"));
    const addEvent = (eventName, callback) => {
        key.addEventListener(eventName, callback);
    };
    key.addEventListener("mousedown", (event) => {
        const mouseEvent = event;
        if (mouseEvent.ctrlKey) {
            markOrUnmarkKey(note);
        }
        else {
            (0, _1.noteOn)(note);
        }
    });
    addEvent("mouseup", () => (0, _1.noteOff)(note));
    key.addEventListener("mouseenter", (event) => {
        const mouseEvent = event;
        if (mouseEvent.ctrlKey) {
            return;
        }
        (0, _1.noteOn)(note);
    });
    addEvent("mouseleave", () => (0, _1.noteOff)(note));
    addEvent("touchstart", () => (0, _1.noteOn)(note));
    addEvent("touchend", () => (0, _1.noteOff)(note));
}
