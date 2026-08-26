use tuningplayground::{convert_notes_core, midi};

fn main() {
    println!(
        "{}",
        convert_notes_core(vec!["C4".into(), "E-4".into(), "G4".into()])
    );

    let path = "content/misc/blobs/jm_mozdi.mid";
    match std::fs::read(path) {
        Ok(bytes) => match midi::parse(&bytes) {
            Ok(notes) => {
                let last = notes.iter().fold(0.0f64, |end, note| end.max(note.end));
                println!(
                    "{path}: {} notes, {:.2}s, first {:?}",
                    notes.len(),
                    last,
                    notes.first()
                );
            }
            Err(err) => println!("{path}: {err}"),
        },
        Err(err) => println!("{path}: {err}"),
    }
}
