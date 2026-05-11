use std::collections::HashMap;
use std::f64::consts::TAU;
use std::io::Write;

use music21_rs::tuningsystem::TWELVE_TONE_NAMES;
use music21_rs::TuningSystem;

use crate::{GeneratedBinary, GeneratedText, Result, SAMPLE_RATE};

const MIDDLE_C_PITCH_SPACE: f64 = 60.0;
const JUST_INTONATION: TuningSystem = TuningSystem::JustIntonation;
const EQUAL_TEMPERAMENT: TuningSystem = TuningSystem::EqualTemperament { octave_size: 12 };
const MOZART_DIES_IRAE_MIDI: &[u8] = include_bytes!("../../../content/misc/blobs/jm_mozdi.mid");
const MASTERPIECE_TITLE: &str = "Twelve Rooms for One Piano";
const MASTERPIECE_AUDIO_FILE: &str = "recursive-just-intonation-composition.wav";
const MASTERPIECE_MUSICXML_FILE: &str = "recursive-just-intonation-composition.musicxml";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum LongToneColor {
    Piano,
    Pad,
    Bell,
}

#[derive(Clone, Copy, Debug)]
struct LongEvent {
    start: f64,
    duration: f64,
    midi_note: i32,
    root_pc: i32,
    amplitude: f64,
    pan: f64,
    tone_color: LongToneColor,
}

#[derive(Clone, Copy, Debug)]
struct MidiNote {
    start_tick: u32,
    end_tick: u32,
    note: u8,
    velocity: u8,
    track: usize,
}

#[derive(Clone, Copy, Debug)]
struct CompositionSection {
    root_pc: i32,
    bass: i32,
    tones: &'static [i32],
    melody: &'static [i32],
    beats: usize,
    energy: f64,
}

#[derive(Clone, Copy, Debug)]
struct RhythmPulse {
    offset_ticks: u8,
    duration_ticks: u8,
    accent: f64,
}

const RHYTHM_DIVISIONS: i32 = 12;

const RHYTHM_GRAVE: [RhythmPulse; 5] = [
    RhythmPulse {
        offset_ticks: 0,
        duration_ticks: 18,
        accent: 1.16,
    },
    RhythmPulse {
        offset_ticks: 18,
        duration_ticks: 6,
        accent: 0.72,
    },
    RhythmPulse {
        offset_ticks: 24,
        duration_ticks: 9,
        accent: 0.92,
    },
    RhythmPulse {
        offset_ticks: 33,
        duration_ticks: 3,
        accent: 0.70,
    },
    RhythmPulse {
        offset_ticks: 36,
        duration_ticks: 12,
        accent: 1.04,
    },
];

const RHYTHM_SURGE: [RhythmPulse; 5] = [
    RhythmPulse {
        offset_ticks: 0,
        duration_ticks: 9,
        accent: 0.92,
    },
    RhythmPulse {
        offset_ticks: 9,
        duration_ticks: 3,
        accent: 0.68,
    },
    RhythmPulse {
        offset_ticks: 12,
        duration_ticks: 12,
        accent: 1.12,
    },
    RhythmPulse {
        offset_ticks: 27,
        duration_ticks: 6,
        accent: 0.78,
    },
    RhythmPulse {
        offset_ticks: 36,
        duration_ticks: 12,
        accent: 1.02,
    },
];

const RHYTHM_PROCESSION: [RhythmPulse; 5] = [
    RhythmPulse {
        offset_ticks: 0,
        duration_ticks: 6,
        accent: 0.78,
    },
    RhythmPulse {
        offset_ticks: 6,
        duration_ticks: 6,
        accent: 0.82,
    },
    RhythmPulse {
        offset_ticks: 12,
        duration_ticks: 18,
        accent: 1.14,
    },
    RhythmPulse {
        offset_ticks: 33,
        duration_ticks: 3,
        accent: 0.66,
    },
    RhythmPulse {
        offset_ticks: 36,
        duration_ticks: 12,
        accent: 1.08,
    },
];

const RHYTHM_SUSPENSION: [RhythmPulse; 4] = [
    RhythmPulse {
        offset_ticks: 0,
        duration_ticks: 24,
        accent: 1.18,
    },
    RhythmPulse {
        offset_ticks: 27,
        duration_ticks: 6,
        accent: 0.76,
    },
    RhythmPulse {
        offset_ticks: 33,
        duration_ticks: 3,
        accent: 0.66,
    },
    RhythmPulse {
        offset_ticks: 39,
        duration_ticks: 9,
        accent: 0.94,
    },
];

const MASTERPIECE_SECTIONS: [CompositionSection; 12] = [
    CompositionSection {
        root_pc: 2,
        bass: 38,
        tones: &[0, 3, 7, 10, 14],
        melody: &[62, 61, 62, 57, 60, 55, 57, 53, 55, 57, 60, 62],
        beats: 12,
        energy: 0.48,
    },
    CompositionSection {
        root_pc: 9,
        bass: 33,
        tones: &[0, 3, 7, 10, 13],
        melody: &[64, 62, 61, 57, 55, 57, 60, 62, 64, 67, 65, 64],
        beats: 12,
        energy: 0.56,
    },
    CompositionSection {
        root_pc: 5,
        bass: 41,
        tones: &[0, 4, 7, 11, 14],
        melody: &[65, 64, 65, 69, 72, 69, 67, 65, 64, 60, 62, 65],
        beats: 12,
        energy: 0.62,
    },
    CompositionSection {
        root_pc: 0,
        bass: 36,
        tones: &[0, 3, 7, 10, 14],
        melody: &[
            67, 65, 64, 60, 62, 64, 67, 71, 72, 71, 67, 64, 62, 60, 59, 60,
        ],
        beats: 16,
        energy: 0.66,
    },
    CompositionSection {
        root_pc: 7,
        bass: 43,
        tones: &[0, 3, 7, 10, 14],
        melody: &[67, 65, 62, 59, 62, 65, 67, 70, 74, 72, 70, 67],
        beats: 12,
        energy: 0.68,
    },
    CompositionSection {
        root_pc: 10,
        bass: 34,
        tones: &[0, 4, 7, 10, 14],
        melody: &[70, 69, 65, 62, 65, 69, 72, 74, 77, 74, 72, 70],
        beats: 12,
        energy: 0.72,
    },
    CompositionSection {
        root_pc: 4,
        bass: 40,
        tones: &[0, 3, 6, 10, 13],
        melody: &[76, 74, 72, 67, 69, 72, 76, 79, 81, 79, 76, 74],
        beats: 12,
        energy: 0.76,
    },
    CompositionSection {
        root_pc: 9,
        bass: 33,
        tones: &[0, 3, 7, 10, 15],
        melody: &[
            76, 77, 81, 79, 76, 72, 69, 67, 69, 72, 76, 79, 81, 79, 76, 72,
        ],
        beats: 16,
        energy: 0.86,
    },
    CompositionSection {
        root_pc: 2,
        bass: 38,
        tones: &[0, 3, 7, 10, 14],
        melody: &[74, 72, 69, 65, 67, 69, 72, 74, 77, 76, 74, 72],
        beats: 12,
        energy: 0.80,
    },
    CompositionSection {
        root_pc: 11,
        bass: 35,
        tones: &[0, 3, 7, 10, 14],
        melody: &[79, 77, 74, 71, 67, 66, 67, 71, 74, 77, 79, 83],
        beats: 12,
        energy: 0.68,
    },
    CompositionSection {
        root_pc: 7,
        bass: 43,
        tones: &[0, 3, 7, 10, 14],
        melody: &[77, 74, 72, 67, 65, 62, 59, 62, 65, 67, 70, 74],
        beats: 12,
        energy: 0.58,
    },
    CompositionSection {
        root_pc: 2,
        bass: 38,
        tones: &[0, 3, 7, 10, 14],
        melody: &[
            74, 72, 69, 65, 62, 61, 62, 57, 60, 55, 57, 53, 50, 53, 57, 62,
        ],
        beats: 16,
        energy: 0.52,
    },
];

pub(crate) fn generated_audio_files() -> Result<Vec<GeneratedBinary>> {
    Ok(vec![
        GeneratedBinary {
            name: MASTERPIECE_AUDIO_FILE,
            bytes: render_long_events_to_wav(&build_masterpiece_events(), 4.0, 0.78, 0.30)?,
        },
        GeneratedBinary {
            name: "mozart-dies-irae-recursive-just-intonation-piano.wav",
            bytes: render_long_events_to_wav(&build_mozart_dies_irae_events()?, 3.0, 0.56, 0.16)?,
        },
    ])
}

pub(crate) fn generated_media_text_files() -> Vec<GeneratedText> {
    vec![GeneratedText {
        name: MASTERPIECE_MUSICXML_FILE,
        text: build_masterpiece_musicxml(),
    }]
}

fn build_masterpiece_events() -> Vec<LongEvent> {
    let mut events = Vec::new();
    let beat = 60.0 / 68.0;

    let mut cursor = 0.0;
    add_intro_resonance(&mut events, beat);
    for (section_index, section) in MASTERPIECE_SECTIONS.iter().enumerate() {
        add_composition_section(&mut events, cursor, beat, section_index, *section);
        cursor += section.beats as f64 * beat;
    }

    add_masterpiece_coda(&mut events, cursor, beat);
    events
}

fn add_intro_resonance(events: &mut Vec<LongEvent>, beat: f64) {
    for (index, (midi_note, amplitude, pan)) in [
        (26, 0.22, -0.32),
        (38, 0.25, -0.22),
        (50, 0.18, -0.10),
        (57, 0.14, 0.08),
        (62, 0.12, 0.18),
        (65, 0.09, 0.26),
        (69, 0.07, 0.34),
    ]
    .into_iter()
    .enumerate()
    {
        events.push(LongEvent {
            start: index as f64 * beat * 0.22,
            duration: beat * (6.2 - index as f64 * 0.18),
            midi_note,
            root_pc: 2,
            amplitude,
            pan,
            tone_color: LongToneColor::Pad,
        });
    }
}

fn add_composition_section(
    events: &mut Vec<LongEvent>,
    start: f64,
    beat: f64,
    section_index: usize,
    section: CompositionSection,
) {
    add_bass_ostinato(events, start, beat, section);
    add_pad_chords(events, start, beat, section_index, section);
    add_inner_arpeggios(events, start, beat, section_index, section);
    add_melody(events, start, beat, section_index, section);
    add_counter_line(events, start, beat, section_index, section);
    add_recursive_branch(
        events,
        start + section.beats as f64 * beat * 0.56,
        beat * 3.6,
        section.root_pc,
        fold_into_register(
            section.bass + 24 + section.tones[section_index % section.tones.len()],
            50,
            82,
        ),
        2,
        0.09 + section.energy * 0.055,
        0.34,
    );
    add_transition_bells(events, start, beat, section_index, section);
}

fn add_bass_ostinato(
    events: &mut Vec<LongEvent>,
    start: f64,
    beat: f64,
    section: CompositionSection,
) {
    for phrase in 0..section.beats.div_ceil(4) {
        let phrase_start = start + phrase as f64 * 4.0 * beat;
        let pattern = if phrase % 2 == 0 {
            [
                (0.0, 0, 1.18, 0.46),
                (1.5, 7, 0.36, 0.20),
                (1.75, 12, 0.28, 0.17),
                (2.0, 0, 0.84, 0.38),
                (2.75, section.tones[3].rem_euclid(12), 0.32, 0.18),
                (3.25, 7, 0.52, 0.22),
            ]
        } else {
            [
                (0.0, 0, 0.82, 0.42),
                (0.75, 12, 0.28, 0.16),
                (1.25, 7, 0.48, 0.21),
                (2.0, 0, 1.18, 0.40),
                (3.0, section.tones[1].rem_euclid(12), 0.34, 0.16),
                (3.5, 12, 0.42, 0.20),
            ]
        };

        for (offset_beats, interval, duration_beats, amplitude) in pattern {
            if phrase * 4 + offset_beats as usize >= section.beats {
                continue;
            }

            events.push(LongEvent {
                start: phrase_start + offset_beats * beat,
                duration: duration_beats * beat,
                midi_note: section.bass + interval,
                root_pc: section.root_pc,
                amplitude: amplitude * (0.82 + section.energy * 0.34),
                pan: -0.42,
                tone_color: LongToneColor::Piano,
            });
        }
    }
}

fn add_pad_chords(
    events: &mut Vec<LongEvent>,
    start: f64,
    beat: f64,
    section_index: usize,
    section: CompositionSection,
) {
    for phrase in 0..section.beats.div_ceil(4) {
        let phrase_start = start + phrase as f64 * 4.0 * beat;
        let remaining_beats = section.beats.saturating_sub(phrase * 4).min(4) as f64;
        for (voice, offset) in section.tones.iter().enumerate() {
            let register = if voice < 2 { 12 } else { 24 };
            events.push(LongEvent {
                start: phrase_start + voice as f64 * beat * 0.035,
                duration: beat * (remaining_beats + 0.42),
                midi_note: fold_into_register(section.bass + register + *offset, 43, 78),
                root_pc: section.root_pc,
                amplitude: (0.085 + section.energy * 0.060) / (voice as f64 + 1.0).sqrt(),
                pan: -0.20 + voice as f64 * 0.12,
                tone_color: LongToneColor::Pad,
            });
        }

        if section.energy > 0.54 && (phrase + section_index) % 2 == 0 {
            events.push(LongEvent {
                start: phrase_start + beat * 2.72,
                duration: beat * 2.4,
                midi_note: fold_into_register(
                    section.bass + 24 + section.tones[(phrase + 1) % section.tones.len()],
                    55,
                    80,
                ),
                root_pc: section.root_pc,
                amplitude: 0.038 + section.energy * 0.034,
                pan: 0.42,
                tone_color: LongToneColor::Bell,
            });
        }
    }
}

fn add_inner_arpeggios(
    events: &mut Vec<LongEvent>,
    start: f64,
    beat: f64,
    section_index: usize,
    section: CompositionSection,
) {
    let mut arpeggio = Vec::with_capacity(section.tones.len() * 3);
    for octave in [12, 24, 36] {
        for offset in section.tones {
            arpeggio.push(fold_into_register(section.bass + octave + *offset, 45, 80));
        }
    }

    let step = beat / 3.0;
    for index in 0..section.beats * 3 {
        if (index + section_index) % 18 == 17 && section.energy < 0.78 {
            continue;
        }

        let stride = if section_index % 3 == 0 { 4 } else { 5 };
        let contour = match index % 12 {
            0 | 5 | 9 => 0,
            1 | 6 => 2,
            2 | 7 | 10 => 4,
            3 | 8 => 1,
            _ => 3,
        };
        let note_index = (index / 3 * stride + contour + section_index) % arpeggio.len();
        let pan = if index % 3 == 0 { 0.12 } else { 0.28 };
        events.push(LongEvent {
            start: start + index as f64 * step + (section_index % 2) as f64 * beat * 0.035,
            duration: beat * if index % 6 == 5 { 0.48 } else { 0.34 },
            midi_note: arpeggio[note_index],
            root_pc: section.root_pc,
            amplitude: 0.054 + section.energy * 0.060,
            pan,
            tone_color: LongToneColor::Piano,
        });
    }
}

fn add_melody(
    events: &mut Vec<LongEvent>,
    start: f64,
    beat: f64,
    section_index: usize,
    section: CompositionSection,
) {
    let mut note_cursor = 0;
    for phrase in 0..section.beats.div_ceil(4) {
        let phrase_start = start + phrase as f64 * 4.0 * beat;
        let cell = melody_rhythm_cell(section_index, phrase);

        for (pulse_index, pulse) in cell.iter().enumerate() {
            let beat_position = phrase as f64 * 4.0 + ticks_to_beats(pulse.offset_ticks);
            if beat_position >= section.beats as f64 {
                continue;
            }

            let cadence = pulse_index + 1 == cell.len()
                || beat_position + ticks_to_beats(pulse.duration_ticks) >= section.beats as f64;
            let midi_note = section.melody[note_cursor % section.melody.len()];
            note_cursor += 1;

            events.push(LongEvent {
                start: phrase_start + ticks_to_beats(pulse.offset_ticks) * beat,
                duration: ticks_to_beats(pulse.duration_ticks)
                    * beat
                    * if cadence { 1.18 } else { 0.92 },
                midi_note,
                root_pc: section.root_pc,
                amplitude: (if cadence {
                    0.30 + section.energy * 0.18
                } else {
                    0.22 + section.energy * 0.14
                }) * pulse.accent,
                pan: 0.06,
                tone_color: LongToneColor::Piano,
            });
        }
    }
}

fn melody_rhythm_cell(section_index: usize, phrase: usize) -> &'static [RhythmPulse] {
    match (section_index + phrase) % 4 {
        0 => &RHYTHM_GRAVE,
        1 => &RHYTHM_SURGE,
        2 => &RHYTHM_PROCESSION,
        _ => &RHYTHM_SUSPENSION,
    }
}

fn ticks_to_beats(ticks: u8) -> f64 {
    f64::from(ticks) / f64::from(RHYTHM_DIVISIONS)
}

fn fold_into_register(mut midi_note: i32, min: i32, max: i32) -> i32 {
    while midi_note > max {
        midi_note -= 12;
    }
    while midi_note < min {
        midi_note += 12;
    }
    midi_note
}

fn add_counter_line(
    events: &mut Vec<LongEvent>,
    start: f64,
    beat: f64,
    section_index: usize,
    section: CompositionSection,
) {
    let delay = if section_index % 2 == 0 { 1.5 } else { 2.25 };
    for (index, midi_note) in section.melody.iter().rev().step_by(2).enumerate() {
        let beat_position = delay + index as f64 * if index % 3 == 2 { 1.5 } else { 2.0 };
        if beat_position >= section.beats as f64 {
            break;
        }

        events.push(LongEvent {
            start: start + beat_position * beat,
            duration: beat * if index % 3 == 2 { 0.86 } else { 1.28 },
            midi_note: *midi_note - 12,
            root_pc: section.root_pc,
            amplitude: (0.09 + section.energy * 0.070) * (1.0 - index as f64 * 0.035).max(0.60),
            pan: -0.04,
            tone_color: if section.energy > 0.78 {
                LongToneColor::Bell
            } else {
                LongToneColor::Piano
            },
        });
    }
}

fn add_recursive_branch(
    events: &mut Vec<LongEvent>,
    start: f64,
    span: f64,
    root_pc: i32,
    midi_note: i32,
    depth: usize,
    amplitude: f64,
    pan: f64,
) {
    if depth == 0 || span < 0.12 {
        return;
    }

    let midi_note = fold_into_register(midi_note, 48, 83);
    events.push(LongEvent {
        start,
        duration: span * 0.72,
        midi_note,
        root_pc,
        amplitude,
        pan,
        tone_color: LongToneColor::Bell,
    });

    let child_span = span * 0.48;
    for (branch, interval) in [7, 4, 12].into_iter().enumerate() {
        let child_start = start + span * (0.28 + branch as f64 * 0.18);
        let fold = if branch == 2 && depth % 2 == 0 {
            -12
        } else {
            0
        };
        add_recursive_branch(
            events,
            child_start,
            child_span,
            root_pc,
            fold_into_register(midi_note + interval + fold, 48, 83),
            depth - 1,
            amplitude * (0.58 - branch as f64 * 0.06),
            (pan - 0.18 + branch as f64 * 0.18).clamp(-0.60, 0.60),
        );
    }
}

fn add_transition_bells(
    events: &mut Vec<LongEvent>,
    start: f64,
    beat: f64,
    section_index: usize,
    section: CompositionSection,
) {
    let final_bar = start + (section.beats as f64 - 1.75).max(0.0) * beat;
    for (index, offset) in section
        .tones
        .iter()
        .cycle()
        .skip(section_index)
        .take(4)
        .enumerate()
    {
        events.push(LongEvent {
            start: final_bar + index as f64 * beat * 0.34,
            duration: beat * (1.16 - index as f64 * 0.10),
            midi_note: fold_into_register(
                section.bass + 24 + *offset + if index > 1 { 12 } else { 0 },
                54,
                82,
            ),
            root_pc: section.root_pc,
            amplitude: 0.060 + section.energy * 0.050,
            pan: 0.10 + index as f64 * 0.11,
            tone_color: LongToneColor::Bell,
        });
    }
}

fn add_masterpiece_coda(events: &mut Vec<LongEvent>, coda_start: f64, beat: f64) {
    for (index, midi_note) in [84, 81, 79, 77, 74, 72, 69, 65, 62, 60, 57, 53, 50, 45]
        .into_iter()
        .enumerate()
    {
        events.push(LongEvent {
            start: coda_start + index as f64 * beat * 0.58,
            duration: beat * if index < 4 { 1.38 } else { 1.05 },
            midi_note,
            root_pc: 2,
            amplitude: 0.23 + (14 - index).max(1) as f64 * 0.010,
            pan: 0.22 - index as f64 * 0.032,
            tone_color: if index < 3 {
                LongToneColor::Bell
            } else {
                LongToneColor::Piano
            },
        });
    }

    for (midi_note, pan, amplitude) in [
        (26, -0.38, 0.20),
        (38, -0.28, 0.30),
        (50, -0.16, 0.25),
        (57, -0.02, 0.19),
        (62, 0.10, 0.17),
        (65, 0.20, 0.12),
        (69, 0.30, 0.10),
        (74, 0.38, 0.07),
    ] {
        events.push(LongEvent {
            start: coda_start + 7.6 * beat,
            duration: beat * 8.4,
            midi_note,
            root_pc: 2,
            amplitude,
            pan,
            tone_color: LongToneColor::Pad,
        });
    }
}

fn build_masterpiece_musicxml() -> String {
    let mut score = String::new();
    score.push_str(r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>"#);
    score.push('\n');
    score.push_str(
        r#"<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 3.1 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">"#,
    );
    score.push('\n');
    score.push_str(r#"<score-partwise version="3.1">"#);
    score.push('\n');
    score.push_str(&format!(
        "  <work><work-title>{}</work-title></work>\n",
        escape_xml(MASTERPIECE_TITLE)
    ));
    score.push_str("  <identification>\n");
    score.push_str("    <creator type=\"composer\">Codex with music21-rs</creator>\n");
    score.push_str("    <encoding><software>recursive-ji-core</software></encoding>\n");
    score.push_str("  </identification>\n");
    score.push_str("  <defaults>\n");
    score.push_str("    <scaling><millimeters>7.0</millimeters><tenths>40</tenths></scaling>\n");
    score.push_str("  </defaults>\n");
    score.push_str(&format!(
        "  <credit page=\"1\"><credit-words justify=\"center\" valign=\"top\" font-size=\"22\">{}</credit-words></credit>\n",
        escape_xml(MASTERPIECE_TITLE)
    ));
    score.push_str("  <credit page=\"1\"><credit-words justify=\"center\" valign=\"top\" font-size=\"10\">Written pitch is approximate; the generated WAV retunes every section around the shown recursive just-intonation root.</credit-words></credit>\n");
    score.push_str("  <part-list>\n");
    score.push_str("    <score-part id=\"P1\"><part-name>Melody</part-name></score-part>\n");
    score.push_str(
        "    <score-part id=\"P2\"><part-name>Recursive harmony</part-name></score-part>\n",
    );
    score.push_str("  </part-list>\n");
    push_melody_part_musicxml(&mut score);
    push_harmony_part_musicxml(&mut score);
    score.push_str("</score-partwise>\n");
    score
}

fn push_melody_part_musicxml(score: &mut String) {
    score.push_str("  <part id=\"P1\">\n");
    let mut measure_number = 1;
    let mut first_measure = true;

    for (section_index, section) in MASTERPIECE_SECTIONS.into_iter().enumerate() {
        let mut note_cursor = 0;
        for measure_index in 0..section.beats / 4 {
            push_measure_start(
                score,
                measure_number,
                first_measure,
                section.root_pc,
                "G",
                2,
            );
            let mut cursor_ticks = 0_i32;
            for pulse in melody_rhythm_cell(section_index, measure_index) {
                let offset_ticks = i32::from(pulse.offset_ticks);
                if offset_ticks > cursor_ticks {
                    push_rest_musicxml(score, offset_ticks - cursor_ticks);
                }

                let midi_note = section.melody[note_cursor % section.melody.len()];
                push_note_musicxml(score, midi_note, i32::from(pulse.duration_ticks), false);
                cursor_ticks = offset_ticks + i32::from(pulse.duration_ticks);
                note_cursor += 1;
            }
            if cursor_ticks < 4 * RHYTHM_DIVISIONS {
                push_rest_musicxml(score, 4 * RHYTHM_DIVISIONS - cursor_ticks);
            }
            score.push_str("  </measure>\n");
            measure_number += 1;
            first_measure = false;
        }
    }

    score.push_str("  </part>\n");
}

fn push_harmony_part_musicxml(score: &mut String) {
    score.push_str("  <part id=\"P2\">\n");
    let mut measure_number = 1;
    let mut first_measure = true;

    for section in MASTERPIECE_SECTIONS {
        for _ in 0..section.beats / 4 {
            push_measure_start(
                score,
                measure_number,
                first_measure,
                section.root_pc,
                "F",
                4,
            );
            push_note_musicxml(score, section.bass, 4 * RHYTHM_DIVISIONS, false);
            for offset in section.tones.iter().take(4) {
                push_note_musicxml(
                    score,
                    fold_into_register(section.bass + 24 + *offset, 43, 78),
                    4 * RHYTHM_DIVISIONS,
                    true,
                );
            }
            score.push_str("  </measure>\n");
            measure_number += 1;
            first_measure = false;
        }
    }

    score.push_str("  </part>\n");
}

fn push_measure_start(
    score: &mut String,
    measure_number: usize,
    first_measure: bool,
    root_pc: i32,
    clef_sign: &str,
    clef_line: i32,
) {
    score.push_str(&format!("  <measure number=\"{measure_number}\">\n"));
    if first_measure {
        score.push_str("    <attributes>\n");
        score.push_str(&format!(
            "      <divisions>{RHYTHM_DIVISIONS}</divisions>\n"
        ));
        score.push_str("      <key><fifths>0</fifths></key>\n");
        score.push_str("      <time><beats>4</beats><beat-type>4</beat-type></time>\n");
        score.push_str(&format!(
            "      <clef><sign>{}</sign><line>{clef_line}</line></clef>\n",
            escape_xml(clef_sign)
        ));
        score.push_str("    </attributes>\n");
        score.push_str("    <direction placement=\"above\">\n");
        score.push_str("      <direction-type><metronome><beat-unit>quarter</beat-unit><per-minute>68</per-minute></metronome></direction-type>\n");
        score.push_str("      <sound tempo=\"68\"/>\n");
        score.push_str("    </direction>\n");
    }
    score.push_str("    <direction placement=\"above\">\n");
    score.push_str(&format!(
        "      <direction-type><words font-size=\"9\">RJI root: {}</words></direction-type>\n",
        escape_xml(tone_name(root_pc))
    ));
    score.push_str("    </direction>\n");
}

fn push_note_musicxml(score: &mut String, midi_note: i32, duration_ticks: i32, chord: bool) {
    score.push_str("    <note>\n");
    if chord {
        score.push_str("      <chord/>\n");
    }
    score.push_str(&musicxml_pitch(midi_note));
    push_duration_musicxml(score, duration_ticks);
    score.push_str("    </note>\n");
}

fn push_rest_musicxml(score: &mut String, duration_ticks: i32) {
    score.push_str("    <note>\n");
    score.push_str("      <rest/>\n");
    push_duration_musicxml(score, duration_ticks);
    score.push_str("    </note>\n");
}

fn push_duration_musicxml(score: &mut String, duration_ticks: i32) {
    let (note_type, dotted) = musicxml_duration_type(duration_ticks);
    score.push_str(&format!("      <duration>{duration_ticks}</duration>\n"));
    score.push_str(&format!("      <type>{note_type}</type>\n"));
    if dotted {
        score.push_str("      <dot/>\n");
    }
}

fn musicxml_duration_type(duration_ticks: i32) -> (&'static str, bool) {
    match duration_ticks {
        3 => ("16th", false),
        6 => ("eighth", false),
        9 => ("eighth", true),
        12 => ("quarter", false),
        18 => ("quarter", true),
        24 => ("half", false),
        36 => ("half", true),
        48 => ("whole", false),
        _ => ("quarter", false),
    }
}

fn musicxml_pitch(midi_note: i32) -> String {
    let (step, alter) = match midi_note.rem_euclid(12) {
        0 => ("C", None),
        1 => ("C", Some(1)),
        2 => ("D", None),
        3 => ("E", Some(-1)),
        4 => ("E", None),
        5 => ("F", None),
        6 => ("F", Some(1)),
        7 => ("G", None),
        8 => ("A", Some(-1)),
        9 => ("A", None),
        10 => ("B", Some(-1)),
        11 => ("B", None),
        _ => unreachable!("pitch classes are modulo 12"),
    };
    let octave = midi_note.div_euclid(12) - 1;
    let alter = alter
        .map(|value| format!("        <alter>{value}</alter>\n"))
        .unwrap_or_default();

    format!(
        "      <pitch>\n        <step>{step}</step>\n{alter}        <octave>{octave}</octave>\n      </pitch>\n"
    )
}

fn tone_name(root_pc: i32) -> &'static str {
    TWELVE_TONE_NAMES[root_pc.rem_euclid(12) as usize]
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn build_mozart_dies_irae_events() -> Result<Vec<LongEvent>> {
    let (division, tempos, mut notes) = parse_midi(MOZART_DIES_IRAE_MIDI)?;
    notes.sort_by_key(|note| (note.start_tick, note.note));
    let tick_to_seconds = TickToSeconds::new(division, tempos);

    let mut low_notes = notes
        .iter()
        .copied()
        .filter(|note| note.note <= 59)
        .collect::<Vec<_>>();
    low_notes.sort_by_key(|note| note.start_tick);

    let mut root_timeline = vec![(0_u32, 2_i32)];
    let mut last_tick = i64::MIN / 2;
    let mut last_pc = 2_i32;
    for note in low_notes {
        let pc = i32::from(note.note) % 12;
        if i64::from(note.start_tick) - last_tick >= i64::from(division / 4) || pc != last_pc {
            root_timeline.push((note.start_tick, pc));
            last_tick = i64::from(note.start_tick);
            last_pc = pc;
        }
    }

    let mut events = Vec::with_capacity(notes.len() + 32);
    let mut root_index = 0;
    let max_tick = notes.iter().map(|note| note.end_tick).max().unwrap_or(0);

    for note in notes {
        while root_index + 1 < root_timeline.len()
            && root_timeline[root_index + 1].0 <= note.start_tick
        {
            root_index += 1;
        }

        let start = tick_to_seconds.seconds(note.start_tick);
        let end = tick_to_seconds.seconds(note.end_tick);
        let duration = (end - start).max(0.035);
        let pitch_weight = if note.note < 55 {
            0.98
        } else if note.note < 72 {
            0.78
        } else {
            0.60
        };
        let track_weight = if note.track >= 5 { 0.64 } else { 0.56 };
        let amplitude = (f64::from(note.velocity) / 127.0).powf(1.12) * pitch_weight * track_weight;
        let pan = ((f64::from(note.note) - 62.0) / 62.0).clamp(-0.36, 0.36);

        events.push(LongEvent {
            start,
            duration,
            midi_note: i32::from(note.note),
            root_pc: root_timeline[root_index].1,
            amplitude,
            pan,
            tone_color: LongToneColor::Piano,
        });
    }

    let total_duration = tick_to_seconds.seconds(max_tick);
    let pedal_count = (total_duration / 4.8).floor() as usize + 1;
    for index in 0..pedal_count {
        events.push(LongEvent {
            start: index as f64 * 4.8,
            duration: 4.65,
            midi_note: 38,
            root_pc: 2,
            amplitude: 0.060,
            pan: -0.24,
            tone_color: LongToneColor::Pad,
        });
    }

    Ok(events)
}

fn parse_midi(data: &[u8]) -> Result<(u16, Vec<(u32, u32)>, Vec<MidiNote>)> {
    if data.get(0..4) != Some(b"MThd") {
        return Err("not a MIDI file".into());
    }

    let header_len = read_u32(data, 4)? as usize;
    let track_count = read_u16(data, 10)? as usize;
    let division = read_u16(data, 12)?;
    let mut pos = 8 + header_len;
    let mut tempos = vec![(0, 500_000)];
    let mut notes = Vec::new();

    for track in 0..track_count {
        if data.get(pos..pos + 4) != Some(b"MTrk") {
            return Err(format!("missing MTrk header at byte {pos}").into());
        }
        let length = read_u32(data, pos + 4)? as usize;
        pos += 8;
        let end = pos + length;
        let mut tick = 0_u32;
        let mut running_status = 0_u8;
        let mut active: HashMap<(u8, u8), Vec<(u32, u8)>> = HashMap::new();

        while pos < end {
            let (delta, next_pos) = read_var_len(data, pos)?;
            pos = next_pos;
            tick = tick.saturating_add(delta);

            let mut status = *data
                .get(pos)
                .ok_or_else(|| format!("unexpected end of MIDI track {track}"))?;
            if status & 0x80 != 0 {
                pos += 1;
                if status < 0xF0 {
                    running_status = status;
                }
            } else if running_status != 0 {
                status = running_status;
            } else {
                return Err(
                    format!("running status without previous status in track {track}").into(),
                );
            }

            if status == 0xFF {
                let meta_type = *data
                    .get(pos)
                    .ok_or_else(|| format!("missing meta type in track {track}"))?;
                pos += 1;
                let (len, next_pos) = read_var_len(data, pos)?;
                pos = next_pos;
                let len = len as usize;
                let payload = data
                    .get(pos..pos + len)
                    .ok_or_else(|| format!("truncated meta event in track {track}"))?;
                pos += len;

                if meta_type == 0x51 && payload.len() == 3 {
                    let tempo = u32::from(payload[0]) << 16
                        | u32::from(payload[1]) << 8
                        | u32::from(payload[2]);
                    tempos.push((tick, tempo));
                }
                continue;
            }

            if matches!(status, 0xF0 | 0xF7) {
                let (len, next_pos) = read_var_len(data, pos)?;
                pos = next_pos + len as usize;
                continue;
            }

            let event_type = status & 0xF0;
            let channel = status & 0x0F;
            if matches!(event_type, 0xC0 | 0xD0) {
                pos += 1;
                continue;
            }

            let note = *data
                .get(pos)
                .ok_or_else(|| format!("missing MIDI event byte in track {track}"))?;
            let velocity = *data
                .get(pos + 1)
                .ok_or_else(|| format!("missing MIDI event byte in track {track}"))?;
            pos += 2;

            if event_type == 0x90 && velocity > 0 {
                active
                    .entry((channel, note))
                    .or_default()
                    .push((tick, velocity));
            } else if event_type == 0x80 || (event_type == 0x90 && velocity == 0) {
                if let Some(stack) = active.get_mut(&(channel, note)) {
                    if !stack.is_empty() {
                        let (start_tick, start_velocity) = stack.remove(0);
                        if tick > start_tick {
                            notes.push(MidiNote {
                                start_tick,
                                end_tick: tick,
                                note,
                                velocity: start_velocity,
                                track,
                            });
                        }
                    }
                }
            }
        }

        pos = end;
    }

    tempos.sort_by_key(|(tick, _)| *tick);
    let mut collapsed_tempos = Vec::<(u32, u32)>::with_capacity(tempos.len());
    for (tick, tempo) in tempos {
        if let Some((last_tick, last_tempo)) = collapsed_tempos.last_mut() {
            if *last_tick == tick {
                *last_tempo = tempo;
                continue;
            }
        }
        collapsed_tempos.push((tick, tempo));
    }
    Ok((division, collapsed_tempos, notes))
}

struct TickToSeconds {
    division: u16,
    anchors: Vec<(u32, f64, u32)>,
}

impl TickToSeconds {
    fn new(division: u16, mut tempos: Vec<(u32, u32)>) -> Self {
        tempos.sort_by_key(|(tick, _)| *tick);
        if tempos.first().is_none_or(|(tick, _)| *tick != 0) {
            tempos.insert(0, (0, 500_000));
        }

        let mut anchors = Vec::with_capacity(tempos.len());
        let mut elapsed = 0.0;
        let (mut previous_tick, mut previous_tempo) = tempos[0];
        anchors.push((previous_tick, elapsed, previous_tempo));

        for (tick, tempo) in tempos.into_iter().skip(1) {
            elapsed += f64::from(tick - previous_tick) * f64::from(previous_tempo)
                / 1_000_000.0
                / f64::from(division);
            anchors.push((tick, elapsed, tempo));
            previous_tick = tick;
            previous_tempo = tempo;
        }

        Self { division, anchors }
    }

    fn seconds(&self, tick: u32) -> f64 {
        let index = self
            .anchors
            .partition_point(|(anchor_tick, _, _)| *anchor_tick <= tick)
            .saturating_sub(1);
        let (anchor_tick, anchor_seconds, tempo) = self.anchors[index];
        anchor_seconds
            + f64::from(tick - anchor_tick) * f64::from(tempo)
                / 1_000_000.0
                / f64::from(self.division)
    }
}

fn read_u16(data: &[u8], pos: usize) -> Result<u16> {
    let bytes = data
        .get(pos..pos + 2)
        .ok_or_else(|| format!("truncated u16 at byte {pos}"))?;
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn read_u32(data: &[u8], pos: usize) -> Result<u32> {
    let bytes = data
        .get(pos..pos + 4)
        .ok_or_else(|| format!("truncated u32 at byte {pos}"))?;
    Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn read_var_len(data: &[u8], mut pos: usize) -> Result<(u32, usize)> {
    let mut value = 0_u32;
    loop {
        let byte = *data
            .get(pos)
            .ok_or_else(|| format!("truncated variable-length value at byte {pos}"))?;
        pos += 1;
        value = (value << 7) | u32::from(byte & 0x7F);
        if byte & 0x80 == 0 {
            return Ok((value, pos));
        }
    }
}

fn render_long_events_to_wav(
    events: &[LongEvent],
    tail_seconds: f64,
    gain: f64,
    room_amount: f64,
) -> Result<Vec<u8>> {
    let duration = events
        .iter()
        .map(|event| event.start + event.duration)
        .fold(0.0_f64, f64::max)
        + tail_seconds;
    let sample_count = (duration * f64::from(SAMPLE_RATE)).ceil() as usize;
    let mut left = vec![0.0_f64; sample_count];
    let mut right = vec![0.0_f64; sample_count];

    for event in events {
        synthesize_event(&mut left, &mut right, *event, gain);
    }
    add_room_echo(&mut left, &mut right, room_amount);
    stereo_wav_bytes(&left, &right)
}

fn synthesize_event(left: &mut [f64], right: &mut [f64], event: LongEvent, gain: f64) {
    let frequency = recursive_frequency_from_midi(event.midi_note, event.root_pc);
    if !(0.0..f64::from(SAMPLE_RATE) * 0.47).contains(&frequency) {
        return;
    }

    let start = (event.start * f64::from(SAMPLE_RATE)).max(0.0) as usize;
    if start >= left.len() {
        return;
    }

    let duration_samples = (event.duration * f64::from(SAMPLE_RATE)).max(1.0) as usize;
    let plan = TonePlan::for_color(event.tone_color);
    let attack_samples = (plan.attack * f64::from(SAMPLE_RATE)).max(1.0) as usize;
    let release_samples = (plan.release * f64::from(SAMPLE_RATE)) as usize;
    let end = (start + duration_samples + release_samples).min(left.len());
    let (pan_left, pan_right) = pan_gains(event.pan);
    let amplitude = gain * event.amplitude * register_weight(event.midi_note, event.tone_color)
        / plan.harmonic_gain_sum();
    let mut oscillators = plan
        .harmonics
        .iter()
        .filter_map(|(multiple, harmonic_gain)| {
            let harmonic_frequency = frequency * *multiple;
            if harmonic_frequency >= f64::from(SAMPLE_RATE) * 0.47 {
                return None;
            }

            let phase_seed =
                f64::from((event.midi_note * 37 + (event.start * 1000.0) as i32).rem_euclid(628))
                    / 100.0;
            let step = TAU * harmonic_frequency / f64::from(SAMPLE_RATE);
            Some(Oscillator {
                sin_phase: (phase_seed * *multiple).sin(),
                cos_phase: (phase_seed * *multiple).cos(),
                sin_step: step.sin(),
                cos_step: step.cos(),
                gain: *harmonic_gain,
            })
        })
        .collect::<Vec<_>>();

    if oscillators.is_empty() {
        return;
    }

    let decay_multiplier = (-1.0 / (f64::from(SAMPLE_RATE) * plan.decay)).exp();
    let sustain_samples = duration_samples.saturating_sub(attack_samples);
    let sustain_env = decay_multiplier.powf(sustain_samples as f64);
    let mut env = 0.0;

    for sample_index in start..end {
        let local_index = sample_index - start;
        if local_index < attack_samples {
            let x = local_index as f64 / attack_samples as f64;
            env = x * x * (3.0 - 2.0 * x);
        } else if local_index < duration_samples {
            env *= decay_multiplier;
        } else {
            let release_position =
                (local_index - duration_samples) as f64 / release_samples.max(1) as f64;
            env = sustain_env * (1.0 - release_position).powi(2);
        }

        let mut value = 0.0;
        for oscillator in &mut oscillators {
            value += oscillator.gain * oscillator.sin_phase;
            let next_sin = oscillator.sin_phase * oscillator.cos_step
                + oscillator.cos_phase * oscillator.sin_step;
            let next_cos = oscillator.cos_phase * oscillator.cos_step
                - oscillator.sin_phase * oscillator.sin_step;
            oscillator.sin_phase = next_sin;
            oscillator.cos_phase = next_cos;
        }

        if event.tone_color == LongToneColor::Piano {
            let transient_samples = (0.030 * f64::from(SAMPLE_RATE)) as usize;
            if local_index < transient_samples {
                let t = local_index as f64 / f64::from(SAMPLE_RATE);
                value += 0.028
                    * (TAU * (frequency * 5.0).min(f64::from(SAMPLE_RATE) * 0.45) * t).sin()
                    * (1.0 - local_index as f64 / transient_samples as f64);
            }
        }

        let sample = value * env * amplitude;
        left[sample_index] += sample * pan_left;
        right[sample_index] += sample * pan_right;
    }
}

fn register_weight(midi_note: i32, tone_color: LongToneColor) -> f64 {
    let high = (midi_note - 72).max(0) as f64;
    let base = (1.0 - high * 0.035).clamp(0.48, 1.0);
    if tone_color == LongToneColor::Bell {
        base * 0.82
    } else {
        base
    }
}

struct TonePlan {
    harmonics: &'static [(f64, f64)],
    attack: f64,
    decay: f64,
    release: f64,
}

impl TonePlan {
    fn for_color(color: LongToneColor) -> Self {
        match color {
            LongToneColor::Pad => Self {
                harmonics: &[(1.0, 1.0), (2.0, 0.16), (3.0, 0.045)],
                attack: 0.18,
                decay: 5.2,
                release: 0.92,
            },
            LongToneColor::Bell => Self {
                harmonics: &[(1.0, 1.0), (2.0, 0.12), (2.99, 0.045)],
                attack: 0.024,
                decay: 2.0,
                release: 0.68,
            },
            LongToneColor::Piano => Self {
                harmonics: &[(1.0, 1.0), (2.0, 0.30), (3.0, 0.095)],
                attack: 0.014,
                decay: 2.15,
                release: 0.34,
            },
        }
    }

    fn harmonic_gain_sum(&self) -> f64 {
        self.harmonics.iter().map(|(_, gain)| gain.abs()).sum()
    }
}

struct Oscillator {
    sin_phase: f64,
    cos_phase: f64,
    sin_step: f64,
    cos_step: f64,
    gain: f64,
}

fn add_room_echo(left: &mut [f64], right: &mut [f64], amount: f64) {
    if amount <= 0.0 {
        return;
    }

    for (delay_seconds, delay_gain) in [(0.047, 0.24), (0.083, 0.17), (0.137, 0.11)] {
        let delay = (delay_seconds * f64::from(SAMPLE_RATE)) as usize;
        let gain_left = amount * delay_gain;
        let gain_right = amount * delay_gain * 0.92;

        for index in delay..left.len() {
            let delayed_left = left[index - delay];
            let delayed_right = right[index - delay];
            left[index] += gain_left * (0.82 * delayed_left + 0.18 * delayed_right);
            right[index] += gain_right * (0.82 * delayed_right + 0.18 * delayed_left);
        }
    }
}

fn stereo_wav_bytes(left: &[f64], right: &[f64]) -> Result<Vec<u8>> {
    let peak = left
        .iter()
        .chain(right.iter())
        .copied()
        .map(f64::abs)
        .fold(0.0_f64, f64::max);
    let scale = if peak > 0.0 { 0.92 / peak } else { 1.0 };
    let channels = 2_u16;
    let bits_per_sample = 16_u16;
    let block_align = channels * bits_per_sample / 8;
    let byte_rate = SAMPLE_RATE * u32::from(block_align);
    let data_len = left.len() as u32 * u32::from(block_align);
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

    for (left_sample, right_sample) in left.iter().zip(right) {
        let left_pcm = (left_sample * scale).clamp(-1.0, 1.0) * f64::from(i16::MAX);
        let right_pcm = (right_sample * scale).clamp(-1.0, 1.0) * f64::from(i16::MAX);
        bytes.write_all(&(left_pcm.round() as i16).to_le_bytes())?;
        bytes.write_all(&(right_pcm.round() as i16).to_le_bytes())?;
    }

    Ok(bytes)
}

fn pan_gains(pan: f64) -> (f64, f64) {
    let angle = (pan.clamp(-1.0, 1.0) + 1.0) * std::f64::consts::FRAC_PI_4;
    (angle.cos(), angle.sin())
}

fn recursive_frequency_from_midi(midi_note: i32, root_pc: i32) -> f64 {
    let root = midi_note - (midi_note - root_pc).rem_euclid(12);
    let root_frequency = JUST_INTONATION.frequency_at(MIDDLE_C_PITCH_SPACE + f64::from(root - 60));
    root_frequency * just_ratio_for_interval(midi_note - root)
}

#[allow(dead_code)]
fn fixed_c_frequency(midi_note: i32) -> f64 {
    JUST_INTONATION.frequency_at(MIDDLE_C_PITCH_SPACE + f64::from(midi_note - 60))
}

#[allow(dead_code)]
fn just_ratio_for_interval(offset: i32) -> f64 {
    let root = JUST_INTONATION.frequency_at(MIDDLE_C_PITCH_SPACE);
    let target = JUST_INTONATION.frequency_at(MIDDLE_C_PITCH_SPACE + f64::from(offset));
    target / root
}

#[allow(dead_code)]
fn twelve_tet_frequency(midi_note: i32) -> f64 {
    EQUAL_TEMPERAMENT.frequency_at(MIDDLE_C_PITCH_SPACE + f64::from(midi_note - 60))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn original_composition_has_a_long_recursive_arc() {
        let events = build_masterpiece_events();
        let duration = events
            .iter()
            .map(|event| event.start + event.duration)
            .fold(0.0_f64, f64::max);
        let roots = events
            .iter()
            .map(|event| event.root_pc)
            .collect::<BTreeSet<_>>();
        let bell_count = events
            .iter()
            .filter(|event| event.tone_color == LongToneColor::Bell)
            .count();
        let pad_count = events
            .iter()
            .filter(|event| event.tone_color == LongToneColor::Pad)
            .count();
        let highest_note = events
            .iter()
            .map(|event| event.midi_note)
            .max()
            .unwrap_or_default();

        assert!(duration > 130.0);
        assert!(events.len() > 1_000);
        assert!(roots.len() >= 7);
        assert!((90..150).contains(&bell_count));
        assert!(pad_count > 100);
        assert!(highest_note <= 84);
    }

    #[test]
    fn masterpiece_musicxml_tracks_the_recursive_root_plan() {
        let score = build_masterpiece_musicxml();

        assert!(score.contains(MASTERPIECE_TITLE));
        assert!(score.contains("<part-name>Melody</part-name>"));
        assert!(score.contains("<part-name>Recursive harmony</part-name>"));
        assert!(score.contains("<divisions>12</divisions>"));
        assert!(score.contains("<dot/>"));
        assert!(score.contains("RJI root: D"));
        assert!(score.contains("RJI root: E"));
        assert!(score.contains("RJI root: G"));
    }

    #[test]
    fn recursive_frequency_uses_music21_recursive_tuning() {
        let root = 64;
        let note = 68;
        let expected = fixed_c_frequency(root) * just_ratio_for_interval(note - root);
        let actual = recursive_frequency_from_midi(note, root.rem_euclid(12));

        assert!((actual - expected).abs() < 0.000_001);
    }
}
