"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.markedKeys = exports.heldKeys = exports.playingTones = void 0;
exports.stopAllTones = stopAllTones;
exports.noteOn = noteOn;
exports._noteOn = _noteOn;
exports.noteOff = noteOff;
const tslib_1 = require("tslib");
const wasm = tslib_1.__importStar(require("wasm"));
const Tone_1 = require("./Tone");
const MIDI_1 = require("./MIDI");
const events_1 = require("./events");
const UI_1 = require("./UI");
document.addEventListener("DOMContentLoaded", UI_1.DOMContentLoaded);
document.addEventListener("visibilitychange", events_1.visibilityChange);
window.addEventListener("blur", stopAllTones);
window.addEventListener("hashchange", events_1.onload);
window.createTone = Tone_1.createTone;
wasm
    .default()
    .then(() => {
    (0, MIDI_1.requestMIDI)();
    UI_1.playButton.onclick = UI_1.play;
    document.addEventListener("keydown", events_1.keydown);
    document.addEventListener("keyup", events_1.keyup);
    document.querySelectorAll(".white-key, .black-key").forEach((key) => {
        (0, UI_1.addEvents)(key);
    });
    (0, events_1.onload)();
    (0, UI_1.playingTonesChanged)();
})
    .catch(console.error);
exports.playingTones = [];
exports.heldKeys = {};
exports.markedKeys = [];
function stopAllTones() {
    Object.keys(exports.playingTones).forEach((key) => {
        const tone_index = parseInt(key);
        exports.playingTones[tone_index].node.stop();
        delete exports.playingTones[tone_index];
        (0, UI_1.keyActive)(tone_index, false);
    });
    (0, UI_1.playingTonesChanged)();
}
function noteOn(tone_index, velocity, cancel) {
    _noteOn(tone_index, velocity, cancel);
    (0, UI_1.playingTonesChanged)();
}
function _noteOn(tone_index, velocity, cancel) {
    tone_index += UI_1.tranposeValue;
    const tone = wasm.get_tone(tone_index);
    const volume = Math.pow(UI_1.volumeValue, 2);
    switch (UI_1.soundMethod.value) {
        case "native":
            playFrequencyNative(tone, volume).catch(console.error);
            break;
        case "sample":
            playFrequencySample(tone, volume, cancel).catch(console.error);
            break;
    }
    (0, UI_1.keyActive)(tone_index, true);
}
function noteOff(tone_index) {
    tone_index += UI_1.tranposeValue;
    if (!(tone_index in exports.playingTones))
        return;
    switch (UI_1.soundMethod.value) {
        case "native":
            exports.playingTones[tone_index].node.stop();
            break;
        case "sample":
            break;
    }
    delete exports.playingTones[tone_index];
    (0, UI_1.playingTonesChanged)();
    (0, UI_1.keyActive)(tone_index, false);
}
let audioContext = null;
function initOrGetAudioContext() {
    return new Promise((resolve, reject) => {
        try {
            if (!audioContext) {
                audioContext = new window.AudioContext();
            }
            resolve(audioContext);
        }
        catch (error) {
            reject(error);
        }
    });
}
let audioBuffer = null;
function initOrGetAudioBuffer() {
    if (!audioBuffer) {
        return fetch("a1.wav")
            .then((response) => response.arrayBuffer())
            .then((arrayBuffer) => initOrGetAudioContext().then((context) => context.decodeAudioData(arrayBuffer)))
            .then((newAudioBuffer) => {
            audioBuffer = newAudioBuffer;
            return audioBuffer;
        });
    }
    else {
        return Promise.resolve(audioBuffer);
    }
}
async function playFrequencySample(tone, volume, cancel) {
    const localAudioContext = await initOrGetAudioContext();
    const source = localAudioContext.createBufferSource();
    source.buffer = await initOrGetAudioBuffer();
    const gainNode = localAudioContext.createGain();
    gainNode.gain.value = volume;
    source.connect(gainNode);
    gainNode.connect(localAudioContext.destination);
    source.playbackRate.value = tone.freq / 220;
    source.start();
    tone.node = source;
    exports.playingTones[tone.index] = tone;
    (0, UI_1.playingTonesChanged)();
    if (cancel) {
        source.onended = () => {
            noteOff(tone.index);
        };
    }
}
async function playFrequencyNative(tone, volume) {
    const localAudioContext = await initOrGetAudioContext();
    const oscillator = localAudioContext.createOscillator();
    const gainNode = localAudioContext.createGain();
    gainNode.gain.value = volume;
    gainNode.connect(localAudioContext.destination);
    oscillator.type = "square";
    oscillator.frequency.setValueAtTime(tone.freq, localAudioContext.currentTime);
    oscillator.connect(gainNode);
    oscillator.start();
    tone.node = oscillator;
    if (tone.index in exports.playingTones)
        exports.playingTones[tone.index].node.stop();
    exports.playingTones[tone.index] = tone;
    (0, UI_1.playingTonesChanged)();
}
