import * as Tone from "tone";

document.addEventListener("keydown", function (event) {
  const logContainer = document.getElementById("logContainer") as HTMLElement;
  if (event.code == "Tab") {
    logContainer.innerHTML = "";
    return;
  }

  let n: number = keyboard[event.code] || -1;

  if (n == -1) return;

  const tuningSelect = document.getElementById("tuningSelect") as HTMLSelectElement;
  let ratio: number = getRatio(tuningSelect, n);

  const baseFreq = document.getElementById("baseFreq") as HTMLInputElement;
  let root: number = parseFloat(baseFreq.value);

  let freq: number = ratio * root;
  logToDiv(freq, logContainer);

  const volumeSlider = document.getElementById("volumeSlider") as HTMLInputElement;
  let volume: number = Math.pow(parseFloat(volumeSlider.value), 2);

  playFrequency(freq, volume);
});

function getRatio(tuningSelect: HTMLSelectElement, n: number): number {
  const equalTemperamentBase = document.getElementById("equalTemperamentBase") as HTMLInputElement;
  let ratio: number;
  switch (tuningSelect.value) {
    default:
    case "equal_temperament":
      ratio = equal_temperament_get_interval(n, parseFloat(equalTemperamentBase.value));
      break;
    case "just_intonation":
      ratio = table_get_interval(n, just_intonation);
      break;
    case "pythagorean_tuning":
      ratio = table_get_interval(n, pythagorean_tuning);
      break;
    case "eleven_limit":
      ratio = table_get_interval(n, eleven_limit);
      break;
    case "fortythree_tone":
      ratio = table_get_interval(n, fortythree_tone);
      break;
  }
  return ratio;
}

function playFrequency(frequency: number, volume: number): void {
  const soundMethod = document.getElementById("soundMethod") as HTMLSelectElement;
  switch (soundMethod.value) {
    default:
    case "native":
      playFrequencyNative(frequency, volume);
      break;
    case "tone.js":
      playFrequencyToneJS(frequency, volume);
      break;
  }
}

function playFrequencyToneJS(frequency: number, volume: number): void {
  const synth = new Tone.Synth().toDestination();
  synth.volume.value *= volume; // TODO: test volume
  synth.triggerAttackRelease(frequency, "8n");
}

function playFrequencyNative(frequency: number, volume: number): void {
  const audioContext = new window.AudioContext(); // TODO: check if audio issue is fixed
  const oscillator = audioContext.createOscillator();
  let gainNode = audioContext.createGain();
  gainNode.gain.value = volume;
  gainNode.connect(audioContext.destination);
  oscillator.type = "square"; // TODO: make this configurable
  oscillator.frequency.setValueAtTime(frequency, audioContext.currentTime);
  oscillator.connect(gainNode);
  oscillator.start();
  oscillator.stop(audioContext.currentTime + 0.3);
}

function toggleInputVisibility(): void {
  const tuningSelect = document.getElementById("tuningSelect") as HTMLSelectElement;
  const equalTemperamentBaseContainer = document.getElementById("equalTemperamentBaseContainer") as HTMLDivElement;

  if (tuningSelect.value === "equal_temperament") {
    equalTemperamentBaseContainer.style.display = "block";
  } else {
    equalTemperamentBaseContainer.style.display = "none";
  }
}

function logToDiv(message: any, logContainer: HTMLElement): void {
  logContainer.innerHTML = "<p>" + message + "Hz</p>" + logContainer.innerHTML;
}

function equal_temperament_get_interval(n: number, base: number): number {
  return Math.pow(2, n / base);
}

function table_get_interval(
  n: number,
  table: FractionTable,
): number {
  let tablesize = Object.keys(table).length;
  let n2: number = n % tablesize;
  let ratio: number = table[n2];
  let octaves: number = Math.floor(n / tablesize);
  console.log(tablesize);
  console.log(n);
  console.log(n2);
  console.log(ratio);
  console.log(octaves);
  return ratio + octaves;
}

type FractionTable = Record<number, number>;
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

const keyboard: Record<string, number> = {
  "IntlBackslash": 0,
  "KeyA": 1,
  "KeyZ": 2,
  "KeyS": 3,
  "KeyX": 4,
  "KeyC": 5,
  "KeyF": 6,
  "KeyV": 7,
  "KeyG": 8,
  "KeyB": 9,
  "KeyN": 10,
  "KeyJ": 11,
  "KeyM": 12,
  "KeyK": 13,
  "Comma": 14,
  "KeyL": 15,
  "Period": 16,
  "Slash": 17,
  "Quote": 18,
  "Digit1": 18,
  "KeyQ": 19,
  "Digit2": 20,
  "KeyW": 21,
  "KeyE": 22,
  "Digit4": 23,
  "KeyR": 24,
  "Digit5": 25,
  "KeyT": 26,
  "Digit6": 27,
  "KeyY": 28,
  "KeyU": 29,
  "Digit8": 30,
  "KeyI": 31,
  "Digit9": 32,
  "KeyO": 33,
  "KeyP": 34,
  "Minus": 35,
  "BracketLeft": 36,
  "Equal": 37,
  "BracketRight": 38,
}