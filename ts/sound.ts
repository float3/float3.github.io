import * as Tone from "tone";

document.addEventListener("keydown", function (event) {
  const tuningSelect = document.getElementById(
    "tuningSelect"
  ) as HTMLSelectElement;

  const instrumentSelect = document.getElementById(
    "instrumentSelect"
  ) as HTMLSelectElement;


  let n: number;

  switch (event.key.toUpperCase()) {
    case "Y":
      n = 0;
      break;
    case "S":
      n = 1;
      break;
    case "X":
      n = 2;
      break;
    case "D":
      n = 3;
      break;
    case "C":
      n = 4;
      break;
    case "V":
      n = 5;
      break;
    case "G":
      n = 6;
      break;
    case "B":
      n = 7;
      break;
    case "H":
      n = 8;
      break;
    case "N":
      n = 9;
      break;
    case "J":
      n = 10;
      break;
    case "M":
      n = 11;
      break;
    case ",":
    case "<":
      n = 12;
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
      ratio = twelve_tet_get_interval(n);
      break;
    case "just_intonation":
      ratio = just_intonation_get_interval(n);
      break;
  }

  const base_freq: number = 220;


  switch (instrumentSelect.value) {
    case "tone.js":
      playFrequencyTone(ratio * base_freq);
      break;
    default:
    case "audioContext":
      playFrequency(ratio * base_freq);
      break;
  }
});

const audioContext = new (window.AudioContext || window.webkitAudioContext)();

function playFrequencyTone(frequency: number) {
  console.log(frequency);
  const synth = new Tone.Synth().toDestination();
  synth.triggerAttackRelease(frequency, "8n");
}

function playFrequency(frequency: number) {
  console.log(frequency);
  const oscillator = audioContext.createOscillator();
  oscillator.type = "square"; // You can change the waveform type if needed
  oscillator.frequency.setValueAtTime(frequency, audioContext.currentTime);
  oscillator.connect(audioContext.destination);
  oscillator.start();
  oscillator.stop(audioContext.currentTime + 0.3); // Stop after 0.5 seconds
}

function twelve_tet_get_interval(n: number): number {
  return Math.pow(2, n / 12);
}

function just_intonation_get_interval(n: number): number {
  return just_intonation[n];
}

type FractionTable = Record<number, number>;

const just_intonation: FractionTable = {
  0: 1,
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
  12: 2,
};
