use std::error::Error;
use std::f64::consts::TAU;
use std::io::Write;

use music21_rs::tuningsystem::TWELVE_TONE_NAMES;
use music21_rs::{abc_chord, abc_note, Pitch, TuningSystem};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub const SAMPLE_RATE: u32 = 44_100;
const MIDDLE_C_PITCH_SPACE: f64 = 60.0;

const EQUAL_TEMPERAMENT: TuningSystem = TuningSystem::EqualTemperament { octave_size: 12 };
const JUST_INTONATION: TuningSystem = TuningSystem::JustIntonation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChordKind {
    Major,
    Dominant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tuning {
    TwelveTet,
    FixedCJust,
    RecursiveJust,
    TwelveTetRootedJust,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToneColor {
    Harmonic,
    PureSine,
}

#[derive(Clone, Copy)]
struct Chord {
    name: &'static str,
    root_pc: i32,
    octave_shift: i32,
    kind: ChordKind,
}

#[derive(Clone, Copy)]
struct SplitPair {
    note: &'static str,
    abc_pitch: &'static str,
    chord: Chord,
    offset: i32,
}

#[derive(Clone, Copy)]
struct NoteEvent {
    frequency: f64,
    start: f64,
    duration: f64,
    amplitude: f64,
}

pub struct GeneratedBinary {
    pub name: &'static str,
    pub bytes: Vec<u8>,
}

pub struct GeneratedText {
    pub name: &'static str,
    pub text: String,
}

impl ChordKind {
    fn offsets(self) -> &'static [i32] {
        match self {
            ChordKind::Major => &[0, 4, 7],
            ChordKind::Dominant => &[0, 4, 7, 10],
        }
    }
}

impl Tuning {
    fn all() -> [Self; 3] {
        [Self::TwelveTet, Self::FixedCJust, Self::RecursiveJust]
    }

    fn file_stem(self) -> &'static str {
        match self {
            Tuning::TwelveTet => "twelve-tet-progression",
            Tuning::FixedCJust => "fixed-c-ji-progression",
            Tuning::RecursiveJust => "recursive-ji-progression",
            Tuning::TwelveTetRootedJust => "twelve-tet-rooted-ji-progression",
        }
    }

    fn sine_file_stem(self) -> &'static str {
        match self {
            Tuning::TwelveTet => "twelve-tet-sine-progression",
            Tuning::FixedCJust => "fixed-c-ji-sine-progression",
            Tuning::RecursiveJust => "recursive-ji-sine-progression",
            Tuning::TwelveTetRootedJust => "twelve-tet-rooted-ji-sine-progression",
        }
    }

    fn drone_file_stem(self) -> &'static str {
        match self {
            Tuning::TwelveTet => "twelve-tet-c-drone-progression",
            Tuning::FixedCJust => "fixed-c-ji-c-drone-progression",
            Tuning::RecursiveJust => "recursive-ji-c-drone-progression",
            Tuning::TwelveTetRootedJust => "twelve-tet-rooted-ji-c-drone-progression",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Tuning::TwelveTet => "12-TET",
            Tuning::FixedCJust => "Fixed C just intonation",
            Tuning::RecursiveJust => "Recursive just intonation",
            Tuning::TwelveTetRootedJust => "12-TET-rooted just intonation",
        }
    }
}

impl Chord {
    fn major(name: &'static str, root_pc: i32) -> Self {
        Self {
            name,
            root_pc,
            octave_shift: -1,
            kind: ChordKind::Major,
        }
    }

    fn dominant(name: &'static str, root_pc: i32) -> Self {
        Self {
            name,
            root_pc,
            octave_shift: -1,
            kind: ChordKind::Dominant,
        }
    }
}

pub fn generated_audio_files() -> Result<Vec<GeneratedBinary>> {
    let mut files = Vec::new();

    for tuning in Tuning::all() {
        files.push(GeneratedBinary {
            name: concat_wav(tuning.file_stem()),
            bytes: wav_bytes(&render_progression(tuning, ToneColor::Harmonic))?,
        });
        files.push(GeneratedBinary {
            name: concat_wav(tuning.sine_file_stem()),
            bytes: wav_bytes(&render_progression(tuning, ToneColor::PureSine))?,
        });
        files.push(GeneratedBinary {
            name: concat_wav(tuning.drone_file_stem()),
            bytes: wav_bytes(&render_c_drone_progression(tuning))?,
        });
    }

    files.push(GeneratedBinary {
        name: "recursive-ji-note-splits.wav",
        bytes: wav_bytes(&render_note_splits())?,
    });
    files.push(GeneratedBinary {
        name: "twelve-tet-rooted-ji-progression.wav",
        bytes: wav_bytes(&render_progression(
            Tuning::TwelveTetRootedJust,
            ToneColor::Harmonic,
        ))?,
    });

    Ok(files)
}

pub fn generated_text_files() -> Vec<GeneratedText> {
    vec![GeneratedText {
        name: "recursive-ji-frequencies.csv",
        text: frequency_report(),
    }]
}

pub fn chord_progression_abc() -> Result<String> {
    let chords = progression();
    let mut bars = Vec::new();

    for chord in chords {
        let pitches = notated_pitches(chord)?;
        bars.push(format!(
            "\"{}\"{}4",
            abc_label(notation_chord_label(chord)),
            abc_chord(&pitches)?
        ));
    }

    let mut abc = String::from(
        "X:1\nT:Recursive just intonation chord progression\nL:1/4\nM:4/4\nK:C clef=treble\n",
    );

    for (index, bar) in bars.iter().enumerate() {
        abc.push_str(bar);
        if index + 1 == bars.len() {
            abc.push_str(" |]\n");
        } else if (index + 1) % 4 == 0 {
            abc.push_str(" |\n");
        } else {
            abc.push_str(" | ");
        }
    }

    Ok(abc)
}

pub fn note_splits_abc() -> Result<String> {
    let mut abc = String::from("X:2\nT:Pitch-name splits\nL:1/4\nM:3/4\nK:C clef=treble\n");

    for pair in split_pairs() {
        let token = abc_note(&Pitch::from_name(pair.abc_pitch)?)?;
        let fixed = note_frequency(Tuning::FixedCJust, pair.chord, pair.offset);
        let recursive = note_frequency(Tuning::RecursiveJust, pair.chord, pair.offset);
        let difference = format_signed_cents(cents_between(recursive, fixed));
        let context = chord_context_label(pair.chord);

        abc.push_str(&format!(
            "\"^{} {} fixed +0.000c\"{} \"^recursive {}c\"{} \"^together +0.000/{}c\"[{}{}] |",
            abc_label(&context),
            abc_label(pair.note),
            token,
            difference,
            token,
            difference,
            token,
            token,
        ));
        abc.push('\n');
    }

    abc.push_str("|]\n");
    Ok(abc)
}

pub fn render_progression(tuning: Tuning, tone_color: ToneColor) -> Vec<f32> {
    let chords = progression();
    let mut events = Vec::new();
    let mut cursor = 0.0;

    for chord in chords {
        add_chord_events(&mut events, chord, tuning, cursor, 1.8);
        cursor += 1.8;
    }

    synthesize_with_tone_color(&events, cursor + 0.4, tone_color)
}

pub fn render_c_drone_progression(tuning: Tuning) -> Vec<f32> {
    let chords = progression();
    let mut events = Vec::new();
    let mut cursor = 0.0;

    for chord in chords {
        add_chord_events(&mut events, chord, tuning, cursor, 1.8);
        cursor += 1.8;
    }

    events.push(NoteEvent {
        frequency: fixed_c_frequency(-1, 0),
        start: 0.0,
        duration: cursor + 0.4,
        amplitude: 0.12,
    });

    synthesize_with_tone_color(&events, cursor + 0.4, ToneColor::Harmonic)
}

pub fn render_note_splits() -> Vec<f32> {
    let mut events = Vec::new();
    let mut cursor = 0.0;

    for pair in split_pairs() {
        let recursive = note_frequency(Tuning::RecursiveJust, pair.chord, pair.offset);
        let fixed = note_frequency(Tuning::FixedCJust, pair.chord, pair.offset);
        events.push(NoteEvent {
            frequency: fixed,
            start: cursor,
            duration: 0.85,
            amplitude: 0.4,
        });
        events.push(NoteEvent {
            frequency: recursive,
            start: cursor + 0.9,
            duration: 0.85,
            amplitude: 0.4,
        });
        events.push(NoteEvent {
            frequency: fixed,
            start: cursor + 1.9,
            duration: 1.2,
            amplitude: 0.22,
        });
        events.push(NoteEvent {
            frequency: recursive,
            start: cursor + 1.9,
            duration: 1.2,
            amplitude: 0.22,
        });
        cursor += 3.4;
    }

    synthesize_with_tone_color(&events, cursor + 0.4, ToneColor::Harmonic)
}

pub fn frequency_report() -> String {
    let mut body =
        String::from("tuning,chord,note,frequency_hz,cents_vs_12_tet,cents_vs_fixed_c_ji\n");

    for chord in progression() {
        for offset in chord.kind.offsets() {
            for tuning in Tuning::all() {
                let frequency = note_frequency(tuning, chord, *offset);
                let tet = note_frequency(Tuning::TwelveTet, chord, *offset);
                let fixed = note_frequency(Tuning::FixedCJust, chord, *offset);
                body.push_str(&format!(
                    "{},{},{},{:.3},{},{}\n",
                    tuning.label(),
                    chord.name,
                    TWELVE_TONE_NAMES[(chord.root_pc + *offset).rem_euclid(12) as usize],
                    frequency,
                    format_cents(cents_between(frequency, tet)),
                    format_cents(cents_between(frequency, fixed)),
                ));
            }
        }
    }

    body
}

fn concat_wav(stem: &'static str) -> &'static str {
    match stem {
        "twelve-tet-progression" => "twelve-tet-progression.wav",
        "twelve-tet-sine-progression" => "twelve-tet-sine-progression.wav",
        "twelve-tet-c-drone-progression" => "twelve-tet-c-drone-progression.wav",
        "fixed-c-ji-progression" => "fixed-c-ji-progression.wav",
        "fixed-c-ji-sine-progression" => "fixed-c-ji-sine-progression.wav",
        "fixed-c-ji-c-drone-progression" => "fixed-c-ji-c-drone-progression.wav",
        "recursive-ji-progression" => "recursive-ji-progression.wav",
        "recursive-ji-sine-progression" => "recursive-ji-sine-progression.wav",
        "recursive-ji-c-drone-progression" => "recursive-ji-c-drone-progression.wav",
        "twelve-tet-rooted-ji-progression" => "twelve-tet-rooted-ji-progression.wav",
        "twelve-tet-rooted-ji-sine-progression" => "twelve-tet-rooted-ji-sine-progression.wav",
        "twelve-tet-rooted-ji-c-drone-progression" => {
            "twelve-tet-rooted-ji-c-drone-progression.wav"
        }
        _ => unreachable!("unexpected recursive JI audio stem"),
    }
}

fn add_chord_events(
    events: &mut Vec<NoteEvent>,
    chord: Chord,
    tuning: Tuning,
    start: f64,
    duration: f64,
) {
    let offsets = chord.kind.offsets();

    for (index, offset) in offsets.iter().enumerate() {
        events.push(NoteEvent {
            frequency: note_frequency(tuning, chord, *offset),
            start: start + index as f64 * 0.15,
            duration: duration - index as f64 * 0.08,
            amplitude: 0.28,
        });
    }

    for offset in offsets {
        events.push(NoteEvent {
            frequency: note_frequency(tuning, chord, *offset + 12),
            start: start + 0.55,
            duration: duration * 0.62,
            amplitude: 0.11,
        });
    }
}

fn progression() -> [Chord; 12] {
    [
        Chord::major("C", 0),
        Chord::major("E", 4),
        Chord::major("G#/Ab", 8),
        Chord::major("C", 0),
        Chord::major("F", 5),
        Chord::major("A", 9),
        Chord::major("D", 2),
        Chord::dominant("G7", 7),
        Chord::major("C", 0),
        Chord::major("E", 4),
        Chord::major("F", 5),
        Chord::major("C", 0),
    ]
}

fn split_pairs() -> [SplitPair; 4] {
    [
        SplitPair {
            note: "G#/Ab",
            abc_pitch: "G#4",
            chord: Chord::major("E", 4),
            offset: 4,
        },
        SplitPair {
            note: "A",
            abc_pitch: "A4",
            chord: Chord::major("F", 5),
            offset: 4,
        },
        SplitPair {
            note: "C#/Db",
            abc_pitch: "C#5",
            chord: Chord::major("A", 9),
            offset: 4,
        },
        SplitPair {
            note: "F",
            abc_pitch: "F4",
            chord: Chord::dominant("G7", 7),
            offset: 10,
        },
    ]
}

fn notated_pitches(chord: Chord) -> Result<Vec<Pitch>> {
    let names = match (chord.name, chord.kind) {
        ("C", ChordKind::Major) => &["C4", "E4", "G4"][..],
        ("E", ChordKind::Major) => &["E4", "G#4", "B4"][..],
        ("G#/Ab", ChordKind::Major) => &["A-4", "C5", "E-5"][..],
        ("F", ChordKind::Major) => &["F4", "A4", "C5"][..],
        ("A", ChordKind::Major) => &["A4", "C#5", "E5"][..],
        ("D", ChordKind::Major) => &["D4", "F#4", "A4"][..],
        ("G7", ChordKind::Dominant) => &["G3", "B3", "D4", "F4"][..],
        _ => &["C4", "E4", "G4"][..],
    };

    names
        .iter()
        .map(|name| Pitch::from_name(*name).map_err(|err| err.into()))
        .collect()
}

fn chord_context_label(chord: Chord) -> String {
    match chord.kind {
        ChordKind::Major => format!("{} major", notation_chord_label(chord)),
        ChordKind::Dominant => chord.name.to_string(),
    }
}

fn notation_chord_label(chord: Chord) -> &'static str {
    match (chord.name, chord.kind) {
        ("G#/Ab", ChordKind::Major) => "Ab",
        _ => chord.name,
    }
}

fn note_frequency(tuning: Tuning, chord: Chord, offset: i32) -> f64 {
    match tuning {
        Tuning::TwelveTet => EQUAL_TEMPERAMENT.frequency_at(chromatic_pitch_space(
            chord.octave_shift,
            chord.root_pc + offset,
        )),
        Tuning::FixedCJust => fixed_c_frequency(chord.octave_shift, chord.root_pc + offset),
        Tuning::RecursiveJust => {
            let root = fixed_c_frequency(chord.octave_shift, chord.root_pc);
            root * just_ratio_for_interval(offset)
        }
        Tuning::TwelveTetRootedJust => {
            let root = EQUAL_TEMPERAMENT
                .frequency_at(chromatic_pitch_space(chord.octave_shift, chord.root_pc));
            root * just_ratio_for_interval(offset)
        }
    }
}

fn fixed_c_frequency(octave_shift: i32, absolute_index: i32) -> f64 {
    JUST_INTONATION.frequency_at(chromatic_pitch_space(octave_shift, absolute_index))
}

fn just_ratio_for_interval(offset: i32) -> f64 {
    let root = JUST_INTONATION.frequency_at(MIDDLE_C_PITCH_SPACE);
    let target = JUST_INTONATION.frequency_at(MIDDLE_C_PITCH_SPACE + f64::from(offset));
    target / root
}

fn chromatic_pitch_space(octave_shift: i32, absolute_index: i32) -> f64 {
    MIDDLE_C_PITCH_SPACE + f64::from(octave_shift * 12 + absolute_index)
}

fn synthesize_with_tone_color(
    events: &[NoteEvent],
    duration: f64,
    tone_color: ToneColor,
) -> Vec<f32> {
    let sample_count = (duration * f64::from(SAMPLE_RATE)).ceil() as usize;
    let mut buffer = vec![0.0_f64; sample_count];

    for event in events {
        let start_index = (event.start * f64::from(SAMPLE_RATE)).max(0.0) as usize;
        let end_index = ((event.start + event.duration) * f64::from(SAMPLE_RATE))
            .ceil()
            .min(sample_count as f64) as usize;

        for (index, sample) in buffer
            .iter_mut()
            .enumerate()
            .take(end_index)
            .skip(start_index)
        {
            let local_t = index as f64 / f64::from(SAMPLE_RATE) - event.start;
            let phase = TAU * event.frequency * local_t;
            let tone = match tone_color {
                ToneColor::Harmonic => {
                    phase.sin() * 0.78 + (phase * 2.0).sin() * 0.17 + (phase * 3.0).sin() * 0.05
                }
                ToneColor::PureSine => phase.sin(),
            };
            *sample += tone * envelope(local_t, event.duration) * event.amplitude;
        }
    }

    normalize(buffer)
}

fn envelope(t: f64, duration: f64) -> f64 {
    let attack = 0.025;
    let release = 0.18_f64.min(duration * 0.35);
    if t < attack {
        t / attack
    } else if t > duration - release {
        ((duration - t) / release).clamp(0.0, 1.0)
    } else {
        let sustain_fade = 1.0 - 0.35 * (t / duration);
        sustain_fade.clamp(0.0, 1.0)
    }
}

fn normalize(buffer: Vec<f64>) -> Vec<f32> {
    let peak = buffer.iter().copied().map(f64::abs).fold(0.0_f64, f64::max);
    let gain = if peak > 0.0 { 0.86 / peak } else { 1.0 };

    buffer
        .into_iter()
        .map(|sample| (sample * gain).clamp(-1.0, 1.0) as f32)
        .collect()
}

fn wav_bytes(samples: &[f32]) -> Result<Vec<u8>> {
    let channels = 1_u16;
    let bits_per_sample = 16_u16;
    let block_align = channels * bits_per_sample / 8;
    let byte_rate = SAMPLE_RATE * u32::from(block_align);
    let data_len = samples.len() as u32 * u32::from(block_align);
    let mut bytes = Vec::with_capacity(44 + data_len as usize);

    bytes.write_all(b"RIFF")?;
    bytes.write_all(&(36 + data_len).to_le_bytes())?;
    bytes.write_all(b"WAVE")?;
    bytes.write_all(b"fmt ")?;
    bytes.write_all(&16_u32.to_le_bytes())?;
    bytes.write_all(&1_u16.to_le_bytes())?;
    bytes.write_all(&channels.to_le_bytes())?;
    bytes.write_all(&SAMPLE_RATE.to_le_bytes())?;
    bytes.write_all(&byte_rate.to_le_bytes())?;
    bytes.write_all(&block_align.to_le_bytes())?;
    bytes.write_all(&bits_per_sample.to_le_bytes())?;
    bytes.write_all(b"data")?;
    bytes.write_all(&data_len.to_le_bytes())?;

    for sample in samples {
        let pcm = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
        bytes.write_all(&pcm.to_le_bytes())?;
    }

    Ok(bytes)
}

fn cents_between(a: f64, b: f64) -> f64 {
    1200.0 * (a / b).log2()
}

fn format_cents(value: f64) -> String {
    let rounded = (value * 1000.0).round() / 1000.0;
    if rounded == 0.0 {
        "0.000".to_string()
    } else {
        format!("{rounded:.3}")
    }
}

fn format_signed_cents(value: f64) -> String {
    let rounded = (value * 1000.0).round() / 1000.0;
    if rounded >= 0.0 {
        format!("+{rounded:.3}")
    } else {
        format!("{rounded:.3}")
    }
}

fn abc_label(label: &str) -> String {
    label.replace('"', "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_e_major_third_is_not_fixed_c_g_sharp() {
        let chord = Chord::major("E", 4);
        let recursive = note_frequency(Tuning::RecursiveJust, chord, 4);
        let fixed = note_frequency(Tuning::FixedCJust, chord, 4);

        assert!((cents_between(recursive, fixed) + 34.282).abs() < 0.01);
    }

    #[test]
    fn twelve_tet_rooted_just_keeps_tet_roots_and_just_intervals() {
        let chord = Chord::major("E", 4);
        let root = note_frequency(Tuning::TwelveTetRootedJust, chord, 0);
        let tet_root = note_frequency(Tuning::TwelveTet, chord, 0);
        let third = note_frequency(Tuning::TwelveTetRootedJust, chord, 4);

        assert!((cents_between(root, tet_root)).abs() < 0.001);
        assert!((third / root - 5.0 / 4.0).abs() < 0.000_001);
    }

    #[test]
    fn renders_audio() {
        let samples = render_progression(Tuning::RecursiveJust, ToneColor::Harmonic);

        assert!(samples.len() > SAMPLE_RATE as usize);
        assert!(samples.iter().any(|sample| sample.abs() > 0.01));
    }

    #[test]
    fn renders_pure_sine_progression() {
        let samples = render_progression(Tuning::RecursiveJust, ToneColor::PureSine);

        assert!(samples.len() > SAMPLE_RATE as usize);
        assert!(samples.iter().any(|sample| sample.abs() > 0.01));
    }

    #[test]
    fn renders_c_drone_progression() {
        let samples = render_c_drone_progression(Tuning::RecursiveJust);

        assert!(samples.len() > SAMPLE_RATE as usize);
        assert!(samples.iter().any(|sample| sample.abs() > 0.01));
    }

    #[test]
    fn generates_notation() {
        let progression = chord_progression_abc().unwrap();
        let splits = note_splits_abc().unwrap();

        assert!(progression.contains("Recursive just intonation chord progression"));
        assert!(progression.contains("\"Ab\""));
        assert!(!progression.contains("\"G#/Ab\""));
        assert!(splits.contains("Pitch-name splits"));
        assert!(splits.contains("-34.283c"));
    }
}
