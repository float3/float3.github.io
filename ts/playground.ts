import * as Tone from "tone";

type FractionTable = Record<number, number>;
type ToneList = Array<[number, OscillatorNode]>;

var logContainer: HTMLElement;
var tuningSelect: HTMLSelectElement;
var baseFreq: HTMLInputElement;
var volumeSlider: HTMLInputElement;
var equalTemperamentBase: HTMLInputElement;
var equalTemperamentBaseContainer: HTMLDivElement;

var synth: Tone.Synth<Tone.SynthOptions>;
var audioContext: AudioContext;

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
  equalTemperamentBase = document.getElementById(
    "equalTemperamentBase"
  ) as HTMLInputElement;
  equalTemperamentBaseContainer = document.getElementById(
    "equalTemperamentBaseContainer"
  ) as HTMLDivElement;

  synth = new Tone.Synth().toDestination();

  playingNotes = [];
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
    console.log("test");
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
  logToDiv(freq);

  let volume: number = Math.pow(parseFloat(volumeSlider.value), 2);

  playFrequency(freq, volume, n);
}

function getRatio(n: number): number {
  let ratio: number;
  switch (tuningSelect.value) {
    case "equal_temperament":
      ratio = equal_temperament_get_interval(
        n,
        parseFloat(equalTemperamentBase.value)
      );
      break;
    default:
      ratio = table_get_interval(n, table_table[tuningSelect.value]);
      break;
  }
  return ratio;
}

function playFrequency(frequency: number, volume: number, n: number): void {
  const soundMethod = document.getElementById(
    "soundMethod"
  ) as HTMLSelectElement;
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

function toggleInputVisibility(): void {
  if (tuningSelect.value === "equal_temperament") {
    equalTemperamentBaseContainer.style.display = "block";
  } else {
    equalTemperamentBaseContainer.style.display = "none";
  }
}

function logToDiv(message: any): void {
  logContainer.innerHTML = "<p>" + message + "Hz</p>" + logContainer.innerHTML;
}

function equal_temperament_get_interval(n: number, base: number): number {
  return Math.pow(2, n / base);
}

function table_get_interval(n: number, table: FractionTable): number {
  let tablesize = Object.keys(table).length;
  let n2: number = n % tablesize;
  let ratio: number = table[n2];
  let octaves: number = Math.floor(n / tablesize);
  return ratio + octaves;
}

function step_algorithm(stepsize: number, idx: number) {
  let ratio = table_get_interval(stepsize, just_intonation);
  let current_ratio = 1;
  let current_idx = 0;
  while (true) {
    if (current_idx == idx) {
      return current_ratio;
    }
    current_ratio *= ratio;
    current_idx += stepsize;
    current_idx %= 12;
    if (current_ratio > 2) {
      current_ratio /= 2;
    }
  }
}

// TODO implement 24 Tone Just Intonation?

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

const table_table: Record<string, FractionTable> = {
  "just_intonation": just_intonation,
  "pythagorean_tuning": pythagorean_tuning,
  "eleven_limit": eleven_limit,
  "fortythree_tone": fortythree_tone,
};

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
