use base64::{Engine as _, engine::general_purpose};
use num_bigint::BigUint;
use num_traits::{Num, Zero};

pub fn text_to_hex_bytes(text: String) -> String {
    format_bytes(text.as_bytes().iter().copied())
}

pub fn hex_bytes_to_text(text: String) -> String {
    String::from_utf8_lossy(&parse_bytes(&text).unwrap_or_default()).into_owned()
}

pub fn text_to_binary_bytes(text: String) -> String {
    text.as_bytes()
        .iter()
        .map(|byte| format!("{byte:08b}"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn binary_bytes_to_text(text: String) -> String {
    String::from_utf8_lossy(&parse_bytes(&text).unwrap_or_default()).into_owned()
}

pub fn text_to_base64(text: String) -> String {
    general_purpose::STANDARD.encode(text.as_bytes())
}

pub fn base64_to_text(text: String) -> String {
    let normalized = text.split_whitespace().collect::<String>();
    general_purpose::STANDARD
        .decode(normalized)
        .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
        .unwrap_or_default()
}

pub fn escape_html(text: String) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

pub fn unescape_html(text: String) -> String {
    let mut output = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        if chars[index] != '&' {
            output.push(chars[index]);
            index += 1;
            continue;
        }

        let Some(relative_end) = chars[index..].iter().position(|char| *char == ';') else {
            output.push(chars[index]);
            index += 1;
            continue;
        };
        let end = index + relative_end;
        let entity = chars[index + 1..end].iter().collect::<String>();
        let decoded = match entity.as_str() {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => decode_numeric_entity(&entity),
        };

        if let Some(decoded) = decoded {
            output.push(decoded);
            index = end + 1;
        } else {
            output.push(chars[index]);
            index += 1;
        }
    }

    output
}

pub fn text_to_code_points(text: String) -> String {
    text.chars()
        .map(|char| format!("U+{:04X}", char as u32))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn code_points_to_text(text: String) -> String {
    text.split(|char: char| char.is_whitespace() || char == ',' || char == ';')
        .filter(|token| !token.is_empty())
        .filter_map(|token| {
            let hex = token
                .trim_start_matches("U+")
                .trim_start_matches("u+")
                .trim_start_matches("0x")
                .trim_start_matches("0X")
                .trim_start_matches("\\u{")
                .trim_start_matches("\\u")
                .trim_end_matches('}');
            u32::from_str_radix(hex, 16).ok().and_then(char::from_u32)
        })
        .collect()
}

pub fn integer_to_bytes(text: String, little_endian: bool) -> String {
    let Some(value) = parse_integer(&text) else {
        return String::new();
    };

    let mut bytes = value.to_bytes_be();
    if bytes.is_empty() {
        bytes.push(0);
    }
    if little_endian {
        bytes.reverse();
    }

    format_bytes(bytes)
}

pub fn bytes_to_integer(text: String, little_endian: bool) -> String {
    let Ok(mut bytes) = parse_bytes(&text) else {
        return String::new();
    };
    if little_endian {
        bytes.reverse();
    }

    BigUint::from_bytes_be(&bytes).to_string()
}

pub fn reverse_byte_order(text: String) -> String {
    let Ok(mut bytes) = parse_bytes(&text) else {
        return String::new();
    };
    bytes.reverse();
    format_bytes(bytes)
}

fn decode_numeric_entity(entity: &str) -> Option<char> {
    let value = if let Some(hex) = entity
        .strip_prefix("#x")
        .or_else(|| entity.strip_prefix("#X"))
    {
        u32::from_str_radix(hex, 16).ok()?
    } else if let Some(decimal) = entity.strip_prefix('#') {
        decimal.parse::<u32>().ok()?
    } else {
        return None;
    };

    char::from_u32(value)
}

fn parse_integer(text: &str) -> Option<BigUint> {
    let normalized = text.trim().replace('_', "");
    if normalized.is_empty() {
        return Some(BigUint::zero());
    }
    if normalized.starts_with('-') {
        return None;
    }

    if let Some(binary) = normalized
        .strip_prefix("0b")
        .or_else(|| normalized.strip_prefix("0B"))
    {
        return BigUint::from_str_radix(binary, 2).ok();
    }
    if let Some(hex) = normalized
        .strip_prefix("0x")
        .or_else(|| normalized.strip_prefix("0X"))
    {
        return BigUint::from_str_radix(hex, 16).ok();
    }
    BigUint::from_str_radix(&normalized, 10).ok()
}

fn parse_bytes(text: &str) -> Result<Vec<u8>, String> {
    let normalized = text.replace([',', ';'], " ");
    let normalized = normalized.trim();
    if normalized.is_empty() {
        return Ok(Vec::new());
    }

    if normalized.chars().any(char::is_whitespace) {
        return normalized
            .split_whitespace()
            .map(parse_byte_token)
            .collect::<Result<Vec<_>, _>>();
    }

    let chars: Vec<char> = normalized.chars().collect();
    let mut bytes = Vec::new();
    let mut index = 0;

    while index < chars.len() {
        if chars[index] == '0' && index + 1 < chars.len() && matches!(chars[index + 1], 'x' | 'X') {
            index += 2;
        }

        let start = index;
        while index < chars.len() && index - start < 2 && chars[index].is_ascii_hexdigit() {
            index += 1;
        }

        if start == index {
            index += 1;
            continue;
        }

        let token = chars[start..index].iter().collect::<String>();
        bytes.push(parse_byte_token(&token)?);
    }

    Ok(bytes)
}

fn parse_byte_token(token: &str) -> Result<u8, String> {
    let clean = token
        .strip_prefix("0x")
        .or_else(|| token.strip_prefix("0X"))
        .unwrap_or(token);

    if clean.len() == 8 && clean.chars().all(|char| matches!(char, '0' | '1')) {
        return u8::from_str_radix(clean, 2).map_err(|_| token.to_string());
    }

    u8::from_str_radix(clean, 16).map_err(|_| token.to_string())
}

fn format_bytes(bytes: impl IntoIterator<Item = u8>) -> String {
    bytes
        .into_iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_hex_round_trips() {
        let encoded = text_to_hex_bytes("hello 世界".to_string());
        assert_eq!(encoded, "68 65 6C 6C 6F 20 E4 B8 96 E7 95 8C");
        assert_eq!(hex_bytes_to_text(encoded), "hello 世界");
    }

    #[test]
    fn endian_round_trips_large_integers() {
        let input = "305419896".to_string();
        assert_eq!(integer_to_bytes(input.clone(), false), "12 34 56 78");
        assert_eq!(integer_to_bytes(input, true), "78 56 34 12");
        assert_eq!(
            bytes_to_integer("78 56 34 12".to_string(), true),
            "305419896"
        );
    }

    #[test]
    fn code_points_round_trip() {
        let encoded = text_to_code_points("漢字🙂".to_string());
        assert_eq!(encoded, "U+6F22 U+5B57 U+1F642");
        assert_eq!(code_points_to_text(encoded), "漢字🙂");
    }
}
