//! Every scale the playground can put under its keys, realised into one shape.
//!
//! Five kinds of thing feed it: music21-rs's built-in tuning systems, its
//! regular temperaments realised as moment-of-symmetry scales, equal divisions
//! of any interval, the Scala archive it bundles, and a `.scl` file the reader
//! pastes. The page draws all of them the same way: a root frequency, a list of
//! degrees rising from it, and the interval the list repeats at.
//!
//! A *step* counts degrees from five periods below the root, so that in a
//! twelve-tone octave scale a step is a MIDI note number: step 60 is the root
//! C4, step 69 is A4. In any other scale step `5·n` is the root and each step
//! up is the next degree, wrapping at the period.

use music21_rs::tuningsystem::adaptive::{AdaptiveTuningSystem, RECURSIVE_JI};
use music21_rs::tuningsystem::{
    ALL_TUNING_SYSTEMS, C4, CN1, COMMON_EQUAL_TEMPERAMENTS, EqualDivision, HISTORICAL_TEMPERAMENTS,
    TWELVE_TONE_NAMES_SHARP, TuningSystem, WIKI_TEMPERAMENTS,
};
use music21_rs::{ScalaArchive, ScalaScale};
use serde::Serialize;
use std::sync::OnceLock;

/// The period the root sits in, counted from step zero.
pub const ROOT_PERIOD: i64 = 5;

/// The share id of the one adaptive tuning.
pub const ADAPTIVE_ID: &str = "adaptive:recursive";

/// One degree of a realised scale.
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Degree {
    /// `3/2` for an exact ratio, `7\12` for a step of an equal division,
    /// `701.955¢` where the scale is given in cents.
    pub ratio_label: String,
    pub ratio: f64,
    /// Above the root.
    pub cents: f64,
    /// Against the equal division of the same period into the same number of
    /// steps.
    pub from_equal: f64,
    pub frequency: f64,
}

/// What the page says about the regular temperament a scale was made from.
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct TemperamentFacts {
    pub name: String,
    pub page: String,
    pub subgroup: String,
    pub rank: usize,
    pub periods_per_equave: u32,
    pub generators: Vec<Generator>,
    pub optimization: String,
    pub commas: Vec<String>,
    pub published_moments: Vec<String>,
    /// Note counts to the equave that make a moment of symmetry, up to seventy-two.
    pub moments: Vec<u32>,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Generator {
    pub ratio: String,
    pub cents: f64,
}

/// A scale ready to be played.
#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct Scale {
    pub id: String,
    pub name: String,
    pub family: String,
    pub description: String,
    /// Degrees to the period.
    pub count: usize,
    pub period_ratio: f64,
    pub period_cents: f64,
    pub root_hz: f64,
    /// Whether a key's pitch depends on the lowest key held.
    pub adaptive: bool,
    /// Twelve degrees to an octave, so keys have note names and a staff.
    pub twelve_tone: bool,
    /// `count + 1` entries; the last is the period.
    pub degrees: Vec<Degree>,
    pub temperament: Option<TemperamentFacts>,
}

#[derive(Serialize, Clone, Debug)]
pub struct SystemEntry {
    pub id: String,
    pub name: String,
    pub family: String,
    pub description: String,
    pub count: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct EqualPreset {
    pub id: String,
    pub label: String,
    pub divisions: u32,
    pub numerator: i32,
    pub denominator: i32,
    pub note: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ScalaEntry {
    pub id: String,
    pub file: String,
    pub description: String,
    pub count: usize,
}

/// Everything the picker lists before the reader types anything.
#[derive(Serialize, Clone, Debug)]
pub struct Library {
    pub systems: Vec<SystemEntry>,
    pub temperaments: Vec<TemperamentFacts>,
    pub equal: Vec<EqualPreset>,
    pub scala_count: usize,
}

/// One key of the keyboard the page draws for the current scale.
#[derive(Serialize, Clone, Debug)]
pub struct Key {
    pub step: i64,
    pub degree: usize,
    pub label: String,
    pub cents: f64,
    pub frequency: f64,
    pub black: bool,
}

fn archive() -> &'static ScalaArchive {
    static ARCHIVE: OnceLock<ScalaArchive> = OnceLock::new();
    ARCHIVE.get_or_init(ScalaArchive::bundled)
}

fn cents_of(ratio: f64) -> f64 {
    1200.0 * ratio.log2()
}

fn family_of(tuning: TuningSystem) -> &'static str {
    use TuningSystem::*;
    match tuning {
        EqualTemperament { .. } | WholeTone | QuarterTone => "Equal divisions",
        CarlosHarmonic | CarlosHarmonic24 => "Harmonic series",
        PythagoreanTuning | FiveLimit | ElevenLimit | FortyThreeTone | PtolemyIntenseDiatonic => {
            "Just intonation"
        }
        Javanese | Thai | IndianAlt | Indian22 => "World and regional",
        _ if HISTORICAL_TEMPERAMENTS.contains(&tuning) => "Historical temperaments",
        _ => "Other",
    }
}

fn check_root(root_hz: f64) -> Result<(), String> {
    if root_hz.is_finite() && root_hz > 0.0 {
        Ok(())
    } else {
        Err("the root frequency has to be a positive number of hertz".to_string())
    }
}

/// Degrees from a list of cents above the root, the last being the period.
fn degrees_from_cents(cents: &[f64], period_cents: f64, root_hz: f64) -> Vec<Degree> {
    let count = cents.len().saturating_sub(1).max(1) as f64;
    let equal_step = period_cents / count;
    cents
        .iter()
        .enumerate()
        .map(|(index, &cents)| {
            let ratio = 2f64.powf(cents / 1200.0);
            Degree {
                ratio_label: format!("{cents:.3}¢"),
                ratio,
                cents,
                from_equal: cents - index as f64 * equal_step,
                frequency: root_hz * ratio,
            }
        })
        .collect()
}

/// The names the picker shows for the built-in systems.
pub fn library() -> Library {
    let mut systems: Vec<SystemEntry> = ALL_TUNING_SYSTEMS
        .into_iter()
        .map(|tuning| SystemEntry {
            id: tuning.id().to_string(),
            name: tuning.display_name().to_string(),
            family: family_of(tuning).to_string(),
            description: tuning.description().to_string(),
            count: tuning.octave_size() as usize,
        })
        .collect();
    systems.push(SystemEntry {
        id: ADAPTIVE_ID.to_string(),
        name: "Recursive just intonation".to_string(),
        family: "Adaptive".to_string(),
        description: adaptive_description().to_string(),
        count: 12,
    });

    let temperaments = WIKI_TEMPERAMENTS.iter().map(temperament_facts).collect();

    let mut equal: Vec<EqualPreset> = COMMON_EQUAL_TEMPERAMENTS
        .into_iter()
        .filter_map(|tuning| match tuning {
            TuningSystem::EqualTemperament { octave_size } if octave_size != 12 => {
                Some((octave_size, 2, 1, edo_note(octave_size)))
            }
            _ => None,
        })
        .chain([
            (5, 2, 1, "five equal steps, the shape of Javanese slendro"),
            (7, 2, 1, "seven equal steps, the shape of the Thai scale"),
            (15, 2, 1, "Blackwood's decatonic playground"),
            (17, 2, 1, "a neutral-third system with a sharp fifth"),
            (13, 3, 1, "Bohlen-Pierce, thirteen steps of a twelfth"),
            (9, 3, 2, "Wendy Carlos's alpha, 78 cents a step"),
            (11, 3, 2, "Wendy Carlos's beta, 63.8 cents a step"),
            (20, 3, 2, "Wendy Carlos's gamma, 35.1 cents a step"),
            (8, 5, 2, "eight steps of a major tenth"),
        ])
        .map(|(divisions, numerator, denominator, note)| EqualPreset {
            id: format!("equal:{divisions}:{numerator}:{denominator}"),
            label: EqualDivision::of_ratio(divisions, numerator, denominator)
                .map(|division| division.to_string())
                .unwrap_or_else(|_| format!("{divisions}ed{numerator}/{denominator}")),
            divisions,
            numerator,
            denominator,
            note: note.to_string(),
        })
        .collect();
    equal.sort_by_key(|preset| {
        (
            preset.numerator * 100 + preset.denominator,
            preset.divisions,
        )
    });

    Library {
        systems,
        temperaments,
        equal,
        scala_count: archive().len(),
    }
}

fn edo_note(divisions: u32) -> &'static str {
    match divisions {
        19 => "a meantone system where the diesis is one step",
        22 => "the Indian shruti count, and a superpyth and porcupine tuning",
        24 => "quarter tones",
        31 => "Huygens and Fokker's near-quarter-comma meantone",
        41 => "a near-Pythagorean system with good sevenths",
        53 => "the Holderian comma, close to just intonation",
        72 => "twelfth tones, a superset of 12, 24 and 36",
        _ => "an equal division of the octave",
    }
}

fn adaptive_description() -> &'static str {
    "Every key is tuned in just intonation from the lowest key held, so the same key sounds at a \
     different pitch in a different chord. The table shows the tuning over C; hold a bass note and \
     the rest retune to it."
}

fn temperament_facts(named: &music21_rs::tuningsystem::NamedTemperament) -> TemperamentFacts {
    let temperament = named.temperament().ok();
    let moments = temperament
        .as_ref()
        .and_then(|temperament| temperament.moments(72).ok())
        .unwrap_or_default();
    TemperamentFacts {
        name: named.name.to_string(),
        page: named.page.to_string(),
        subgroup: named
            .subgroup
            .iter()
            .map(|prime| prime.to_string())
            .collect::<Vec<_>>()
            .join("."),
        rank: named.rank(),
        periods_per_equave: named.periods_per_equave,
        generators: named
            .generator_ratios
            .iter()
            .zip(named.generator_cents.iter())
            .map(|(ratio, cents)| Generator {
                ratio: (*ratio).to_string(),
                cents: *cents,
            })
            .collect(),
        optimization: named.optimization.to_string(),
        commas: named
            .commas
            .iter()
            .map(|comma| (*comma).to_string())
            .collect(),
        published_moments: named
            .moments
            .iter()
            .map(|moment| (*moment).to_string())
            .collect(),
        moments,
    }
}

/// Scala scales whose file name matches, or every scale for an empty query.
pub fn scala_search(query: &str, limit: usize) -> Vec<ScalaEntry> {
    let archive = archive();
    let mut names: Vec<&str> = if query.trim().is_empty() {
        archive.names().collect()
    } else {
        archive.search(query)
    };
    names.sort_unstable();
    names
        .into_iter()
        .take(if limit == 0 { usize::MAX } else { limit })
        .filter_map(|file| {
            let scale = archive.get(file)?;
            Some(ScalaEntry {
                id: format!("scala:{file}"),
                file: file.to_string(),
                description: scale.description().to_string(),
                count: scale.len(),
            })
        })
        .collect()
}

/// The scale a share id names.
pub fn realize(id: &str, root_hz: f64) -> Result<Scale, String> {
    check_root(root_hz)?;
    if id == ADAPTIVE_ID {
        return Ok(adaptive(root_hz));
    }
    if let Some(file) = id.strip_prefix("scala:") {
        let scale = archive()
            .get(file)
            .ok_or_else(|| format!("no bundled Scala scale is named {file}"))?;
        if scale.is_empty() {
            return Err(format!("{file} has no degrees to play"));
        }
        return Ok(from_scala(
            id,
            file.trim_end_matches(".scl"),
            "Scala archive",
            scale,
            root_hz,
        ));
    }
    if let Some(rest) = id.strip_prefix("temperament:") {
        let (name, notes) = rest
            .split_once(':')
            .ok_or_else(|| format!("{id} names no note count"))?;
        let notes: u32 = notes
            .parse()
            .map_err(|_| format!("{notes} is not a number of notes"))?;
        return temperament_scale(name, notes, root_hz);
    }
    if let Some(rest) = id.strip_prefix("equal:") {
        let parts: Vec<&str> = rest.split(':').collect();
        let [divisions, numerator, denominator] = parts[..] else {
            return Err(format!("{id} is not divisions:numerator:denominator"));
        };
        let parse = |text: &str| {
            text.parse::<i64>()
                .map_err(|_| format!("{text} is not a whole number"))
        };
        return equal_division(
            parse(divisions)? as u32,
            parse(numerator)? as i32,
            parse(denominator)? as i32,
            root_hz,
        );
    }
    let tuning = ALL_TUNING_SYSTEMS
        .into_iter()
        .find(|tuning| tuning.id() == id)
        .ok_or_else(|| format!("no tuning is named {id}"))?;
    Ok(built_in(tuning, root_hz))
}

/// A scale from the text of a `.scl` file.
pub fn realize_scl(name: &str, contents: &str, root_hz: f64) -> Result<Scale, String> {
    check_root(root_hz)?;
    let scale = ScalaScale::parse(contents).map_err(|error| format!("{name}: {error}"))?;
    if scale.is_empty() {
        return Err(format!("{name} has no degrees to play"));
    }
    let stem = name.trim_end_matches(".scl");
    Ok(from_scala(
        &format!("pasted:{stem}"),
        stem,
        "Your scale",
        &scale,
        root_hz,
    ))
}

fn built_in(tuning: TuningSystem, root_hz: f64) -> Scale {
    let count = tuning.octave_size() as usize;
    let equal_step = 1200.0 / count as f64;
    let degrees = (0..=count)
        .map(|degree| {
            let fraction = tuning.fraction(degree);
            let ratio = fraction.ratio();
            let cents = cents_of(ratio);
            Degree {
                ratio_label: if fraction.base() == 0 {
                    fraction.label()
                } else {
                    format!("{degree}\\{count}")
                },
                ratio,
                cents,
                from_equal: cents - degree as f64 * equal_step,
                frequency: root_hz * ratio,
            }
        })
        .collect();
    Scale {
        id: tuning.id().to_string(),
        name: tuning.display_name().to_string(),
        family: family_of(tuning).to_string(),
        description: tuning.description().to_string(),
        count,
        period_ratio: 2.0,
        period_cents: 1200.0,
        root_hz,
        adaptive: false,
        twelve_tone: count == 12,
        degrees,
        temperament: None,
    }
}

/// The table the adaptive scale places its keys on is music21-rs's own, so the
/// two cannot drift apart. It was once written here as `CarlosHarmonic`, which
/// kept the harmonic series under the keys after music21-rs moved to five-limit.
fn adaptive(root_hz: f64) -> Scale {
    let AdaptiveTuningSystem::Recursive {
        root_tuning_system, ..
    } = RECURSIVE_JI;
    let mut scale = built_in(root_tuning_system, root_hz);
    scale.id = ADAPTIVE_ID.to_string();
    scale.name = "Recursive just intonation".to_string();
    scale.family = "Adaptive".to_string();
    scale.description = adaptive_description().to_string();
    scale.adaptive = true;
    scale
}

fn from_scala(id: &str, name: &str, family: &str, scale: &ScalaScale, root_hz: f64) -> Scale {
    let count = scale.len().max(1);
    let period_cents = scale.period().cents();
    let equal_step = period_cents / count as f64;
    // The archive keeps the period apart from the degrees; the page wants it
    // as the last row, the way every other scale here ends.
    let degrees = scale
        .degrees()
        .iter()
        .chain(std::iter::once(&scale.period()))
        .enumerate()
        .map(|(degree, entry)| Degree {
            ratio_label: if entry.as_fraction().is_some() {
                entry.to_string()
            } else {
                format!("{:.3}¢", entry.cents())
            },
            ratio: entry.ratio(),
            cents: entry.cents(),
            from_equal: entry.cents() - degree as f64 * equal_step,
            frequency: root_hz * entry.ratio(),
        })
        .collect();
    Scale {
        id: id.to_string(),
        name: name.to_string(),
        family: family.to_string(),
        description: scale.description().to_string(),
        count: scale.len(),
        period_ratio: scale.period().ratio(),
        period_cents,
        root_hz,
        adaptive: false,
        twelve_tone: scale.len() == 12 && (scale.period().ratio() - 2.0).abs() < 1e-9,
        degrees,
        temperament: None,
    }
}

fn temperament_scale(name: &str, notes: u32, root_hz: f64) -> Result<Scale, String> {
    let named = WIKI_TEMPERAMENTS
        .iter()
        .find(|named| named.name == name)
        .ok_or_else(|| format!("no temperament is named {name}"))?;
    let temperament = named.temperament().map_err(|error| error.to_string())?;
    let mos = temperament.mos(notes).map_err(|error| error.to_string())?;
    let periods = temperament.periods_per_equave().unsigned_abs().max(1);
    let period_cents = temperament.period_cents();
    let mut cents = Vec::new();
    for period in 0..periods {
        for degree in mos.degrees() {
            cents.push(degree + f64::from(period) * period_cents);
        }
    }
    let equave_cents = temperament.equave_cents();
    cents.push(equave_cents);
    let pattern = mos
        .word()
        .map(|word| format!(", pattern {word}"))
        .unwrap_or_default();
    let generators = named
        .generator_ratios
        .iter()
        .zip(named.generator_cents.iter())
        .map(|(ratio, cents)| format!("{ratio} at {cents:.3}¢"))
        .collect::<Vec<_>>()
        .join(", ");
    Ok(Scale {
        id: format!("temperament:{name}:{notes}"),
        name: format!("{}, {notes} notes", named.page),
        family: "Regular temperament".to_string(),
        description: format!(
            "{} temperament in the {} subgroup, {notes} notes to the equave{pattern}; generator {generators}.",
            named.page,
            temperament.subgroup_name()
        ),
        count: cents.len() - 1,
        period_ratio: 2f64.powf(equave_cents / 1200.0),
        period_cents: equave_cents,
        root_hz,
        adaptive: false,
        twelve_tone: false,
        degrees: degrees_from_cents(&cents, equave_cents, root_hz),
        temperament: Some(temperament_facts(named)),
    })
}

fn equal_division(
    divisions: u32,
    numerator: i32,
    denominator: i32,
    root_hz: f64,
) -> Result<Scale, String> {
    let division = EqualDivision::of_ratio(divisions, numerator, denominator)
        .map_err(|error| error.to_string())?;
    let cents: Vec<f64> = (0..=divisions)
        .map(|degree| division.cents_at(degree as i32))
        .collect();
    let mut degrees = degrees_from_cents(&cents, division.period_cents(), root_hz);
    for (degree, entry) in degrees.iter_mut().enumerate() {
        entry.ratio_label = format!("{degree}\\{divisions}");
    }
    let octave = numerator == 2 && denominator == 1;
    Ok(Scale {
        id: format!("equal:{divisions}:{numerator}:{denominator}"),
        name: division.to_string(),
        family: "Equal division".to_string(),
        description: format!(
            "{divisions} equal steps of {numerator}/{denominator}, {:.3} cents each.",
            division.step_cents()
        ),
        count: divisions as usize,
        period_ratio: f64::from(numerator) / f64::from(denominator),
        period_cents: division.period_cents(),
        root_hz,
        adaptive: false,
        twelve_tone: octave && divisions == 12,
        degrees,
        temperament: None,
    })
}

impl Scale {
    /// Twelve-tone equal temperament over C4 at its usual pitch.
    pub fn default_scale() -> Self {
        built_in(TuningSystem::EqualTemperament { octave_size: 12 }, C4)
    }

    /// Which period a step falls in, and which degree of it.
    pub fn split(&self, step: i64) -> (i64, usize) {
        let count = self.count.max(1) as i64;
        (step.div_euclid(count), step.rem_euclid(count) as usize)
    }

    /// The pitch of a step, in hertz.
    pub fn frequency(&self, step: i64) -> f64 {
        let (period, degree) = self.split(step);
        self.root_hz
            * self.period_ratio.powi((period - ROOT_PERIOD) as i32)
            * self.degrees.get(degree).map_or(1.0, |entry| entry.ratio)
    }

    /// The pitch of a step when `context` is the lowest step held.
    ///
    /// In a fixed scale that is [`Scale::frequency`]. In the adaptive one the
    /// context is tuned as a degree over C and the step as a just interval over
    /// the context, so a third over a third is two just thirds.
    pub fn frequency_from(&self, context: i64, step: i64) -> f64 {
        if !self.adaptive || step < context {
            return self.frequency(step);
        }
        let local = (step - context) as f64;
        let above_context = RECURSIVE_JI.frequency_at(0.0, local, None) / CN1;
        self.frequency(context) * above_context
    }

    /// The name of a step, as music21 spells it: `C#N4` for the staff and the
    /// chord namer in a twelve-tone scale, and the degree with its period
    /// otherwise.
    pub fn note_name(&self, step: i64) -> String {
        let (period, degree) = self.split(step);
        if self.twelve_tone {
            let octave = period - 1;
            return format!("{}N{octave}", TWELVE_TONE_NAMES_SHARP[degree]);
        }
        let offset = period - ROOT_PERIOD;
        if offset == 0 {
            (degree + 1).to_string()
        } else {
            format!("{}{offset:+}", degree + 1)
        }
    }

    /// The label a key carries: `C#4` in a twelve-tone scale, the degree number
    /// otherwise.
    pub fn key_label(&self, step: i64) -> String {
        if self.twelve_tone {
            let (period, degree) = self.split(step);
            return format!("{}{}", TWELVE_TONE_NAMES_SHARP[degree], period - 1);
        }
        self.note_name(step)
    }

    /// The keys the page draws: the eighty-eight of a piano for a twelve-tone
    /// scale, and for any other a run of periods from one below the root, as
    /// many as fit comfortably.
    pub fn keyboard(&self) -> Vec<Key> {
        let count = self.count.max(1) as i64;
        let (start, keys) = if self.twelve_tone {
            (21, 88)
        } else {
            let periods = ((60.0 / count as f64).round() as i64).clamp(2, 6);
            ((ROOT_PERIOD - 1) * count, count * periods)
        };
        (start..start + keys)
            .map(|step| {
                let (_, degree) = self.split(step);
                Key {
                    step,
                    degree,
                    label: self.key_label(step),
                    cents: (self.degrees[degree].from_equal * 10.0).round() / 10.0,
                    frequency: self.frequency(step),
                    black: self.twelve_tone && matches!(degree, 1 | 3 | 6 | 8 | 10),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_tone_steps_are_midi_numbers() {
        let scale = Scale::default_scale();
        assert!((scale.frequency(69) - 440.0).abs() < 1e-9);
        assert!((scale.frequency(60) - C4).abs() < 1e-9);
        assert_eq!(scale.note_name(61), "C#N4");
        assert_eq!(scale.key_label(60), "C4");
        assert_eq!(scale.keyboard().len(), 88);
    }

    #[test]
    fn every_library_entry_realizes() {
        let library = library();
        for system in &library.systems {
            let scale = realize(&system.id, C4).expect(&system.id);
            assert_eq!(scale.count, system.count, "{}", system.id);
            assert_eq!(scale.degrees.len(), scale.count + 1, "{}", system.id);
        }
        for preset in &library.equal {
            let scale = realize(&preset.id, C4).expect(&preset.id);
            assert_eq!(scale.count, preset.divisions as usize);
        }
        for temperament in &library.temperaments {
            let Some(size) = temperament
                .moments
                .iter()
                .find(|size| (5..=12).contains(*size))
                .or(temperament.moments.last())
            else {
                continue;
            };
            let id = format!("temperament:{}:{size}", temperament.name);
            let scale = realize(&id, C4).expect(&id);
            assert_eq!(scale.count, *size as usize, "{id}");
            assert!(scale.temperament.is_some());
        }
        assert!(library.scala_count > 3000);
    }

    #[test]
    fn scala_scales_realize_and_search() {
        let found = scala_search("bohlen", 0);
        assert!(!found.is_empty());
        let scale = realize(&found[0].id, C4).unwrap();
        assert_eq!(scale.count, found[0].count);
        assert_eq!(scale.degrees.len(), scale.count + 1);
        assert!((scale.degrees[scale.count].ratio - scale.period_ratio).abs() < 1e-9);
        assert!((scale.frequency(5 * scale.count as i64) - C4).abs() < 1e-9);
        assert_eq!(scala_search("", 0).len(), library().scala_count);
    }

    #[test]
    fn a_third_over_a_third_is_two_just_thirds() {
        let scale = realize(ADAPTIVE_ID, C4).unwrap();
        assert!((scale.frequency_from(60, 64) - C4 * 5.0 / 4.0).abs() < 1e-9);
        assert!((scale.frequency_from(64, 68) - C4 * 25.0 / 16.0).abs() < 1e-9);
        assert!((scale.frequency_from(60, 72) - 2.0 * C4).abs() < 1e-9);
        assert!((scale.frequency_from(64, 60) - C4).abs() < 1e-9);
    }

    /// Keys are tuned from the lowest one held, so a chord in inversion is tuned
    /// from its bass. It is the same chord only because five-limit's fourth and
    /// sixths complement its fifth and thirds.
    #[test]
    fn a_chord_in_inversion_is_the_same_chord() {
        let scale = realize(ADAPTIVE_ID, C4).unwrap();
        assert!((scale.frequency(65) - C4 * 4.0 / 3.0).abs() < 1e-9);
        assert!((scale.frequency_from(67, 72) - 2.0 * C4).abs() < 1e-9);
        assert!((scale.frequency_from(64, 72) - 2.0 * C4).abs() < 1e-9);
    }

    #[test]
    fn a_pasted_scale_is_its_own() {
        let scale = realize_scl(
            "fifth.scl",
            "! fifth.scl\n!\nA fifth and an octave\n2\n!\n3/2\n2/1\n",
            C4,
        )
        .unwrap();
        assert_eq!(scale.count, 2);
        assert_eq!(scale.degrees[1].ratio_label, "3/2");
        assert_eq!(scale.note_name(10), "1");
        assert_eq!(scale.note_name(12), "1+1");
        assert!(realize_scl("bad.scl", "nonsense", C4).is_err());
    }

    #[test]
    fn a_non_octave_scale_repeats_at_its_period() {
        let scale = realize("equal:13:3:1", C4).unwrap();
        assert!(!scale.twelve_tone);
        assert!((scale.frequency(13 * 6) - 3.0 * C4).abs() < 1e-9);
        assert_eq!(scale.keyboard()[0].step, 4 * 13);
    }
}
