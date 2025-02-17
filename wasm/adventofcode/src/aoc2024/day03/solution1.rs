// this would have been easy with regex but i decided to do parsing since it's more fun
#[derive(Debug)]
pub(super) enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

pub fn solve(input: &str) -> String {
    input
        .char_indices()
        .filter_map(|(i, _)| parse_mul_at(input, i))
        .filter_map(|instr| match instr {
            (Instruction::Mul(x, y), _) => Some(x * y),
            _ => None,
        })
        .sum::<i32>()
        .to_string()
}

pub(super) fn parse_instruction(input: &str, start: usize) -> Option<(Instruction, usize)> {
    if input.len().saturating_sub(start) < 2 {
        return None;
    }

    if input[start..].starts_with("mul(") {
        parse_mul_at(input, start)
    } else if input[start..].starts_with("do()") {
        Some((Instruction::Do, start + 4))
    } else if input[start..].starts_with("don't()") {
        Some((Instruction::Dont, start + 7))
    } else {
        None
    }
}

fn parse_mul_at(input: &str, start: usize) -> Option<(Instruction, usize)> {
    if input.len().saturating_sub(start) < 4 {
        return None;
    }

    if &input[start..start + 4] != "mul(" {
        return None;
    }

    let rest = &input[start + 4..];
    let comma_pos = rest.find(',')?;
    let x_str = rest[..comma_pos].trim();

    let rest_after_comma = &rest[comma_pos + 1..];
    let paren_pos = rest_after_comma.find(')')?;
    let y_str = rest_after_comma[..paren_pos].trim();

    if is_valid_number(x_str) && is_valid_number(y_str) {
        let x = x_str.parse::<i32>().ok()?;
        let y = y_str.parse::<i32>().ok()?;
        Some((
            Instruction::Mul(x, y),
            start + 4 + paren_pos + comma_pos + 1,
        ))
    } else {
        None
    }
}

fn is_valid_number(s: &str) -> bool {
    let len = s.len();
    (1..=3).contains(&len) && s.chars().all(|c| c.is_ascii_digit())
}
