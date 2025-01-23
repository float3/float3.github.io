use std::sync::Weak;

use derivative::Derivative;

use crate::{prebase::ProtoM21Object, specifier::Specifier};

use super::pitch::Pitch;

#[derive(Clone, Debug, Derivative)]
#[derivative(PartialEq)]
pub(crate) struct Accidental {
    proto: ProtoM21Object,
    display_type: String,
    display_status: Option<bool>,
    display_style: String,
    display_size: String,
    display_location: String,
    #[derivative(PartialEq = "ignore")]
    client: Option<Weak<Pitch>>,
    name: String,
    modifier: String,
    pub(crate) alter: f64,
}

impl Accidental {
    pub(crate) fn new_from_string(name: &str) -> Accidental {
        let specifier = match name {
            "natural" => Specifier::Str("natural".to_string()),
            "sharp" => Specifier::Str("sharp".to_string()),
            "flat" => Specifier::Str("flat".to_string()),
            "double-sharp" => Specifier::Str("double-sharp".to_string()),
            "double-flat" => Specifier::Str("double-flat".to_string()),
            "half-sharp" => Specifier::Str("half-sharp".to_string()),
            "half-flat" => Specifier::Str("half-flat".to_string()),
            _ => Specifier::Str("natural".to_string()),
        };
        Accidental::new_from_specifier(Some(specifier))
    }

    pub(crate) fn new_from_specifier(specifier: Option<Specifier>) -> Accidental {
        let specifier = specifier.unwrap_or(Specifier::Str("natural".to_string()));
        let (name, alter) = match specifier {
            Specifier::Int(value) => match value {
                0 => ("natural".to_string(), 0.0),
                1 => ("sharp".to_string(), 1.0),
                -1 => ("flat".to_string(), -1.0),
                _ => ("unknown".to_string(), 0.0),
            },
            Specifier::Str(ref s) => match s.as_str() {
                "natural" => ("natural".to_string(), 0.0),
                "sharp" => ("sharp".to_string(), 1.0),
                "flat" => ("flat".to_string(), -1.0),
                _ => ("unknown".to_string(), 0.0),
            },
            Specifier::Float(value) => ("altered".to_string(), value),
        };

        Accidental {
            proto: ProtoM21Object::new(),
            display_type: "normal".to_string(),
            display_status: None,
            display_style: "normal".to_string(),
            display_size: "full".to_string(),
            display_location: "normal".to_string(),
            client: None,
            name,
            modifier: String::new(),
            alter,
        }
    }

    pub(crate) fn natural() -> Accidental {
        Accidental::new_from_string("natural")
    }

    pub(crate) fn sharp() -> Accidental {
        Accidental::new_from_string("sharp")
    }

    pub(crate) fn flat() -> Accidental {
        Accidental::new_from_string("flat")
    }

    pub(crate) fn double_sharp() -> Accidental {
        Accidental::new_from_string("double-sharp")
    }

    pub(crate) fn double_flat() -> Accidental {
        Accidental::new_from_string("double-flat")
    }

    pub(crate) fn half_sharp() -> Accidental {
        Accidental::new_from_string("half-sharp")
    }

    pub(crate) fn half_flat() -> Accidental {
        Accidental::new_from_string("half-flat")
    }

    pub(crate) fn set_display_type(&mut self, display_type: &str) {
        self.display_type = display_type.to_string();
    }

    pub(crate) fn set_display_status(&mut self, status: Option<bool>) {
        self.display_status = status;
    }
}
