use crate::Fraction;
use crate::TuningSystem;

pub(crate) fn equal_temperament(tone: usize, octave_size: usize) -> Fraction {
    Fraction::new_with_base(tone as u32, octave_size as u32, 2)
}

// pub(crate) fn equal_temperament_12(tone: usize) -> Fraction {
//     equal_temperament(tone, 12)
// }

pub(crate) fn get_ratio_from_step_algorithm(n: usize, octave_size: usize, step_size: usize) -> Fraction {
    let ratio = TuningSystem::JustIntonation.get_fraction_from_table(step_size);
    let n2 = n % octave_size;
    let mut current_ratio = Fraction::new(1, 1);
    let mut current_idx = 0;
    let two = Fraction::new(2, 1);
    while current_idx != n2 {
        current_ratio *= ratio;
        current_idx += step_size;
        current_idx %= octave_size;
        if current_ratio > two {
            current_ratio /= two;
        }
    }
    let octaves = (n / octave_size) as f64;
    current_ratio *= Fraction::new(2u32.pow(octaves as u32), 1);
    current_ratio
}
