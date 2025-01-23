type F = fraction::Fraction;

use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Mutex, Weak},
};

use crate::{defaults::IntegerType, fraction::FractionPow, pitch::pitch::Pitch};

#[derive(Debug, Clone, Copy)]
enum IntervalQuality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

#[derive(Debug, Clone)]
pub(crate) struct Interval {
    pitch_start: Weak<Pitch>,
    pitch_end: Weak<Pitch>,
    implicit_diatonic: bool,
    pub(crate) generic: GenericInterval,
    pub(crate) diatonic: DiatonicInterval,
    pub(crate) chromatic: ChromaticInterval,
    interval_type: IntervalType,
    interval_quality: IntervalQuality,
}

#[derive(Clone, Debug)]
pub(crate) struct DiatonicInterval {
    specifier: String,
    generic: GenericInterval,
}

#[derive(Clone, Debug)]
pub(crate) struct ChromaticInterval {
    pub(crate) semitones: IntegerType,
    pub(crate) simple_directed: IntegerType,
}
#[derive(Clone, Debug)]
pub(crate) struct GenericInterval {
    pub(crate) simple_directed: IntegerType,
}

#[derive(Clone, Debug)]
pub(crate) enum IntervalType {
    Harmonic,
    Melodic,
}

lazy_static! {
    static ref PYTHAGOREAN_CACHE: Mutex<HashMap<String, (Pitch, F)>> = Mutex::new(HashMap::new());
}

impl Interval {
    pub(crate) fn new(pitch_start: Pitch, pitch_end: Pitch) -> Option<Interval> {
        let generic = notes_to_generic(&pitch_start, &pitch_end);
        let chromatic = notes_to_chromatic(&pitch_start, &pitch_end);
        let diatonic = intervals_to_diatonic(&generic, &chromatic);

        Some(Interval {
            pitch_start: std::sync::Arc::downgrade(&std::sync::Arc::new(pitch_start)),
            pitch_end: std::sync::Arc::downgrade(&std::sync::Arc::new(pitch_end)),
            implicit_diatonic: false,
            generic,
            diatonic,
            chromatic,
            interval_type: todo!(),
            interval_quality: todo!(),
        })
    }

    pub(crate) fn new_from_name(name: &str) -> Option<Interval> {
        todo!()
    }

    pub(crate) fn interval_to_pythagorean_ratio(&self) -> Option<F> {
        let start_pitch = Pitch::new("C1".to_string());
        let end_pitch_wanted = start_pitch.transpose(&self.clone());

        let mut cache = PYTHAGOREAN_CACHE.lock().unwrap();

        let mut end_pitch_ratio: Option<(Pitch, F)> = None;
        if cache.contains_key(&end_pitch_wanted.name) {
            end_pitch_ratio = Some(cache.get(&end_pitch_wanted.name).unwrap().clone());
        } else {
            let mut end_pitch_up = start_pitch.clone();
            let mut end_pitch_down = start_pitch.clone();
            for counter in 0..37 {
                if end_pitch_up.name == end_pitch_wanted.name {
                    end_pitch_ratio = Some((end_pitch_up, F::new(3u64, 2u64).pow(counter)));
                    break;
                } else if end_pitch_down.name == end_pitch_wanted.name {
                    end_pitch_ratio = Some((end_pitch_down, F::new(2u64, 3u64).pow(counter)));
                    break;
                } else {
                    end_pitch_up = end_pitch_up.transpose(&Interval::new_from_name("P5").unwrap());
                    end_pitch_down =
                        end_pitch_down.transpose(&Interval::new_from_name("-P5").unwrap());
                }
            }
            match end_pitch_ratio.clone() {
                Some((end_pitch, ratio)) => {
                    cache.insert(end_pitch_wanted.name.clone(), (end_pitch, ratio));
                }
                None => {
                    return None;
                }
            }
        }
        match end_pitch_ratio {
            Some((end_pitch, ratio)) => {
                let octaves = (end_pitch_wanted.ps() - end_pitch.ps()) as IntegerType / 12;
                Some(ratio * F::new(2u64, 1u64).pow(octaves))
            }
            _ => None,
        }
    }
}

impl GenericInterval {
    pub(crate) fn new(simple_directed: IntegerType) -> GenericInterval {
        todo!("GenericInterval::new")
    }
}

fn intervals_to_diatonic(g_int: &GenericInterval, c_int: &ChromaticInterval) -> DiatonicInterval {
    let specifier = get_specifier_from_generic_chromatic(g_int, c_int);
    DiatonicInterval {
        specifier,
        generic: g_int.clone(),
    }
}

fn get_specifier_from_generic_chromatic(
    g_int: &GenericInterval,
    c_int: &ChromaticInterval,
) -> String {
    todo!()
}

fn notes_to_chromatic(p1: &Pitch, p2: &Pitch) -> ChromaticInterval {
    ChromaticInterval {
        semitones: (p2.ps() - p1.ps()) as IntegerType,
        simple_directed: p2.diatonic_note_num() - p1.diatonic_note_num(),
    }
}

fn notes_to_generic(p1: &Pitch, p2: &Pitch) -> GenericInterval {
    let staff_dist = p2.diatonic_note_num() - p1.diatonic_note_num();
    let gen_dist = convert_staff_distance_to_interval(staff_dist);
    GenericInterval::new(gen_dist)
}

fn convert_staff_distance_to_interval(staff_dist: IntegerType) -> IntegerType {
    match staff_dist {
        0 => 1,
        dist if dist > 0 => dist + 1,
        dist => dist - 1,
    }
}
