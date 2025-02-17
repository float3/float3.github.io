use crate::equal_temperament;
use crate::get_ratio_from_step_algorithm;
use crate::Fraction;
use crate::ELEVEN_LIMIT;
use crate::FIVE_LIMIT;
use crate::FORTYTHREE_TONE;
use crate::INDIAN_SCALE;
use crate::INDIAN_SCALE_22;
use crate::INDIA_SCALE_ALT;
use crate::JUST_INTONATION;
use crate::JUST_INTONATION_24;
use crate::PYTHAGOREAN_TUNING;
use crate::SHRUTIS;
use crate::SWARAS;
use crate::TWELVE_TONE_NAMES;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TuningSystem {
    EqualTemperament { octave_size: usize },
    StepMethod { octave_size: usize, step_size: usize },

    // Javanese,
    // Thai,
    WholeTone,
    QuarterTone,

    JustIntonation,
    JustIntonation24,
    PythagoreanTuning,

    FiveLimit,
    ElevenLimit,

    FortyThreeTone,

    // ethnic scales
    Indian,
    IndianAlt,
    Indian22,
}

impl TuningSystem {
    pub fn get_fraction(&self, index: usize) -> Fraction {
        match &self {
            TuningSystem::StepMethod { octave_size, step_size } => get_ratio_from_step_algorithm(index, *octave_size, *step_size),
            TuningSystem::EqualTemperament { octave_size } => equal_temperament(index, *octave_size),
            // TuningSystem::Javanese => todo!(), // equal_temperament(index, 5), implement slendro and or pelog maybe
            // TuningSystem::Thai => equal_temperament(index, 9),
            TuningSystem::WholeTone => equal_temperament(index, 6),
            TuningSystem::QuarterTone => equal_temperament(index, 24),
            TuningSystem::JustIntonation
            | TuningSystem::JustIntonation24
            | TuningSystem::PythagoreanTuning
            | TuningSystem::FiveLimit
            | TuningSystem::ElevenLimit
            | TuningSystem::FortyThreeTone
            | TuningSystem::Indian
            | TuningSystem::IndianAlt
            | TuningSystem::Indian22 => self.get_fraction_from_table(index),
        }
    }

    pub fn size(&self) -> usize {
        match &self {
            TuningSystem::JustIntonation
            | TuningSystem::JustIntonation24
            | TuningSystem::PythagoreanTuning
            | TuningSystem::FiveLimit
            | TuningSystem::ElevenLimit
            | TuningSystem::FortyThreeTone
            | TuningSystem::Indian
            | TuningSystem::IndianAlt
            | TuningSystem::Indian22 => self.get_lut_from_tuningsystem().len(),
            TuningSystem::StepMethod { .. } => 12,
            TuningSystem::EqualTemperament { octave_size } => *octave_size,
            // TuningSystem::Thai => 9,
            // TuningSystem::Javanese => 5,
            TuningSystem::WholeTone => 6,
            TuningSystem::QuarterTone => 24,
        }
    }

    pub(crate) fn get_fraction_from_table(&self, index: usize) -> Fraction {
        let lut = self.get_lut_from_tuningsystem();
        let len = lut.len();
        let octave = index / len;
        let index_mod: usize = index % len;
        let mut fraction = lut[index_mod];
        // fraction.numerator += (2ToneIndex.pow(octave as ToneIndex) - 1) * fraction.denominator;
        fraction.numerator *= (2u32).pow(octave as u32);
        fraction
    }

    fn get_lut_from_tuningsystem(&self) -> &[Fraction] {
        let lut: &[Fraction] = match self {
            TuningSystem::JustIntonation => &JUST_INTONATION,
            TuningSystem::JustIntonation24 => &JUST_INTONATION_24,
            TuningSystem::PythagoreanTuning => &PYTHAGOREAN_TUNING,
            TuningSystem::FiveLimit => &FIVE_LIMIT,
            TuningSystem::ElevenLimit => &ELEVEN_LIMIT,
            TuningSystem::FortyThreeTone => &FORTYTHREE_TONE,
            TuningSystem::Indian => &INDIAN_SCALE,
            TuningSystem::IndianAlt => &INDIA_SCALE_ALT,
            TuningSystem::Indian22 => &INDIAN_SCALE_22,

            TuningSystem::StepMethod { ..}
            | TuningSystem::EqualTemperament { .. }
            // | TuningSystem::Thai
            // | TuningSystem::Javanese
            | TuningSystem::QuarterTone
            | TuningSystem::WholeTone => {
                unreachable!("these tuning methods don't have LUTs. Use get_fraction instead.")
            }
        };
        lut
    }

    pub(crate) fn get_tone_name(&self, tone_index: usize) -> String {
        // if indian or indianalt we want to use 7
        let name = match self {
            TuningSystem::EqualTemperament { octave_size } if *octave_size == 24 => {
                TWELVE_TONE_NAMES[tone_index % octave_size / 2]
            }
            TuningSystem::EqualTemperament { octave_size } if *octave_size == 12 => TWELVE_TONE_NAMES[tone_index % octave_size],
            TuningSystem::EqualTemperament { octave_size } if *octave_size == 6 => {
                TWELVE_TONE_NAMES[tone_index % octave_size * 2]
            }
            TuningSystem::WholeTone => TWELVE_TONE_NAMES[(tone_index % 6) * 2],
            TuningSystem::EqualTemperament { octave_size } if *octave_size == 4 => {
                TWELVE_TONE_NAMES[tone_index % octave_size * 3]
            }
            TuningSystem::EqualTemperament { octave_size } if *octave_size == 3 => {
                TWELVE_TONE_NAMES[tone_index % octave_size * 4]
            }
            TuningSystem::EqualTemperament { octave_size } if *octave_size == 2 => {
                TWELVE_TONE_NAMES[tone_index % octave_size * 6]
            }
            TuningSystem::EqualTemperament { octave_size } if *octave_size == 1 => {
                TWELVE_TONE_NAMES[tone_index % octave_size * 12]
            }
            TuningSystem::EqualTemperament { octave_size } => TWELVE_TONE_NAMES[tone_index % octave_size * (octave_size / 12)],

            TuningSystem::JustIntonation
            | TuningSystem::PythagoreanTuning
            | TuningSystem::FiveLimit
            | TuningSystem::StepMethod { .. } => TWELVE_TONE_NAMES[tone_index % self.size()],

            TuningSystem::Indian | TuningSystem::IndianAlt => SWARAS[tone_index % SWARAS.len()],
            TuningSystem::Indian22 => SHRUTIS[tone_index % SHRUTIS.len()],

            // TuningSystem::Javanese => SLENDRO[tone_index % SLENDRO.len()],
            // TuningSystem::Thai => "todo",
            TuningSystem::QuarterTone => "todo",
            TuningSystem::JustIntonation24 => "todo",
            TuningSystem::ElevenLimit => "todo",
            TuningSystem::FortyThreeTone => "todo",
        };

        let octave = tone_index / self.size();
        let adjusted_octave: i32 = octave as i32 - 1;
        if adjusted_octave < 0 {
            format!("{}N{}", name, -adjusted_octave)
        } else {
            format!("{}{}", name, adjusted_octave)
        }
    }
}
