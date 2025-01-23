use crate::pitch::pitch::Pitch;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Note {
    pub(crate) pitch: Pitch,
}

impl<'a, 'b> Note {
    pub(crate) fn new(string: String) -> Note {
        let pitch = Pitch::new(string);
        Note { pitch }
    }
}

// #[derive(Clone)]
// pub(crate) enum Note {
//     Specific(Pitch),
//     General(Pitch),
//     Chord(Vec<Note>),
// }

// impl Note {
//     pub(crate) fn new(to_string: String) -> Note {
//         todo!()
//     }

//     pub(crate) fn pitches(&self) -> Vec<Pitch> {
//         match self {
//             Note::Specific(pitch) => vec![pitch.clone()],
//             Note::General(pitch) => vec![pitch.clone()],
//             Note::Chord(notes) => notes.iter().map(|note| note.pitches()).flatten().collect(),
//         }
//     }
// }
