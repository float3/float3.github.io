"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Tone = void 0;
exports.createTone = createTone;
class Tone {
    index;
    freq;
    cents;
    name;
    node;
    constructor(index, freq, cents, name, node) {
        this.index = index;
        this.freq = freq;
        this.cents = cents;
        this.name = name;
        this.node = node;
    }
}
exports.Tone = Tone;
function createTone(index, freq, cents, name, oscillator) {
    return new Tone(index, freq, cents, name, oscillator);
}
