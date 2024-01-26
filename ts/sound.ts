import * as Tone from "tone";

document.addEventListener("keydown", function (event) {
  const tuningSelect = document.getElementById(
    "tuningSelect"
  ) as HTMLSelectElement;

  const instrumentSelect = document.getElementById(
    "instrumentSelect"
  ) as HTMLSelectElement;

  const volumeSlider = document.getElementById(
    "volumeSlider"
  ) as HTMLInputElement;

  let n: number;

  switch (event.code) {
    case "KeyZ":
      n = 0;
      break;
    case "KeyS":
      n = 1;
      break;
    case "KeyX":
      n = 2;
      break;
    case "KeyD":
      n = 3;
      break;
    case "KeyC":
      n = 4;
      break;
    case "KeyV":
      n = 5;
      break;
    case "KeyG":
      n = 6;
      break;
    case "KeyB":
      n = 7;
      break;
    case "KeyH":
      n = 8;
      break;
    case "KeyN":
      n = 9;
      break;
    case "KeyJ":
      n = 10;
      break;
    case "KeyM":
      n = 11;
      break;
    case "KeyQ":
      n = 12;
      break;
    case "Digit2":
      n = 13;
      break;
    case "KeyW":
      n = 14;
      break;
    case "Digit3":
      n = 15;
      break;
    case "KeyE":
      n = 16;
      break;
    case "KeyR":
      n = 17;
      break;
    case "Digit5":
      n = 18;
      break;
    case "KeyT":
      n = 19;
      break;
    case "Digit6":
      n = 20;
      break;
    case "KeyY":
      n = 21;
      break;
    case "Digit7":
      n = 22;
      break;
    case "KeyU":
      n = 23;
      break;
    case "KeyI":
      n = 24;
      break;
    case "Digit9":
      n = 25;
      break;
    case "KeyO":
      n = 26;
      break;
    case "Digit0":
      n = 27;
      break;
    case "KeyP":
      n = 28;
      break;
    default:
      n = -1;
      break;
  }

  if (n == -1) return;

  let ratio: number;

  switch (tuningSelect.value) {
    default:
    case "twelve_tone":
      ratio = equal_temperament_get_interval(n, 12);
      break;
    case "twentyfour_tone":
      ratio = equal_temperament_get_interval(n, 24);
      break;
    case "just_intonation":
      ratio = table_get_interval(n, just_intonation, 12);
      break;
    case "pythagorean_tuning":
      ratio = table_get_interval(n, pythagorean_tuning, 12);
      break;
    case "eleven_limit":
      ratio = table_get_interval(n, eleven_limit, 30);
      break;
    case "fortythree_tone":
      ratio = table_get_interval(n, fortythree_tone, 43);
      break;
  }

  const base_freq: number = 220; // TODO: make this configurable

  let volume: number = Math.pow(parseFloat(volumeSlider.value), 2);

  let freq: number = ratio * base_freq;
  console.log(freq); // TODO: print this to screen or visualize it

  switch (instrumentSelect.value) {
    case "tone.js":
      playFrequencyTone(freq, volume);
      break;
    default:
    case "audioContext":
      playFrequency(freq, volume);
      break;
  }
});

function playFrequencyTone(frequency: number, volume: number) {
  const synth = new Tone.Synth().toDestination();
  synth.volume.value *= volume; // TODO: test volume
  synth.triggerAttackRelease(frequency, "8n");
}

function playFrequency(frequency: number, volume: number) {
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

function equal_temperament_get_interval(n: number, base: number): number {
  return Math.pow(2, n / base);
}

function table_get_interval(
  n: number,
  table: FractionTable,
  tablesize: number
): number {
  let n2: number = n % tablesize;
  let ratio: number = table[n2];
  let octaves: number = Math.floor(n / tablesize);
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
  1: 1 / 1,
  2: 12 / 11,
  3: 11 / 10,
  4: 10 / 9,
  5: 9 / 8,
  6: 8 / 7,
  7: 7 / 6,
  8: 6 / 5,
  9: 11 / 9,
  10: 5 / 4,
  11: 14 / 11,
  12: 9 / 7,
  13: 4 / 3,
  14: 11 / 8,
  15: 7 / 5,
  16: 10 / 7,
  17: 16 / 11,
  18: 3 / 2,
  19: 14 / 9,
  20: 11 / 7,
  21: 8 / 5,
  22: 18 / 11,
  23: 5 / 3,
  24: 12 / 7,
  25: 7 / 4,
  26: 16 / 9,
  27: 9 / 5,
  28: 20 / 11,
  29: 11 / 6
}

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
