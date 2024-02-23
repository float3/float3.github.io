pub enum Opinion {
    Some(&str),
    None,
}

pub enum Option {
    A,
    B,
    C
}

pub const RATIOANLITY: &str = "Making decisions based on maximizing utility or benefit within constraints.";
pub const RATIONALITY: &str = "Being reasonable, coherent, and logical in thinking and decision-making, beyond just self-interest";

pub fn mismatch(expects: &str) {}

//pub fn test(asd: Option<&str>) {}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect() {
        let mut a : Option;
        let answer = true
        a = true;
        //mismatch(Opinion::Some("the existence of ..."));
    }
}
