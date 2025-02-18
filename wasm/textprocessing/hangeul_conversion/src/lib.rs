use block::Block;

mod block;
mod consonant;
pub mod hangeul;
pub mod mr;
pub mod rr;
mod vowel;

trait Index {
    fn index(&self) -> usize;
}

pub trait Print {
    fn revised_romanization(&self) -> String;
    fn mccune_reischauer_romanization(&self) -> String;
    fn hangeul(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::Block;
    use crate::consonant::Consonant;
    use crate::mr;
    use crate::rr;
    use crate::vowel::Vowel;

    #[test]
    fn test_printing() {
        // Example: 가 = ㄱ(initial) + ㅏ(medial)
        let block = Block {
            initial: Consonant::Giyeok(true),
            medial: Vowel::A,
            r#final: None,
        };
        assert_eq!(block.revised_romanization(), "ga");
        assert_eq!(block.mccune_reischauer_romanization(), "ka");
        assert_eq!(block.hangeul(), "가");
    }

    #[test]
    fn test_rr_parsing() {
        // "han" should parse as ㅎ+ㅏ+ㄴ.
        let input = "han";
        let blocks = rr::parse(input).expect("RR parse failed");
        assert_eq!(blocks.len(), 1);
        let block = &blocks[0];
        assert_eq!(block.revised_romanization(), "han");
        assert_eq!(block.hangeul(), "한");
    }

    #[test]
    fn test_mr_parsing() {
        // MR: "han" should parse similarly.
        let input = "han";
        let blocks = mr::parse(input).expect("MR parse failed");
        assert_eq!(blocks.len(), 1);
        let block = &blocks[0];
        assert_eq!(block.mccune_reischauer_romanization(), "han");
        assert_eq!(block.hangeul(), "한");
    }

    #[test]
    fn test_hangeul_parsing() {
        // Parse a full syllable string.
        let input = "한글";
        let blocks = hangeul::parse(input).expect("Hangul parse failed");
        assert_eq!(blocks.len(), 2);
        let recomposed: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(recomposed, input);
    }

    #[test]
    fn test_multiple_blocks_rr() {
        // "hanguk" -> 한(ㅎ+ㅏ+ㄴ) 국(ㄱ+ㅜ+ㄱ)
        let input = "hanguk";
        let blocks = rr::parse(input).expect("RR multiple block parse failed");
        assert_eq!(blocks.len(), 2);
        let recomposed: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(recomposed, "한국");
    }

    #[test]
    fn test_round_trip_rr() {
        let input = "hanguk";
        let blocks = rr::parse(input).expect("RR parse failed");
        let hangul_str: String = blocks.iter().map(|b| b.hangeul()).collect();
        let blocks2 = hangeul::parse(&hangul_str).expect("Hangul re-parse failed");
        let rr_str: String = blocks2.iter().map(|b| b.revised_romanization()).collect();
        assert_eq!(rr_str, "hanguk");
    }

    #[test]
    fn test_round_trip_mr() {
        let input = "hanguk";
        let blocks = mr::parse(input).expect("MR parse failed");
        let hangul_str: String = blocks.iter().map(|b| b.hangeul()).collect();
        let blocks2 = hangeul::parse(&hangul_str).expect("Hangul re-parse failed");
        let mr_str: String = blocks2
            .iter()
            .map(|b| b.mccune_reischauer_romanization())
            .collect();
        assert_eq!(mr_str, "hankuk");
    }

    #[test]
    fn test_hangeul_round_trip() {
        let input = "한국";
        let blocks = hangeul::parse(input).expect("Hangul parse failed");
        let recomposed: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(recomposed, input);
    }

    #[test]
    fn test_hangeul_to_rr() {
        let input = "한국";
        let blocks = hangeul::parse(input).expect("Hangul parse failed");
        let rr_str: String = blocks.iter().map(|b| b.revised_romanization()).collect();
        assert_eq!(rr_str, "hanguk");
    }

    #[test]
    fn test_hangeul_to_mr() {
        let input = "한국";
        let blocks = hangeul::parse(input).expect("Hangul parse failed");
        let mr_str: String = blocks
            .iter()
            .map(|b| b.mccune_reischauer_romanization())
            .collect();
        assert_eq!(mr_str, "hankuk");
    }

    #[test]
    fn test_rr_with_spaces() {
        let input = "han guk";
        let blocks = rr::parse(input).expect("RR parse failed with spaces");
        let hangul_str: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(hangul_str, "한국");
    }

    #[test]
    fn test_mr_with_spaces() {
        let input = "han guk";
        let blocks = mr::parse(input).expect("MR parse failed with spaces");
        let hangul_str: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(hangul_str, "한국");
    }

    #[test]
    fn test_empty_input_rr() {
        let input = "";
        let blocks = rr::parse(input).unwrap();
        assert_eq!(blocks.len(), 0);
    }

    #[test]
    fn test_empty_input_mr() {
        let input = "";
        let blocks = mr::parse(input).unwrap();
        assert_eq!(blocks.len(), 0);
    }

    #[test]
    fn test_invalid_input_rr() {
        let input = "123";
        let blocks = rr::parse(input);
        assert!(blocks.is_none());
    }

    #[test]
    fn test_invalid_input_mr() {
        let input = "abc123";
        let blocks = mr::parse(input);
        assert!(blocks.is_none());
    }

    #[test]
    fn test_all_vowels() {
        // For every vowel, create a block with default silent initial.
        for &(_, vowel) in rr::VOWELS.iter() {
            let block = Block {
                initial: Consonant::Ieung(true),
                medial: vowel,
                r#final: None,
            };
            let ch = block.hangeul().chars().next().unwrap();
            assert!((0xAC00..=0xD7A3).contains(&(ch as u32)));
        }
    }

    #[test]
    fn test_all_initials() {
        // For every initial from the RR table.
        for &(_, cons) in rr::INITIALS.iter() {
            let block = Block {
                initial: cons,
                medial: Vowel::A,
                r#final: None,
            };
            let ch = block.hangeul().chars().next().unwrap();
            assert!((0xAC00..=0xD7A3).contains(&(ch as u32)));
        }
    }

    #[test]
    fn test_complex_sentence() {
        // "hanguk saram" should produce blocks for "한국사람"
        let input = "hanguk saram";
        let blocks = rr::parse(input).expect("RR parse failed");
        let hangul_str: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(hangul_str, "한국사람");
    }

    #[test]
    fn test_ambiguous_final_dang() {
        // "dang" should parse as ㄷ+ㅏ+ㅇ -> "당"
        let input = "dang";
        let blocks = rr::parse(input).expect("RR parse failed");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].hangeul(), "당");
        assert_eq!(blocks[0].revised_romanization(), "dang");
    }

    #[test]
    fn test_rr_mr_consistency() {
        // RR and MR should yield the same Hangul for the same input.
        let input = "hanguk";
        let blocks_rr = rr::parse(input).expect("RR parse failed");
        let blocks_mr = mr::parse(input).expect("MR parse failed");
        let hangul_rr: String = blocks_rr.iter().map(|b| b.hangeul()).collect();
        let hangul_mr: String = blocks_mr.iter().map(|b| b.hangeul()).collect();
        assert_eq!(hangul_rr, hangul_mr);
    }

    #[test]
    fn test_specific_syllable_chae() {
        // "chae" should parse to 채
        let input = "chae";
        let blocks = rr::parse(input).expect("RR parse failed");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].hangeul(), "채");
        assert_eq!(blocks[0].revised_romanization(), "chae");
    }

    #[test]
    fn test_double_initial_kkuk() {
        // "kkuk" should yield 꿀: initial "kk", vowel "u", final "k"
        let input = "kkul";
        let blocks = rr::parse(input).expect("RR parse failed");
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].hangeul(), "꿀");
        assert_eq!(blocks[0].revised_romanization(), "kkul");
    }

    #[test]
    fn test_seoul_rr() {
        // "seoul" in RR should yield 서울.
        let input = "seoul";
        let blocks = rr::parse(input).expect("RR parse failed");
        let hangul_str: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(hangul_str, "서울");
    }

    #[test]
    fn test_saram_mr() {
        // "saram" in MR should yield 사람.
        let input = "saram";
        let blocks = mr::parse(input).expect("MR parse failed");
        let hangul_str: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(hangul_str, "사람");
    }

    #[test]
    fn test_full_system_conversion() {
        // Convert from RR to Hangul and then to MR.
        let input_rr = "hanguk";
        let blocks_rr = rr::parse(input_rr).expect("RR parse failed");
        let hangul_str: String = blocks_rr.iter().map(|b| b.hangeul()).collect();
        let blocks_mr = mr::parse(&hangul_str).expect("MR parse failed");
        let mr_str: String = blocks_mr
            .iter()
            .map(|b| b.mccune_reischauer_romanization())
            .collect();
        assert_eq!(mr_str, "hankuk");
    }

    #[test]
    fn mr_to_rr() {
        let blocks: Vec<Block> = rr::parse("jaewonhanguk").unwrap();
        let mccune: String = blocks
            .iter()
            .map(|b| b.mccune_reischauer_romanization())
            .collect();
        let hangeul: String = blocks.iter().map(|b| b.hangeul()).collect();

        let mccuneblocks: Vec<Block> = mr::parse(&mccune).unwrap();
        assert_eq!(blocks, mccuneblocks);
        let hangeulblocks: Vec<Block> = hangeul::parse(&hangeul).unwrap();
        assert_eq!(blocks, hangeulblocks);
    }

    #[test]
    fn test_round_trip_between_all_methods() {
        // Start with Hangul, convert to RR and MR, then back to Hangul.
        let input = "사랑";
        let blocks_hangeul = hangeul::parse(input).expect("Hangul parse failed");
        let rr_str: String = blocks_hangeul
            .iter()
            .map(|b| b.revised_romanization())
            .collect();
        let blocks_rr = rr::parse(&rr_str).expect("RR parse failed");
        let hangul_from_rr: String = blocks_rr.iter().map(|b| b.hangeul()).collect();
        assert_eq!(hangul_from_rr, input);
        let mr_str: String = blocks_hangeul
            .iter()
            .map(|b| b.mccune_reischauer_romanization())
            .collect();
        println!("MR: {}", mr_str);
        let blocks_mr = mr::parse(&mr_str).expect("MR parse failed");
        let hangul_from_mr: String = blocks_mr.iter().map(|b| b.hangeul()).collect();
        assert_eq!(hangul_from_mr, input);
    }

    #[test]
    fn test_consistency_of_printing() {
        // For a given block, all printing methods must yield consistent outputs.
        let block = Block {
            initial: Consonant::Giyeok(true),
            medial: Vowel::A,
            r#final: Some(Consonant::Nieun(false)),
        };
        let hangul_char = block.hangeul();
        assert!(!hangul_char.is_empty());
        assert!(!block.revised_romanization().is_empty());
        assert!(!block.mccune_reischauer_romanization().is_empty());
    }

    #[test]
    fn test_parse_hangeul_compound() {
        // Parse a compound Hangul string.
        let input = "대한민국";
        let blocks = hangeul::parse(input).expect("Hangul parse failed");
        let recomposed: String = blocks.iter().map(|b| b.hangeul()).collect();
        assert_eq!(recomposed, input);
    }

    #[test]
    fn test_random_example() {
        // A random RR input should round-trip.
        let input = "sarang";
        let blocks = rr::parse(input).expect("RR parse failed");
        let hangul_str: String = blocks.iter().map(|b| b.hangeul()).collect();
        let blocks2 = hangeul::parse(&hangul_str).expect("Hangul parse failed");
        let rr_str: String = blocks2.iter().map(|b| b.revised_romanization()).collect();
        assert_eq!(rr_str, input);
    }
}
