use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BayesResult {
    evidence: f64,
    numerator: f64,
    posterior: f64,
    odds: f64,
    error_code: u8,
}

#[wasm_bindgen]
impl BayesResult {
    #[wasm_bindgen(getter)]
    pub fn evidence(&self) -> f64 {
        self.evidence
    }

    #[wasm_bindgen(getter)]
    pub fn numerator(&self) -> f64 {
        self.numerator
    }

    #[wasm_bindgen(getter)]
    pub fn posterior(&self) -> f64 {
        self.posterior
    }

    #[wasm_bindgen(getter)]
    pub fn odds(&self) -> f64 {
        self.odds
    }

    #[wasm_bindgen(getter)]
    pub fn error_code(&self) -> u8 {
        self.error_code
    }
}

#[wasm_bindgen]
pub fn solve_bayes_percent(
    prior_percent: f64,
    likelihood_percent: f64,
    false_positive_percent: f64,
    evidence_percent: f64,
    compute_evidence: bool,
) -> BayesResult {
    let prior = percent_to_probability(prior_percent);
    let likelihood = percent_to_probability(likelihood_percent);
    let false_positive = percent_to_probability(false_positive_percent);
    let computed_evidence = likelihood * prior + false_positive * (1.0 - prior);
    let evidence = if compute_evidence {
        computed_evidence
    } else {
        percent_to_probability(evidence_percent)
    };
    let numerator = likelihood * prior;

    if evidence <= 0.0 {
        return BayesResult {
            evidence,
            numerator,
            posterior: 0.0,
            odds: 0.0,
            error_code: 1,
        };
    }

    let raw_posterior = numerator / evidence;
    let posterior = clamp_probability(raw_posterior);
    let odds = if posterior >= 1.0 {
        f64::INFINITY
    } else {
        posterior / (1.0 - posterior)
    };

    BayesResult {
        evidence,
        numerator,
        posterior,
        odds,
        error_code: if raw_posterior > 1.0 { 2 } else { 0 },
    }
}

#[wasm_bindgen]
pub fn bayes_number(value: f64) -> String {
    format_number(value)
}

fn percent_to_probability(value: f64) -> f64 {
    clamp_probability(value / 100.0)
}

fn clamp_probability(value: f64) -> f64 {
    if !value.is_finite() {
        return 0.0;
    }

    value.clamp(0.0, 1.0)
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "undefined".to_string();
    }

    let mut rounded = format!("{value:.6}");
    while rounded.contains('.') && rounded.ends_with('0') {
        rounded.pop();
    }
    if rounded.ends_with('.') {
        rounded.pop();
    }

    if rounded == "-0" {
        "0".to_string()
    } else {
        rounded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_posterior_with_computed_evidence() {
        let result = solve_bayes_percent(1.0, 90.0, 5.0, 0.0, true);
        assert!((result.evidence - 0.0585).abs() < 0.000001);
        assert!((result.numerator - 0.009).abs() < 0.000001);
        assert!((result.posterior - 0.1538461538).abs() < 0.000001);
        assert_eq!(result.error_code, 0);
    }

    #[test]
    fn reports_zero_evidence() {
        let result = solve_bayes_percent(1.0, 90.0, 5.0, 0.0, false);
        assert_eq!(result.error_code, 1);
    }

    #[test]
    fn formats_numbers_like_the_browser_solver() {
        assert_eq!(format_number(1.230000), "1.23");
        assert_eq!(format_number(-0.0000001), "0");
        assert_eq!(format_number(f64::NAN), "undefined");
    }
}
