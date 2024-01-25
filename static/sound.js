"use strict";
document.addEventListener("keydown", function (event) {
    const tuningSelect = document.getElementById("tuningSelect");
    const instrumentSelect = document.getElementById("instrumentSelect");
    let n;
    switch (event.code) {
        case "KeyY":
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
        case "KeyZ":
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
    if (n == -1)
        return;
    let ratio;
    switch (tuningSelect.value) {
        default:
        case "twelve_tone":
            ratio = twelve_tet_get_interval(n);
            break;
        case "just_intonation":
            ratio = just_intonation_get_interval(n);
            break;
    }
    const base_freq = 220;
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
const audioContext = new (window.AudioContext || window.webkitAudAudioContextioContext)();
function playFrequencyTone(frequency) {
    console.log(frequency);
    const synth = new Tone.Synth().toDestination();
    synth.triggerAttackRelease(frequency, "8n");
}
function playFrequency(frequency) {
    console.log(frequency);
    const oscillator = audioContext.createOscillator();
    oscillator.type = "square"; // You can change the waveform type if needed
    oscillator.frequency.setValueAtTime(frequency, audioContext.currentTime);
    oscillator.connect(audioContext.destination);
    oscillator.start();
    oscillator.stop(audioContext.currentTime + 0.3); // Stop after 0.5 seconds
}
function twelve_tet_get_interval(n) {
    return Math.pow(2, n / 12);
}
function just_intonation_get_interval(n) {
    let n2 = n % 12;
    let ratio = just_intonation[n2];
    let twelves = Math.floor(n / 12);
    return ratio + twelves;
}
const just_intonation = {
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
};
