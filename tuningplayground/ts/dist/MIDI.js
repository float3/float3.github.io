"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.requestMIDI = requestMIDI;
exports.stopMIDIFile = stopMIDIFile;
exports.playMIDIFile = playMIDIFile;
const _1 = require(".");
const midi_1 = require("@tonejs/midi");
const config_1 = require("./config");
function requestMIDI() {
    if (navigator.requestMIDIAccess) {
        navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);
    }
    else {
        alert("WebMIDI is not supported in this browser.");
    }
}
function onMIDISuccess(midiAccess) {
    const input = midiAccess.inputs.values().next().value;
    if (input) {
        input.onmidimessage = onMIDIMessage;
    }
    else {
        alert("No MIDI input devices found.");
    }
}
function onMIDIFailure(error) {
    console.error("MIDI Access failed:", error);
}
function onMIDIMessage(event) {
    const [status, tone_index, velocity] = event.data;
    const is_note_on = (status & 240) === 144;
    const is_note_off = (status & 240) === 128;
    if (is_note_off) {
        (0, _1.noteOff)(tone_index);
    }
    if (is_note_on) {
        (0, _1.noteOn)(tone_index, velocity);
    }
}
let timeoutIds = [];
function stopMIDIFile() {
    timeoutIds.forEach((id) => clearTimeout(id));
    timeoutIds = [];
}
function playMIDIFile(midiFile) {
    const midi = new midi_1.Midi(midiFile);
    midi.tracks.forEach((track) => {
        const startTime = 0;
        track.notes.forEach((note) => {
            const noteOnTime = note.time * config_1.midiMultiplier - startTime;
            const noteOffTime = (note.time + note.duration) * config_1.midiMultiplier - startTime;
            const velocity = note.velocity;
            if (velocity === 1)
                note.velocity = 127;
            const midiNote = note.midi;
            timeoutIds.push(setTimeout(() => (0, _1.noteOn)(midiNote, velocity), noteOnTime));
            timeoutIds.push(setTimeout(() => (0, _1.noteOff)(midiNote), noteOffTime));
        });
    });
}
