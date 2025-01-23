use std::{ops::Index, sync::Weak, vec};

use crate::{
    defaults::IntegerType,
    exception::{Exception, ExceptionResult},
    interval::Interval,
    note::note::Note,
    pitch::microtone,
    prebase::ProtoM21Object,
    stepname::StepName,
};
use derivative::Derivative;
use itertools::Itertools;

use super::{accidental::Accidental, microtone::Microtone};

const PITCH_SPACE_SIG_DIGITS: IntegerType = 6;
const PITCH_CLASS_STRING: [char; 8] = ['a', 'A', 't', 'T', 'b', 'B', 'e', 'E'];

const STEP_REF: [(StepName, IntegerType); 7] = [
    (StepName::C, 0),
    (StepName::D, 2),
    (StepName::E, 4),
    (StepName::F, 5),
    (StepName::G, 7),
    (StepName::A, 9),
    (StepName::B, 11),
];
const NATURAL_PCS: [IntegerType; 7] = [0, 2, 4, 5, 7, 9, 11];

const STEPREF_REVERSED: [(IntegerType, StepName); 7] = [
    (0, StepName::C),
    (2, StepName::D),
    (4, StepName::E),
    (5, StepName::F),
    (7, StepName::G),
    (9, StepName::A),
    (11, StepName::B),
];

const STEP_TO_DNN_OFFSET: [(StepName, IntegerType); 7] = [
    (StepName::C, 0),
    (StepName::D, 1),
    (StepName::E, 2),
    (StepName::F, 3),
    (StepName::G, 4),
    (StepName::A, 5),
    (StepName::B, 6),
];

#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq)]
pub(crate) struct Pitch {
    proto: ProtoM21Object,
    pub(crate) name: String,
    pub(crate) overriden_freq440: Option<f64>,
    pub(crate) alter: f64,
    pub(crate) accidental: Option<Accidental>,
    pub(crate) octave: Option<IntegerType>,
    #[derivative(PartialEq = "ignore")]
    pub(crate) fundamental: Option<Weak<Pitch>>,
    #[derivative(PartialEq = "ignore")]
    pub(crate) client: Option<Weak<Note>>,
    spelling_is_inferred: bool,
    step: StepName,
    microtone: Option<Microtone>,
    // pub(crate) frequency: f64,
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        note.pitch.clone()
    }
}

#[derive(Clone)]
enum TranspositionIntervalDirection {
    Up,
    Down,
}

impl TranspositionIntervalDirection {
    fn to_string(&self) -> String {
        match self {
            TranspositionIntervalDirection::Up => "d2".to_string(),
            TranspositionIntervalDirection::Down => "-d2".to_string(),
        }
    }
}

type PitchReturn = Option<Pitch>;

impl Pitch {
    pub(crate) fn new(name: String) -> Pitch {
        println!("pitch.new()");

        let step = crate::defaults::PITCH_STEP;
        let overriden_freq440: Option<f64> = None;

        let mut microtone = None;
        let mut accidental = None;
        let mut octave = None;

        let spelling_is_inferred = false;
        let fundamental = None;

        let client = None;
        let alter = todo!();

        Pitch {
            name: name.to_string(),
            alter,
            accidental,
            fundamental,
            octave,
            step: step,
            microtone,
            spelling_is_inferred,
            proto: ProtoM21Object::new(),
            overriden_freq440,
            client,
        }
    }

    pub(crate) fn implicit_octave(&self) -> IntegerType {
        match self.octave {
            Some(octave) => octave,
            None => crate::defaults::PITCH_OCTAVE,
        }
    }

    pub(crate) fn pitch_class(&self) -> IntegerType {
        self.ps() as IntegerType % 12 // maybe need to call round() on ps
    }

    fn pitch_class_setter(&mut self, new_val: IntegerType) {
        /*
            # permit the submission of strings, like "A" and "B"
        valueOut: int|float = _convertPitchClassToNumber(value)
        # get step and accidental w/o octave
        self.step, self._accidental = _convertPsToStep(valueOut)[0:2]

        # do not know what accidental is
        self.spellingIsInferred = True
        # setting step informs client
         */
        let value_out: f64 = convert_pitch_class_to_number(new_val);
        let (step, accidental, _, _) = convert_ps_to_step(value_out).unwrap();
        self.step = step;
        self.accidental = Some(accidental);
        self.spelling_is_inferred = true;
    }

    pub(crate) fn diatonic_note_num(&self) -> IntegerType {
        self.step.step_to_dnn_offset() + 1 + (7 * self.implicit_octave())
    }

    fn diatonic_note_num_setter(&mut self, new_num: IntegerType) {
        let octave = (new_num - 1) / 7;
        let note_name_num = new_num - 1 - (7 * octave);
        let note_name = StepName::step_to_dnn_offset_reverse(note_name_num);
        self.octave = Some(octave);
        self.step = note_name;
    }

    pub(crate) fn ps(&self) -> f64 {
        /*
                step = self._step
        ps = float(((self.implicitOctave + 1) * 12) + STEPREF[step])
        if self.accidental is not None:
            ps = ps + self.accidental.alter
        if self._microtone is not None:
            ps = ps + self.microtone.alter
        return ps
         */
        let step = self.step;
        let ps: f64 = ((self.implicit_octave() + 1) * 12 + StepName::step_ref(&step)) as f64;
        match &self.accidental {
            Some(accidental) => ps + accidental.alter,
            None => ps + self.microtone().alter,
        }
    }

    fn ps_setter(&mut self, new_val: f64) {}
    pub(crate) fn transpose(&self, arg: &Interval) -> Pitch {
        todo!()
    }

    fn transpose_note(&self, note: &Note) -> Note {
        let new_pitch = self.transpose_pitch(&note.pitch);
        let mut new_note = note.clone();
        new_note.pitch = new_pitch;
        new_note
    }

    fn transpose_pitch(&self, arg: &Pitch) -> Pitch {
        todo!()
    }

    fn get_all_common_enharmonics(&self, alter_limit: IntegerType) -> Vec<Pitch> {
        let mut post: Vec<Pitch> = vec![];
        let c = self.simplify_enharmonic(false);
        if c.name != self.name {
            post.push(c);
        }
        let c = self.clone();

        let mut get_enharmonics = |c: Pitch, direction: TranspositionIntervalDirection| {
            let mut c = c;
            while let Some(pitch) = c.get_enharmonic_helper(direction.clone()) {
                if let Some(ref accidental) = pitch.accidental {
                    if accidental.alter.abs() > (alter_limit as f64) {
                        break;
                    }
                }
                if !post.contains(&&pitch) {
                    let cloned_pitch = pitch.clone();
                    post.push(cloned_pitch);
                } else {
                    break;
                }
                c = pitch.clone();
            }
        };

        get_enharmonics(c.clone(), TranspositionIntervalDirection::Up);
        get_enharmonics(c, TranspositionIntervalDirection::Down);

        post
    }

    fn simplify_enharmonic(&self, most_common: bool) -> Pitch {
        let mut c = self.clone();

        if let Some(ref accidental) = c.accidental {
            if accidental.alter.abs() < 2.0 && !["E#", "B#", "C-", "F-"].contains(&c.name.as_str())
            {
                // pass
            } else {
                let save_octave = self.octave;
                c.ps_setter(self.ps());
                if save_octave.is_none() {
                    c.octave = None;
                }
            }
        }

        if most_common {
            match c.name.as_str() {
                "D#" => {
                    c.name = "E".to_string();
                    c.accidental = Some(Accidental::new_from_string("flat"));
                }
                "A#" => {
                    c.name = "B".to_string();
                    c.accidental = Some(Accidental::new_from_string("flat"));
                }
                "G-" => {
                    c.name = "F".to_string();
                    c.accidental = Some(Accidental::new_from_string("sharp"));
                }
                "D-" => {
                    c.name = "C".to_string();
                    c.accidental = Some(Accidental::new_from_string("sharp"));
                }
                _ => {}
            }
        }
        c
    }

    fn get_enharmonic_helper(&self, direction: TranspositionIntervalDirection) -> PitchReturn {
        /*
                       intervalString: t.Literal['d2', '-d2'] = 'd2'
               if not up:
                   intervalString = '-d2'

               if intervalString not in self._transpositionIntervals:
                   self._transpositionIntervals[intervalString] = interval.Interval(intervalString)
               intervalObj = self._transpositionIntervals[intervalString]
               octaveStored = self.octave  # may be None
               p = intervalObj.transposePitch(self, maxAccidental=None)
               if not inPlace:
                   if octaveStored is None:
                       p.octave = None
                   return p
               else:
                   self.step = p.step
                   self.accidental = p.accidental
                   if p.microtone is not None:
                       self.microtone = p.microtone
                   if octaveStored is None:
                       self.octave = None
                   else:
                       self.octave = p.octave
                   return None
        */
        let interval_string = match direction {
            TranspositionIntervalDirection::Up => "d2",
            TranspositionIntervalDirection::Down => "-d2",
        };

        // TODO: cache the transposition intervals?
        // if !self.transpostion_intevals.contains(&interval_string) {}

        let octave_stored = self.octave;
        let mut p = self.transpose(&Interval::new_from_name(interval_string).unwrap());
        if octave_stored.is_none() {
            p.octave = None
        }
        Some(p)
    }

    fn microtone(&self) -> Microtone {
        todo!()
    }

    fn microtone_setter(&mut self) {
        todo!()
    }
}

pub(crate) fn convert_ps_to_step(
    ps: f64,
) -> ExceptionResult<(StepName, Accidental, Microtone, IntegerType)> {
    let (pc, alter, micro, oct_shift) = if ps.fract() == 0.0 {
        let pc = (ps as IntegerType) % 12;
        (pc, 0.0, 0.0, 0)
    } else {
        // Rounding is essential
        let ps_rounded = (ps * 10f64.powi(PITCH_SPACE_SIG_DIGITS as IntegerType)).round()
            / 10f64.powi(PITCH_SPACE_SIG_DIGITS as IntegerType);
        let pc_real = ps_rounded % 12.0;
        let pc_float = pc_real.floor();
        let micro = pc_real - pc_float;
        let pc = pc_float as IntegerType;

        // Determine alter and micro
        let (alter, micro) = if (micro.round() - micro).abs() < 1e-3 {
            // Close to a quarter tone
            (0.5, micro - 0.5)
        } else if micro > 0.25 && micro < 0.75 {
            (0.5, micro - 0.5)
        } else if micro >= 0.75 && micro < 1.0 {
            (1.0, micro - 1.0)
        } else if micro > 0.0 {
            (0.0, micro)
        } else {
            (0.0, 0.0)
        };

        (pc, alter, micro, 0) // Octave shift to be determined later
    };

    // Initialize octave shift
    let mut oct_shift = 0;

    // Determine accidental and possibly adjust pitch class and octave shift
    let mut oct_shift_adjustment = 0;
    let (accidental, pc_name, mut oct_shift_adjustment) = if (alter == 1.0) && (pc == 4 || pc == 11)
    {
        // Enharmonic equivalents for E# or B#
        let acc = Accidental::natural(); // Assuming Accidental::Natural corresponds to no alteration
        let pc_name = (pc + 1) % 12;
        if pc == 11 {
            oct_shift_adjustment = 1;
        }
        (acc, pc_name, oct_shift_adjustment)
    } else if NATURAL_PCS.contains(&pc) {
        // Natural pitch class
        let acc = if alter != 0.0 {
            // Handle half-sharp or other alterations
            // This example assumes only natural is allowed
            // You may need to extend this based on `Accidental` definitions
            match alter {
                0.5 => Accidental::half_sharp(),
                _ => Accidental::natural(),
            }
        } else {
            Accidental::natural()
        };
        (acc, pc, 0)
    } else if [0, 5, 7].contains(&(pc - 1)) && alter >= 1.0 {
        // Possible double sharps
        let acc = Accidental::double_sharp(); // Adjust based on `alter`
        let pc_name = pc + 1;
        (acc, pc_name, 0)
    } else if [0, 5, 7].contains(&(pc - 1)) {
        // Sharps
        let acc = Accidental::sharp();
        let pc_name = pc - 1;
        (acc, pc_name, 0)
    } else if [11, 4].contains(&(pc + 1)) && alter <= -1.0 {
        // Double flats
        let acc = Accidental::double_flat(); // Adjust based on `alter`
        let pc_name = pc - 1;
        (acc, pc_name, 0)
    } else if [11, 4].contains(&(pc + 1)) {
        // Flats
        let acc = Accidental::flat();
        let pc_name = pc + 1;
        (acc, pc_name, 0)
    } else {
        return Err(
            Exception::PitchException(format!("cannot match condition for pc: {}", pc)).into(),
        );
    };

    // Apply octave shift adjustment
    oct_shift += oct_shift_adjustment;

    // Retrieve StepName from STEPREF_REVERSED
    let name = STEPREF_REVERSED.index(pc_name as usize).1;

    // Create Microtone object
    let microtone = if micro != 0.0 {
        Microtone::new(micro * 100.0) // Convert to cents
    } else {
        Microtone::new(0.0)
    };

    Ok((name, accidental, microtone, oct_shift))
}

fn convert_pitch_class_to_number(new_val: IntegerType) -> f64 {
    todo!()
}

pub(crate) fn simplify_multiple_enharmonics<'a>(mut pitches: Vec<Pitch>) -> Vec<Pitch> {
    if pitches.len() < 5 {
        brute_force_enharmonics_search(pitches, |x| dissonance_score(x, true, true, true))
    } else {
        greedy_enharmonics_search(pitches, |x| dissonance_score(x, true, true, true))
    }
}

fn dissonance_score(
    pitches: &[&Pitch],
    small_pythagorean_ratio: bool,
    accidental_penalty: bool,
    triad_award: bool,
) -> f64 {
    let mut score_accidentals = 0.0;
    let mut score_ratio = 0.0;
    let mut score_triad = 0.0;

    if pitches.is_empty() {
        return 0.0;
    }

    if accidental_penalty {
        let accidentals = pitches.iter().map(|p| p.alter.abs()).collect::<Vec<f64>>();
        score_accidentals = accidentals
            .iter()
            .map(|a| if *a > 1.0 { *a } else { 0.0 })
            .sum::<f64>()
            / pitches.len() as f64;
    }

    let mut intervals = vec![];

    if small_pythagorean_ratio | triad_award {
        for (index, p1) in pitches.iter().enumerate() {
            for p2 in pitches.iter().skip(index + 1) {
                let mut p1 = (*p1).clone();
                let mut p2 = (*p2).clone();
                p1.octave = None;
                p2.octave = None;
                match Interval::new(p1.clone(), p2.clone()) {
                    Some(interval) => intervals.push(interval),
                    None => return std::f64::INFINITY,
                }
            }
        }

        if small_pythagorean_ratio {
            for interval in intervals.iter() {
                match interval.interval_to_pythagorean_ratio() {
                    Some(ratio) => {
                        score_ratio += (*(ratio.denom().unwrap()) as f64).ln() * 0.03792663444
                    }
                    None => return std::f64::INFINITY,
                };
            }
            score_ratio /= pitches.len() as f64;
        }

        if triad_award {
            intervals.into_iter().for_each(|interval| {
                let simple_directed = interval.generic.simple_directed;
                let interval_semitones = interval.chromatic.semitones % 12;
                if (simple_directed == 3 && (interval_semitones == 3 || interval_semitones == 4))
                    || (simple_directed == 6
                        && (interval_semitones == 8 || interval_semitones == 9))
                {
                    score_triad -= 1.0;
                }
            });
            score_triad /= pitches.len() as f64;
        }
    }

    (score_accidentals + score_ratio + score_triad)
        / (small_pythagorean_ratio as IntegerType
            + accidental_penalty as IntegerType
            + triad_award as IntegerType) as f64
}

fn greedy_enharmonics_search(
    old_pitches: Vec<Pitch>,
    score_func: fn(&[&Pitch]) -> f64,
) -> Vec<Pitch> {
    let mut new_pitches = vec![old_pitches[0].clone()];
    for old_pitch in old_pitches.iter().skip(1) {
        let mut candidates = vec![old_pitch.clone()];
        candidates.extend(old_pitch.get_all_common_enharmonics(2).into_iter().cloned());
        let new_pitch = candidates
            .iter()
            .min_by(|x, y| {
                dissonance_score(&new_pitches.iter().collect::<Vec<_>>(), true, true, true)
                    .partial_cmp(&score_func(&new_pitches.iter().collect::<Vec<_>>()))
                    .unwrap()
            })
            .unwrap();
        new_pitches.push(new_pitch.clone());
    }
    new_pitches
}

fn brute_force_enharmonics_search(
    old_pitches: Vec<Pitch>,
    score_func: fn(&[&Pitch]) -> f64,
) -> Vec<Pitch> {
    let all_possible_pitches: Vec<Vec<Pitch>> = old_pitches[1..]
        .iter()
        .map(|p| {
            let mut enharmonics = p.get_all_common_enharmonics(2);
            enharmonics.insert(0, (*p).clone());
            enharmonics
        })
        .collect();

    let all_pitch_combinations = all_possible_pitches.into_iter().multi_cartesian_product();

    let mut min_score = f64::MAX;
    let mut best_combination: Vec<Pitch> = Vec::new();

    for combination in all_pitch_combinations {
        let mut pitches: Vec<Pitch> = old_pitches[..1].iter().cloned().collect(); // Use range syntax for clarity
        pitches.extend(combination);
        let score = score_func(&pitches.iter().collect::<Vec<_>>());
        if score < min_score {
            min_score = score;
            best_combination = pitches;
        }
    }

    best_combination
}

fn convert_harmonic_to_cents(mut value: f64) -> IntegerType {
    if value < 0.0 {
        value = 1.0 / value.abs();
    }
    (1200.0 * value.log2()).round() as IntegerType
}
