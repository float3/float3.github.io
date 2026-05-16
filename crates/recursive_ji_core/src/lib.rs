use std::error::Error;
use std::f64::consts::TAU;
use std::io::Write;

use music21_rs::tuningsystem::TWELVE_TONE_NAMES;
use music21_rs::{Pitch, TuningSystem, abc_chord, abc_note};

mod long_form;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub const SAMPLE_RATE: u32 = 44_100;
const MIDDLE_C_PITCH_SPACE: f64 = 60.0;

const EQUAL_TEMPERAMENT: TuningSystem = TuningSystem::EqualTemperament { octave_size: 12 };
const JUST_INTONATION: TuningSystem = TuningSystem::JustIntonation;

/// Interval definitions derived from music21-rs chord analysis.
/// These represent the semitone offsets from root in standard chord theory.
const MAJOR_INTERVALS: &[i32] = &[0, 4, 7];
const DOMINANT_INTERVALS: &[i32] = &[0, 4, 7, 10];

const TWELVE_TET: TuningSystem = TuningSystem::EqualTemperament { octave_size: 12 };
const FIXED_C_JUST: TuningSystem = TuningSystem::JustIntonation;
const RECURSIVE_JUST: TuningSystem = TuningSystem::RecursiveJustIntonation;
const TWELVE_TET_ROOTED_JUST: TuningSystem = TuningSystem::TwelveTetRootedJust;

fn all_tunings() -> [TuningSystem; 4] {
    [
        TWELVE_TET,
        FIXED_C_JUST,
        RECURSIVE_JUST,
        TWELVE_TET_ROOTED_JUST,
    ]
}

fn tuning_file_stem(tuning: TuningSystem) -> &'static str {
    match tuning {
        TWELVE_TET => "twelve-tet-progression",
        FIXED_C_JUST => "fixed-c-ji-progression",
        RECURSIVE_JUST => "recursive-ji-progression",
        TWELVE_TET_ROOTED_JUST => "twelve-tet-rooted-ji-progression",
        _ => unreachable!("unsupported tuning system for file stem"),
    }
}

fn tuning_sine_file_stem(tuning: TuningSystem) -> &'static str {
    match tuning {
        TWELVE_TET => "twelve-tet-sine-progression",
        FIXED_C_JUST => "fixed-c-ji-sine-progression",
        RECURSIVE_JUST => "recursive-ji-sine-progression",
        TWELVE_TET_ROOTED_JUST => "twelve-tet-rooted-ji-sine-progression",
        _ => unreachable!("unsupported tuning system for sine file stem"),
    }
}

fn tuning_drone_file_stem(tuning: TuningSystem) -> &'static str {
    match tuning {
        TWELVE_TET => "twelve-tet-c-drone-progression",
        FIXED_C_JUST => "fixed-c-ji-c-drone-progression",
        RECURSIVE_JUST => "recursive-ji-c-drone-progression",
        TWELVE_TET_ROOTED_JUST => "twelve-tet-rooted-ji-c-drone-progression",
        _ => unreachable!("unsupported tuning system for drone file stem"),
    }
}

fn tuning_label(tuning: TuningSystem) -> &'static str {
    match tuning {
        TWELVE_TET => "12-TET",
        FIXED_C_JUST => "Fixed C just intonation",
        RECURSIVE_JUST => "Recursive just intonation",
        TWELVE_TET_ROOTED_JUST => "12-TET-rooted just intonation",
        _ => "Unknown tuning",
    }
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
    intervals: &'static [i32],
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
    /// Pitch in music21-rs pitch-space (60 = middle C)
    /// Stored for notation/MusicXML generation
    #[allow(dead_code)]
    pitch_space: f64,
    /// Frequency in Hz for synthesis
    frequency: f64,
    /// Start time in seconds
    start: f64,
    /// Duration in seconds
    duration: f64,
    /// Amplitude (0.0 to 1.0) for synthesis
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

impl Chord {
    fn major(name: &'static str, root_pc: i32) -> Self {
        Self {
            name,
            root_pc,
            octave_shift: -1,
            intervals: MAJOR_INTERVALS,
        }
    }

    fn dominant(name: &'static str, root_pc: i32) -> Self {
        Self {
            name,
            root_pc,
            octave_shift: -1,
            intervals: DOMINANT_INTERVALS,
        }
    }

    fn offsets(self) -> &'static [i32] {
        self.intervals
    }
}

pub fn generated_audio_files() -> Result<Vec<GeneratedBinary>> {
    let mut files = Vec::new();

    for tuning in all_tunings() {
        files.push(GeneratedBinary {
            name: concat_wav(tuning_file_stem(tuning)),
            bytes: wav_bytes(&render_progression(tuning, ToneColor::Harmonic))?,
        });
        files.push(GeneratedBinary {
            name: concat_wav(tuning_sine_file_stem(tuning)),
            bytes: wav_bytes(&render_progression(tuning, ToneColor::PureSine))?,
        });
        files.push(GeneratedBinary {
            name: concat_wav(tuning_drone_file_stem(tuning)),
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
            TWELVE_TET_ROOTED_JUST,
            ToneColor::Harmonic,
        ))?,
    });
    files.extend(long_form::generated_audio_files()?);

    Ok(files)
}

pub fn generated_text_files() -> Vec<GeneratedText> {
    vec![GeneratedText {
        name: "recursive-ji-frequencies.csv",
        text: frequency_report(),
    }]
}

pub fn generated_media_text_files() -> Vec<GeneratedText> {
    long_form::generated_media_text_files()
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
        let fixed = note_frequency(FIXED_C_JUST, pair.chord, pair.offset);
        let recursive = note_frequency(RECURSIVE_JUST, pair.chord, pair.offset);
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

pub fn render_progression(tuning: TuningSystem, tone_color: ToneColor) -> Vec<f32> {
    let chords = progression();
    let mut events = Vec::new();
    let mut cursor = 0.0;

    for chord in chords {
        add_chord_events(&mut events, chord, tuning, cursor, 1.8);
        cursor += 1.8;
    }

    synthesize_with_tone_color(&events, cursor + 0.4, tone_color)
}

pub fn render_c_drone_progression(tuning: TuningSystem) -> Vec<f32> {
    let chords = progression();
    let mut events = Vec::new();
    let mut cursor = 0.0;

    for chord in chords {
        add_chord_events(&mut events, chord, tuning, cursor, 1.8);
        cursor += 1.8;
    }

    let pitch_space = chromatic_pitch_space(-1, 0);
    events.push(NoteEvent {
        pitch_space,
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
        let pitch_space =
            chromatic_pitch_space(pair.chord.octave_shift, pair.chord.root_pc + pair.offset);
        let recursive = note_frequency(RECURSIVE_JUST, pair.chord, pair.offset);
        let fixed = note_frequency(FIXED_C_JUST, pair.chord, pair.offset);
        events.push(NoteEvent {
            pitch_space,
            frequency: fixed,
            start: cursor,
            duration: 0.85,
            amplitude: 0.4,
        });
        events.push(NoteEvent {
            pitch_space,
            frequency: recursive,
            start: cursor + 0.9,
            duration: 0.85,
            amplitude: 0.4,
        });
        events.push(NoteEvent {
            pitch_space,
            frequency: fixed,
            start: cursor + 1.9,
            duration: 1.2,
            amplitude: 0.22,
        });
        events.push(NoteEvent {
            pitch_space,
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
        for offset in chord.offsets() {
            for tuning in all_tunings() {
                let frequency = note_frequency(tuning, chord, *offset);
                let tet = note_frequency(TWELVE_TET, chord, *offset);
                let fixed = note_frequency(FIXED_C_JUST, chord, *offset);
                body.push_str(&format!(
                    "{},{},{},{:.3},{},{}\n",
                    tuning_label(tuning),
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
    tuning: TuningSystem,
    start: f64,
    duration: f64,
) {
    let offsets = chord.offsets();

    for (index, offset) in offsets.iter().enumerate() {
        let pitch_space = chromatic_pitch_space(chord.octave_shift, chord.root_pc + *offset);
        events.push(NoteEvent {
            pitch_space,
            frequency: note_frequency(tuning, chord, *offset),
            start: start + index as f64 * 0.15,
            duration: duration - index as f64 * 0.08,
            amplitude: 0.28,
        });
    }

    for offset in offsets {
        let pitch_space = chromatic_pitch_space(chord.octave_shift, chord.root_pc + offset + 12);
        events.push(NoteEvent {
            pitch_space,
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
    let names = match chord.name {
        "C" => &["C4", "E4", "G4"][..],
        "E" => &["E4", "G#4", "B4"][..],
        "G#/Ab" => &["A-4", "C5", "E-5"][..],
        "F" => &["F4", "A4", "C5"][..],
        "A" => &["A4", "C#5", "E5"][..],
        "D" => &["D4", "F#4", "A4"][..],
        "G7" => &["G3", "B3", "D4", "F4"][..],
        _ => &["C4", "E4", "G4"][..],
    };

    names
        .iter()
        .map(|name| Pitch::from_name(*name).map_err(|err| err.into()))
        .collect()
}

fn chord_context_label(chord: Chord) -> String {
    if chord.intervals == DOMINANT_INTERVALS {
        chord.name.to_string()
    } else {
        format!("{} major", notation_chord_label(chord))
    }
}

fn notation_chord_label(chord: Chord) -> &'static str {
    match chord.name {
        "G#/Ab" => "Ab",
        _ => chord.name,
    }
}

fn note_frequency(tuning: TuningSystem, chord: Chord, offset: i32) -> f64 {
    match tuning {
        TWELVE_TET => EQUAL_TEMPERAMENT.frequency_at(chromatic_pitch_space(
            chord.octave_shift,
            chord.root_pc + offset,
        )),
        FIXED_C_JUST => fixed_c_frequency(chord.octave_shift, chord.root_pc + offset),
        RECURSIVE_JUST => {
            let root = fixed_c_frequency(chord.octave_shift, chord.root_pc);
            root * just_ratio_for_interval(offset)
        }
        TWELVE_TET_ROOTED_JUST => {
            let root = EQUAL_TEMPERAMENT
                .frequency_at(chromatic_pitch_space(chord.octave_shift, chord.root_pc));
            root * just_ratio_for_interval(offset)
        }
        _ => unreachable!("unsupported tuning system for note frequency"),
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
        let recursive = note_frequency(RECURSIVE_JUST, chord, 4);
        let fixed = note_frequency(FIXED_C_JUST, chord, 4);

        assert!((cents_between(recursive, fixed) + 34.282).abs() < 0.01);
    }

    #[test]
    fn twelve_tet_rooted_just_keeps_tet_roots_and_just_intervals() {
        let chord = Chord::major("E", 4);
        let root = note_frequency(TWELVE_TET_ROOTED_JUST, chord, 0);
        let tet_root = note_frequency(TWELVE_TET, chord, 0);
        let third = note_frequency(TWELVE_TET_ROOTED_JUST, chord, 4);

        assert!((cents_between(root, tet_root)).abs() < 0.001);
        assert!((third / root - 5.0 / 4.0).abs() < 0.000_001);
    }

    #[test]
    fn renders_audio() {
        let samples = render_progression(RECURSIVE_JUST, ToneColor::Harmonic);

        assert!(samples.len() > SAMPLE_RATE as usize);
        assert!(samples.iter().any(|sample| sample.abs() > 0.01));
    }

    #[test]
    fn renders_pure_sine_progression() {
        let samples = render_progression(RECURSIVE_JUST, ToneColor::PureSine);

        assert!(samples.len() > SAMPLE_RATE as usize);
        assert!(samples.iter().any(|sample| sample.abs() > 0.01));
    }

    #[test]
    fn renders_c_drone_progression() {
        let samples = render_c_drone_progression(RECURSIVE_JUST);

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
