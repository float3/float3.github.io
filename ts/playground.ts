import * as Tone from "tone";

type FractionTable = Record<number, number>;
type ToneList = Array<[number, OscillatorNode]>;

var logContainer: HTMLElement;
var tuningSelect: HTMLSelectElement;
var baseFreq: HTMLInputElement;
var volumeSlider: HTMLInputElement;
var stepSize: HTMLSelectElement;
var stepSizeContainer: HTMLDivElement;
var equalTemperamentBase: HTMLInputElement;
var equalTemperamentBaseContainer: HTMLDivElement;

var synth: Tone.Synth<Tone.SynthOptions>;
var audioContext: AudioContext;
//var pianoSampler: Tone.Sampler;

var playingNotes: ToneList;

document.addEventListener("DOMContentLoaded", () => {
  if (navigator.requestMIDIAccess) {
    navigator.requestMIDIAccess().then(onMIDISuccess, onMIDIFailure);
  } else {
    alert("Web MIDI is working.");
  }

  logContainer = document.getElementById("logContainer") as HTMLElement;
  volumeSlider = document.getElementById("volumeSlider") as HTMLInputElement;
  baseFreq = document.getElementById("baseFreq") as HTMLInputElement;
  tuningSelect = document.getElementById("tuningSelect") as HTMLSelectElement;
  stepSize = document.getElementById("stepSize") as HTMLSelectElement;
  stepSizeContainer = document.getElementById("stepSizeContainer") as HTMLDivElement;
  equalTemperamentBase = document.getElementById("equalTemperamentBase") as HTMLInputElement;
  equalTemperamentBaseContainer = document.getElementById("equalTemperamentBaseContainer") as HTMLDivElement;

  synth = new Tone.Synth().toDestination();
  /*pianoSampler = new Tone.Sampler({
    urls: {
      C4: "path/to/your/piano/C4/sample.mp3",
    },
    baseUrl: "https://example.com/samples/",
    onload: () => console.log("Sample loaded"),
  }).toDestination();*/

  playingNotes = [];

  tuningSelectOnChange();
});

function onMIDISuccess(midiAccess: WebMidi.MIDIAccess) {
  const input = midiAccess.inputs.values().next().value;

  if (input) {
    input.onmidimessage = onMIDIMessage;
  } else {
    alert("No MIDI input devices found.");
  }
}

function onMIDIFailure(error: DOMException) {
  console.error("MIDI Access failed:", error);
}

function onMIDIMessage(event: WebMidi.MIDIMessageEvent) {
  const [status, note, velocity] = event.data;
  const isNoteOn = (status & 0xf0) === 0x90;
  const isNoteOff = (status & 0xf0) === 0x80;

  let n: number = note - 24;
  if (isNoteOn) {
    noteOn(n);
  }

  if (isNoteOff) {
    var newNotes: ToneList = [];
    playingNotes.forEach((note) => {
      check(note, n, newNotes);
    });
    playingNotes = newNotes;
  }
}

document.addEventListener("keydown", function (event) {
  if (event.code == "Tab") {
    logContainer.innerHTML = "";
    return;
  }

  let n: number = keyboard[event.code];

  if (playingNotes.some((note) => note[0] === n) || isNaN(n)) {
    return;
  }
  console.log(n);
  noteOn(n);
});

document.addEventListener("keyup", function (event) {
  var newNotes: ToneList = [];
  playingNotes.forEach((note) => {
    check(note, keyboard[event.code], newNotes);
  });
  playingNotes = newNotes;
});

function check(note: [number, OscillatorNode], n: number, newNotes: ToneList) {
  if (note[0] == n) {
    note[1].stop();
  } else {
    newNotes.push(note);
  }
}

function noteOn(n: number) {
  let ratio: number = getRatio(n);
  let root: number = parseFloat(baseFreq.value);
  let freq: number = ratio * root;
  logToDiv(freq + "Hz");

  let volume: number = Math.pow(parseFloat(volumeSlider.value), 2);

  playFrequency(freq, volume, n);
}

function getRatio(n: number): number {
  let ratio: number;
  switch (tuningSelect.value) {
    case "equal_temperament":
      ratio = getRatioFromEqualTemperament(n, parseFloat(equalTemperamentBase.value));
      break;
    case "step_method":
      ratio = getRatioFromStepAlgorithm(n, parseFloat(stepSize.value), 12);
      break;
    default:
      ratio = getRatioFromTable(n, table_table[tuningSelect.value]);
      break;
  }
  return ratio;
}

function playFrequency(frequency: number, volume: number, n: number): void {
  const soundMethod = document.getElementById("soundMethod") as HTMLSelectElement;
  switch (soundMethod.value) {
    default:
    case "native":
      playFrequencyNative(frequency, volume, n);
      break;
    case "tone.js":
      playFrequencyToneJS(frequency, volume);
      break;
  }
}

function playFrequencyToneJS(frequency: number, volume: number): void {
  //synth.volume.value = // TODO: make volume configurable
  synth.triggerAttackRelease(frequency, "8n");
}

// TODO: use piano sample instead of synth

function playFrequencyNative(
  frequency: number,
  volume: number,
  n: number
): void {
  audioContext = new window.AudioContext();
  const oscillator = audioContext.createOscillator();
  let gainNode = audioContext.createGain();
  gainNode.gain.value = volume;
  gainNode.connect(audioContext.destination);
  oscillator.type = "square"; // TODO: make this configurable
  oscillator.frequency.setValueAtTime(frequency, audioContext.currentTime);
  oscillator.connect(gainNode);
  oscillator.start();
  playingNotes.push([n, oscillator]);
}

function tuningSelectOnChange(): void {
  if (tuningSelect.value == "equal_temperament") {
    equalTemperamentBaseContainer.style.display = "block";
  } else {
    equalTemperamentBaseContainer.style.display = "none";
  }
  if (tuningSelect.value == "step_method") {
    stepSizeContainer.style.display = "block";
  } else {
    stepSizeContainer.style.display = "none";
  }
}

function logToDiv(message: any): void {
  logContainer.innerHTML = "<p>" + message + "</p>" + logContainer.innerHTML;
}

function getRatioFromEqualTemperament(n: number, base: number): number {
  return Math.pow(2, n / base);
}

function getRatioFromTable(n: number, table: FractionTable): number {
  let tablesize = Object.keys(table).length;
  let n2: number = n % tablesize;
  let ratio: number = table[n2];
  let octaves: number = Math.floor(n / tablesize);
  return ratio + octaves;
}

function getRatioFromStepAlgorithm(n: number, stepsize: number, max: number) {
  let ratio = getRatioFromTable(stepsize, just_intonation);
  let n2 = n % max;
  let current_ratio = 1;
  let current_idx = 0;
  while (current_idx !== n2) {
    current_ratio *= ratio;
    current_idx += stepsize;
    current_idx %= max;
    if (current_ratio > 2) {
      current_ratio /= 2;
    }
  }
  let octaves = Math.floor(n / max);
  return current_ratio + octaves;
}

function findCoprimes(num: number): number[] {
  const coprimes: number[] = [];

  for (let i = 2; i < num; i++) {
    if (gcd(num, i) === 1) {
      coprimes.push(i);
    }
  }

  return coprimes;
}

function gcd(a: number, b: number): number {
  if (b === 0) {
    return a;
  } else {
    return gcd(b, a % b);
  }
}

// TODO: unused tables: just_intonation_24, indian_scale, indian_scale_full, five_limit
// TODO: different equal temeperaments?
// TODO: arabic scales
// TODO: visualise keyboard
// TODO: japanese/chinese scales
// TODO: MIDI file playing
// TODO: calculate co primes for base size and let user choose one of them??
// TODO: tell user about VPMK
// TODO: add a record button to record and output midi
// TODO: have tuning system with just_intonation but derrive half of the ratios from the reverse ratio of the first one so the perfect fifth also provides the perfect fourth and the major third also provides minor 6th


const just_intonation: FractionTable = {
  0: 1 / 1,
  1: 17 / 16,
  2: 9 / 8,
  3: 19 / 16,
  4: 5 / 4,
  5: 4 / 3,
  6: 45 / 32,
  7: 3 / 2,
  8: 51 / 32,
  9: 27 / 16,
  10: 57 / 32,
  11: 15 / 8,
};

const just_intonation_24: FractionTable = {
  0: 1 / 1,
  1: 33 / 32,
  2: 17 / 16,
  3: 35 / 32,
  4: 9 / 8,
  5: 37 / 32,
  6: 19 / 16,
  7: 39 / 32,
  8: 5 / 4,
  9: 41 / 32,
  10: 4 / 3,
  11: 11 / 8,
  12: 45 / 32,
  13: 93 / 64,
  14: 3 / 2,
  15: 99 / 64,
  16: 51 / 32,
  17: 105 / 64,
  18: 27 / 16,
  19: 111 / 64,
  20: 57 / 32,
  21: 117 / 64,
  22: 15 / 8,
  23: 31 / 16,
};

const pythagorean_tuning: FractionTable = {
  0: 1 / 1,
  1: 256 / 243,
  2: 9 / 8,
  3: 32 / 27,
  4: 81 / 64,
  5: 4 / 3,
  6: 729 / 512,
  7: 3 / 2,
  8: 27 / 16,
  9: 16 / 9,
  10: 243 / 128,
  11: 15 / 8,
};

const five_limit: FractionTable = {
  0: 1 / 1,
  1: 16 / 15,
  2: 9 / 8,
  3: 6 / 5,
  4: 5 / 4,
  5: 4 / 3,
  6: 64 / 45,
  7: 3 / 2,
  8: 8 / 5,
  9: 5 / 3,
  10: 16 / 9,
  11: 15 / 8,
}

const eleven_limit: FractionTable = {
  0: 1 / 1,
  1: 12 / 11,
  2: 11 / 10,
  3: 10 / 9,
  4: 9 / 8,
  5: 8 / 7,
  6: 7 / 6,
  7: 6 / 5,
  8: 11 / 9,
  9: 5 / 4,
  10: 14 / 11,
  11: 9 / 7,
  12: 4 / 3,
  13: 11 / 8,
  14: 7 / 5,
  15: 10 / 7,
  16: 16 / 11,
  17: 3 / 2,
  18: 14 / 9,
  19: 11 / 7,
  20: 8 / 5,
  21: 18 / 11,
  22: 5 / 3,
  23: 12 / 7,
  24: 7 / 4,
  25: 16 / 9,
  26: 9 / 5,
  27: 20 / 11,
  28: 11 / 6,
};

const fortythree_tone: FractionTable = {
  0: 1 / 1,
  1: 81 / 80,
  2: 33 / 32,
  3: 21 / 20,
  4: 16 / 15,
  5: 12 / 11,
  6: 11 / 10,
  7: 10 / 9,
  8: 9 / 8,
  9: 8 / 7,
  10: 7 / 6,
  11: 32 / 27,
  12: 6 / 5,
  13: 11 / 9,
  14: 5 / 4,
  15: 14 / 11,
  16: 9 / 7,
  17: 21 / 16,
  18: 4 / 3,
  19: 27 / 20,
  20: 11 / 8,
  21: 7 / 5,
  22: 10 / 7,
  23: 16 / 11,
  24: 40 / 27,
  25: 3 / 2,
  26: 23 / 21,
  27: 14 / 9,
  28: 11 / 7,
  29: 8 / 5,
  30: 18 / 11,
  31: 5 / 3,
  32: 27 / 16,
  33: 12 / 7,
  34: 7 / 4,
  35: 16 / 8,
  36: 9 / 5,
  37: 20 / 11,
  38: 11 / 6,
  39: 15 / 8,
  40: 40 / 21,
  41: 64 / 33,
  42: 160 / 81,
};

const indian_scale: FractionTable = {
  0: 1 / 1, //sa
  1: 9 / 8, //re
  2: 5 / 4, //ga
  3: 4 / 3, //ma
  4: 3 / 2, //pa
  5: 5 / 3, //dha
  //5: 27 / 16, //dha 
  6: 15 / 8, //ni
}

const indian_scale_full: FractionTable = {
  0: 1 / 1,
  1: 256 / 243,
  2: 16 / 15,
  3: 10 / 9,
  4: 9 / 8,
  5: 32 / 27,
  6: 6 / 5,
  7: 5 / 4,
  8: 81 / 64,
  9: 4 / 3,
  10: 27 / 20,
  11: 45 / 32,
  12: 729 / 512,
  13: 3 / 2,
  14: 128 / 81,
  15: 8 / 5,
  16: 5 / 3,
  17: 27 / 16,
  18: 16 / 9,
  19: 9 / 5,
  20: 15 / 8,
  21: 243 / 128,
}

const step_method_1: FractionTable = {
}

const step_method_5: FractionTable = {
}

const step_method_7: FractionTable = {
}

const step_method_11: FractionTable = {
}

const table_table: Record<string, FractionTable> = {
  "just_intonation": just_intonation,
  "pythagorean_tuning": pythagorean_tuning,
  "eleven_limit": eleven_limit,
  "fortythree_tone": fortythree_tone,
};

// TODO: implement Midi player

const keyboard: Record<string, number> = {
  //TODO: adjust this to match real DAW keymaps and maybe detect keymap and switch between different layouts
  "IntlBackslash": -2,
  "KeyA": -1,
  "KeyZ": 0, // 24
  "KeyS": 1,
  "KeyX": 2,
  "KeyC": 3,
  "KeyF": 4,
  "KeyV": 5,
  "KeyG": 6,
  "KeyB": 7,
  "KeyN": 8,
  "KeyJ": 9,
  "KeyM": 10,
  "KeyK": 11,
  "Comma": 12,
  "KeyL": 13,
  "Period": 14,
  "Slash": 15,
  "Quote": 16,
  "Digit1": 16,
  "BackSlash": 17,
  "KeyQ": 17, // 36
  "Digit2": 18,
  "KeyW": 19,
  "KeyE": 20,
  "Digit4": 21,
  "KeyR": 22,
  "Digit5": 23,
  "KeyT": 24,
  "Digit6": 25,
  "KeyY": 26,
  "KeyU": 27,
  "Digit8": 28,
  "KeyI": 29,
  "Digit9": 30,
  "KeyO": 31,
  "KeyP": 32,
  "Minus": 33,
  "BracketLeft": 34,
  "Equal": 35,
  "BracketRight": 36,
};
