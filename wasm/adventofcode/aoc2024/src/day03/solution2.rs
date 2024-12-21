use super::solution1::{parse_instruction, Instruction};

pub fn solve(input: &str) -> String {
    let mut sum = 0;
    let mut mul_enabled = true;

    let mut index = 0;
    while index < input.len() {
        if let Some((instr, next_idx)) = parse_instruction(input, index) {
            match instr {
                Instruction::Mul(x, y) => {
                    if mul_enabled {
                        sum += x * y;
                    }
                }
                Instruction::Do => {
                    mul_enabled = true;
                }
                Instruction::Dont => {
                    mul_enabled = false;
                }
            }
            index = next_idx;
        } else {
            index += 1;
        }
    }

    sum.to_string()
}
