use std::f64::consts::TAU;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::{Result, Site, SiteError};

const SAMPLE_RATE: u32 = 44_100;
const C4_HZ: f64 = 261.625_565_300_598_6;
const DEFAULT_OUTPUT_DIR: &str = "content/blog/recursive-just-intonation";
const NOTE_NAMES: [&str; 12] = [
    "C", "C#/Db", "D", "D#/Eb", "E", "F", "F#/Gb", "G", "G#/Ab", "A", "A#/Bb", "B",
];
const JUST_C_SCALE: [Ratio; 12] = [
    Ratio::new(1, 1),
    Ratio::new(17, 16),
    Ratio::new(9, 8),
    Ratio::new(19, 16),
    Ratio::new(5, 4),
    Ratio::new(4, 3),
    Ratio::new(45, 32),
    Ratio::new(3, 2),
    Ratio::new(51, 32),
    Ratio::new(27, 16),
    Ratio::new(57, 32),
    Ratio::new(15, 8),
];

#[derive(Clone, Copy)]
struct Ratio {
    numerator: u32,
    denominator: u32,
}

#[derive(Clone, Copy)]
enum ChordKind {
    Major,
    Dominant,
}

#[derive(Clone, Copy)]
enum Tuning {
    TwelveTet,
    FixedCJust,
    RecursiveJust,
}

#[derive(Clone, Copy)]
struct Chord {
    name: &'static str,
    root_pc: i32,
    octave_shift: i32,
    kind: ChordKind,
}

#[derive(Clone, Copy)]
struct NoteEvent {
    frequency: f64,
    start: f64,
    duration: f64,
    amplitude: f64,
}

impl Ratio {
    const fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn as_f64(self) -> f64 {
        f64::from(self.numerator) / f64::from(self.denominator)
    }
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
    fn file_stem(self) -> &'static str {
        match self {
            Tuning::TwelveTet => "twelve-tet-progression",
            Tuning::FixedCJust => "fixed-c-ji-progression",
            Tuning::RecursiveJust => "recursive-ji-progression",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Tuning::TwelveTet => "12-TET",
            Tuning::FixedCJust => "Fixed C just intonation",
            Tuning::RecursiveJust => "Recursive just intonation",
        }
    }
}

pub(crate) fn generate(site: &Site, args: &[String]) -> Result<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return Ok(());
    }

    if args.len() > 1 {
        return Err(Box::new(SiteError::new(
            "recursive-ji-music accepts at most one output directory",
        )));
    }

    let output_dir = args.first().map_or_else(
        || site.root.join(DEFAULT_OUTPUT_DIR),
        |path| site.root.join(path),
    );
    fs::create_dir_all(&output_dir)?;

    for tuning in [Tuning::TwelveTet, Tuning::FixedCJust, Tuning::RecursiveJust] {
        let samples = render_progression(tuning);
        let path = output_dir.join(format!("{}.wav", tuning.file_stem()));
        write_wav(&path, &samples)?;
        println!("wrote {}", relative_to_root(site, &path).display());
    }

    let split_samples = render_note_splits();
    let split_path = output_dir.join("recursive-ji-note-splits.wav");
    write_wav(&split_path, &split_samples)?;
    println!("wrote {}", relative_to_root(site, &split_path).display());

    let csv_path = output_dir.join("recursive-ji-frequencies.csv");
    fs::write(&csv_path, frequency_report())?;
    println!("wrote {}", relative_to_root(site, &csv_path).display());

    Ok(())
}

fn render_progression(tuning: Tuning) -> Vec<f32> {
    let chords = progression();
    let mut events = Vec::new();
    let mut cursor = 0.0;

    for chord in chords {
        add_chord_events(&mut events, chord, tuning, cursor, 1.8);
        cursor += 1.8;
    }

    synthesize(&events, cursor + 0.4)
}

fn render_note_splits() -> Vec<f32> {
    let pairs = [
        ("G#/Ab", Chord::major("E", 4), 4),
        ("A", Chord::major("F", 5), 4),
        ("C#/Db", Chord::major("A", 9), 4),
        ("F", Chord::dominant("G7", 7), 10),
    ];
    let mut events = Vec::new();
    let mut cursor = 0.0;

    for (_name, chord, offset) in pairs {
        let recursive = note_frequency(Tuning::RecursiveJust, chord, offset);
        let fixed = note_frequency(Tuning::FixedCJust, chord, offset);
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

    synthesize(&events, cursor + 0.4)
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

fn note_frequency(tuning: Tuning, chord: Chord, offset: i32) -> f64 {
    match tuning {
        Tuning::TwelveTet => {
            let semitones_from_c4 = chord.octave_shift * 12 + chord.root_pc + offset;
            C4_HZ * 2.0_f64.powf(f64::from(semitones_from_c4) / 12.0)
        }
        Tuning::FixedCJust => fixed_c_frequency(chord.octave_shift, chord.root_pc + offset),
        Tuning::RecursiveJust => {
            let root = fixed_c_frequency(chord.octave_shift, chord.root_pc);
            root * just_ratio_for_interval(offset)
        }
    }
}

fn fixed_c_frequency(octave_shift: i32, absolute_index: i32) -> f64 {
    let octave = octave_shift + absolute_index.div_euclid(12);
    let pc = absolute_index.rem_euclid(12) as usize;
    C4_HZ * JUST_C_SCALE[pc].as_f64() * 2.0_f64.powi(octave)
}

fn just_ratio_for_interval(offset: i32) -> f64 {
    let octave = offset.div_euclid(12);
    let pc = offset.rem_euclid(12) as usize;
    JUST_C_SCALE[pc].as_f64() * 2.0_f64.powi(octave)
}

fn synthesize(events: &[NoteEvent], duration: f64) -> Vec<f32> {
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
            let tone = phase.sin() * 0.78 + (phase * 2.0).sin() * 0.17 + (phase * 3.0).sin() * 0.05;
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

fn write_wav(path: &Path, samples: &[f32]) -> Result<()> {
    let mut file = fs::File::create(path)?;
    let channels = 1_u16;
    let bits_per_sample = 16_u16;
    let block_align = channels * bits_per_sample / 8;
    let byte_rate = SAMPLE_RATE * u32::from(block_align);
    let data_len = samples.len() as u32 * u32::from(block_align);

    file.write_all(b"RIFF")?;
    file.write_all(&(36 + data_len).to_le_bytes())?;
    file.write_all(b"WAVE")?;
    file.write_all(b"fmt ")?;
    file.write_all(&16_u32.to_le_bytes())?;
    file.write_all(&1_u16.to_le_bytes())?;
    file.write_all(&channels.to_le_bytes())?;
    file.write_all(&SAMPLE_RATE.to_le_bytes())?;
    file.write_all(&byte_rate.to_le_bytes())?;
    file.write_all(&block_align.to_le_bytes())?;
    file.write_all(&bits_per_sample.to_le_bytes())?;
    file.write_all(b"data")?;
    file.write_all(&data_len.to_le_bytes())?;

    for sample in samples {
        let pcm = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)).round() as i16;
        file.write_all(&pcm.to_le_bytes())?;
    }

    Ok(())
}

fn frequency_report() -> String {
    let mut body =
        String::from("tuning,chord,note,frequency_hz,cents_vs_12_tet,cents_vs_fixed_c_ji\n");

    for chord in progression() {
        for offset in chord.kind.offsets() {
            for tuning in [Tuning::TwelveTet, Tuning::FixedCJust, Tuning::RecursiveJust] {
                let frequency = note_frequency(tuning, chord, *offset);
                let tet = note_frequency(Tuning::TwelveTet, chord, *offset);
                let fixed = note_frequency(Tuning::FixedCJust, chord, *offset);
                body.push_str(&format!(
                    "{},{},{},{:.3},{:.3},{:.3}\n",
                    tuning.label(),
                    chord.name,
                    NOTE_NAMES[(chord.root_pc + *offset).rem_euclid(12) as usize],
                    frequency,
                    cents_between(frequency, tet),
                    cents_between(frequency, fixed),
                ));
            }
        }
    }

    body
}

fn cents_between(a: f64, b: f64) -> f64 {
    1200.0 * (a / b).log2()
}

fn relative_to_root(site: &Site, path: &Path) -> PathBuf {
    path.strip_prefix(&site.root).unwrap_or(path).to_path_buf()
}

fn print_help() {
    println!(
        "\
recursive-ji-music

Usage:
  cargo run --manifest-path tools/site/Cargo.toml -- recursive-ji-music [OUTPUT]

Defaults:
  OUTPUT  content/blog/recursive-just-intonation

Writes:
  twelve-tet-progression.wav
  fixed-c-ji-progression.wav
  recursive-ji-progression.wav
  recursive-ji-note-splits.wav
  recursive-ji-frequencies.csv
"
    );
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
    fn renders_audio() {
        let samples = render_progression(Tuning::RecursiveJust);

        assert!(samples.len() > SAMPLE_RATE as usize);
        assert!(samples.iter().any(|sample| sample.abs() > 0.01));
    }
}
