//! Reading a standard MIDI file.
//!
//! The playground wants one thing from a `.mid`: the notes, each with a pitch,
//! a loudness, and the moment it starts and stops. That was worth about 200 KB
//! of `@tonejs/midi` in the bundle — a library that builds a whole object model
//! of tracks, instruments, control changes and pitch bends so that the page
//! could read four numbers off each note and throw the rest away.
//!
//! The format is small and fixed (SMF 1.0), so it is read here instead.

/// A note, in the terms the playground plays it back on.
#[derive(Debug, PartialEq)]
pub struct Note {
    pub key: u8,
    /// 0 to 127, as the file writes it and as the Web MIDI API reports it, so
    /// that a file and a plugged-in keyboard speak the same units.
    pub velocity: u8,
    pub start: f64,
    pub end: f64,
}

/// The default tempo the format assumes until a track says otherwise: 120 bpm,
/// which is half a second per quarter note.
const DEFAULT_MICROSECONDS_PER_QUARTER: f64 = 500_000.0;

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }

    fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.at)
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8], String> {
        if self.remaining() < count {
            return Err("the file ends in the middle of an event".to_string());
        }
        let slice = &self.bytes[self.at..self.at + count];
        self.at += count;
        Ok(slice)
    }

    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, String> {
        let bytes = self.take(2)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    fn u32(&mut self) -> Result<u32, String> {
        let bytes = self.take(4)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// A variable-length quantity: seven bits per byte, high bit meaning "more
    /// to come". Delta times and meta-event lengths are both written this way.
    fn variable(&mut self) -> Result<u32, String> {
        let mut value: u32 = 0;
        for _ in 0..4 {
            let byte = self.u8()?;
            value = (value << 7) | u32::from(byte & 0x7f);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err("a variable-length quantity runs past four bytes".to_string())
    }
}

/// One event, reduced to what playback needs.
enum Event {
    NoteOn { key: u8, velocity: u8 },
    NoteOff { key: u8 },
    Tempo { microseconds_per_quarter: f64 },
    Other,
}

/// The notes of a MIDI file, in the order they start.
pub fn parse(bytes: &[u8]) -> Result<Vec<Note>, String> {
    let mut reader = Reader::new(bytes);

    if reader.take(4)? != b"MThd" {
        return Err("not a MIDI file: it does not start with MThd".to_string());
    }
    let header_length = reader.u32()?;
    let header = Reader::new(reader.take(header_length as usize)?);
    let mut header = header;
    let _format = header.u16()?;
    let _tracks = header.u16()?;
    let division = header.u16()?;

    // Collected across every track first, because a tick means nothing until
    // the tempo in force at that tick is known, and in a format 1 file every
    // tempo change lives in the first track while the notes live in the rest.
    let mut tempos: Vec<(u64, f64)> = Vec::new();
    let mut notes: Vec<(u64, u64, u8, u8)> = Vec::new();

    while reader.remaining() >= 8 {
        let kind = reader.take(4)?;
        let length = reader.u32()? as usize;
        let chunk = reader.take(length.min(reader.remaining()))?;

        // The spec says to skip any chunk that is not a track rather than fail.
        if kind != b"MTrk" {
            continue;
        }

        read_track(chunk, &mut tempos, &mut notes)?;
    }

    tempos.sort_by_key(|(tick, _)| *tick);
    let clock = Clock::new(division, &tempos);

    notes.sort_by_key(|(start, ..)| *start);
    Ok(notes
        .into_iter()
        .map(|(start, end, key, velocity)| Note {
            key,
            velocity,
            start: clock.seconds(start),
            end: clock.seconds(end),
        })
        .collect())
}

fn read_track(
    chunk: &[u8],
    tempos: &mut Vec<(u64, f64)>,
    notes: &mut Vec<(u64, u64, u8, u8)>,
) -> Result<(), String> {
    let mut track = Reader::new(chunk);
    let mut tick: u64 = 0;
    let mut status: Option<u8> = None;

    // Keys still down, per track: the same key can be held in two tracks at
    // once, and closing the wrong one would swap the two notes' lengths.
    let mut sounding: Vec<(u64, u8, u8)> = Vec::new();

    while track.remaining() > 0 {
        tick += u64::from(track.variable()?);

        match read_event(&mut track, &mut status)? {
            Event::Tempo {
                microseconds_per_quarter,
            } => tempos.push((tick, microseconds_per_quarter)),
            Event::NoteOn { key, velocity } => sounding.push((tick, key, velocity)),
            Event::NoteOff { key } => {
                // The last matching note down is the one this closes.
                if let Some(index) = sounding.iter().rposition(|(_, down, _)| *down == key) {
                    let (began, key, velocity) = sounding.remove(index);
                    notes.push((began, tick, key, velocity));
                }
            }
            Event::Other => {}
        }
    }

    // A file that never lifts a key is malformed, but dropping the note would
    // be a silent hole in the playback; end it where the track ends instead.
    for (began, key, velocity) in sounding {
        notes.push((began, tick, key, velocity));
    }

    Ok(())
}

fn read_event(track: &mut Reader<'_>, status: &mut Option<u8>) -> Result<Event, String> {
    let mut byte = track.u8()?;

    // Running status: a channel event may leave its status byte out and mean
    // the last one. Anything below 0x80 is the first data byte of such an event.
    if byte < 0x80 {
        track.at -= 1;
        byte = status.ok_or_else(|| "an event begins with no status byte".to_string())?;
    } else if byte < 0xf0 {
        *status = Some(byte);
    } else {
        *status = None;
    }

    match byte & 0xf0 {
        0x80 => {
            let key = track.u8()?;
            let _velocity = track.u8()?;
            Ok(Event::NoteOff { key })
        }
        0x90 => {
            let key = track.u8()?;
            let velocity = track.u8()?;
            // A note on with no force behind it is how most files lift a key.
            Ok(if velocity == 0 {
                Event::NoteOff { key }
            } else {
                Event::NoteOn { key, velocity }
            })
        }
        0xa0 | 0xb0 | 0xe0 => {
            track.take(2)?;
            Ok(Event::Other)
        }
        0xc0 | 0xd0 => {
            track.take(1)?;
            Ok(Event::Other)
        }
        _ => match byte {
            0xff => {
                let kind = track.u8()?;
                let length = track.variable()? as usize;
                let data = track.take(length)?;
                Ok(if kind == 0x51 && data.len() == 3 {
                    Event::Tempo {
                        microseconds_per_quarter: f64::from(u32::from_be_bytes([
                            0, data[0], data[1], data[2],
                        ])),
                    }
                } else {
                    Event::Other
                })
            }
            0xf0 | 0xf7 => {
                let length = track.variable()? as usize;
                track.take(length)?;
                Ok(Event::Other)
            }
            other => Err(format!("unknown MIDI status byte {other:#04x}")),
        },
    }
}

/// Turns ticks into seconds, following the tempo changes as it goes.
struct Clock {
    /// Seconds per tick, and the tick it takes effect at, in order.
    rates: Vec<(u64, f64)>,
}

impl Clock {
    fn new(division: u16, tempos: &[(u64, f64)]) -> Self {
        // The top bit picks the unit: clear means ticks per quarter note, set
        // means SMPTE, where the tick length is fixed and tempo is irrelevant.
        if division & 0x8000 != 0 {
            let frames_per_second = match -((division >> 8) as i8) {
                29 => 29.97,
                frames => f64::from(frames),
            };
            let ticks_per_frame = f64::from(division & 0xff);
            let seconds_per_tick = 1.0 / (frames_per_second * ticks_per_frame).max(1.0);
            return Self {
                rates: vec![(0, seconds_per_tick)],
            };
        }

        let ticks_per_quarter = f64::from(division.max(1));
        let seconds_per_tick =
            |microseconds_per_quarter: f64| microseconds_per_quarter / 1e6 / ticks_per_quarter;

        let mut rates = vec![(0, seconds_per_tick(DEFAULT_MICROSECONDS_PER_QUARTER))];
        for (tick, microseconds_per_quarter) in tempos {
            let rate = seconds_per_tick(*microseconds_per_quarter);
            if *tick == 0 {
                rates[0].1 = rate;
            } else {
                rates.push((*tick, rate));
            }
        }

        Self { rates }
    }

    fn seconds(&self, tick: u64) -> f64 {
        let mut seconds = 0.0;
        let mut previous = 0u64;

        for (index, (_, rate)) in self.rates.iter().enumerate() {
            let next = self.rates.get(index + 1).map_or(u64::MAX, |(at, _)| *at);

            if tick <= previous {
                break;
            }

            let until = tick.min(next);
            seconds += (until - previous) as f64 * rate;
            previous = until;
        }

        seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A one-track file: middle C for one quarter note, then E for one, at the
    /// default 120 bpm, with 96 ticks to the quarter.
    fn file() -> Vec<u8> {
        let mut track: Vec<u8> = Vec::new();
        // delta 0, note on C4 velocity 64
        track.extend([0x00, 0x90, 60, 64]);
        // delta 96, note off C4
        track.extend([0x60, 0x80, 60, 0]);
        // delta 0, note on E4 velocity 127 (running status left out)
        track.extend([0x00, 0x90, 64, 127]);
        // delta 96, note on E4 velocity 0, which means off
        track.extend([0x60, 64, 0]);
        // end of track
        track.extend([0x00, 0xff, 0x2f, 0x00]);

        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(b"MThd");
        bytes.extend(6u32.to_be_bytes());
        bytes.extend(0u16.to_be_bytes());
        bytes.extend(1u16.to_be_bytes());
        bytes.extend(96u16.to_be_bytes());
        bytes.extend(b"MTrk");
        bytes.extend((track.len() as u32).to_be_bytes());
        bytes.extend(track);
        bytes
    }

    #[test]
    fn reads_notes_with_their_times_and_velocities() {
        let notes = parse(&file()).unwrap();

        assert_eq!(notes.len(), 2);
        assert_eq!(notes[0].key, 60);
        assert_eq!(notes[0].velocity, 64);
        assert!((notes[0].start - 0.0).abs() < 1e-9);
        assert!((notes[0].end - 0.5).abs() < 1e-9, "{}", notes[0].end);

        assert_eq!(notes[1].key, 64);
        assert_eq!(notes[1].velocity, 127);
        assert!((notes[1].start - 0.5).abs() < 1e-9);
        assert!((notes[1].end - 1.0).abs() < 1e-9);
    }

    /// Running status is not an exotic corner: most files lean on it heavily,
    /// and a reader that does not follow it desynchronises on the first note.
    #[test]
    fn follows_running_status() {
        let notes = parse(&file()).unwrap();
        assert_eq!(notes[1].end - notes[1].start, 0.5);
    }

    #[test]
    fn a_tempo_change_moves_everything_after_it() {
        let mut bytes = file();
        // Rewrite the file with a tempo of 240 bpm set at tick 0.
        let mut track: Vec<u8> = Vec::new();
        track.extend([0x00, 0xff, 0x51, 0x03]);
        track.extend([0x03, 0xd0, 0x90]); // 250_000 microseconds per quarter
        track.extend([0x00, 0x90, 60, 64]);
        track.extend([0x60, 0x80, 60, 0]);
        track.extend([0x00, 0xff, 0x2f, 0x00]);

        bytes.truncate(14);
        bytes.extend(b"MTrk");
        bytes.extend((track.len() as u32).to_be_bytes());
        bytes.extend(track);

        let notes = parse(&bytes).unwrap();
        assert_eq!(notes.len(), 1);
        assert!((notes[0].end - 0.25).abs() < 1e-9, "{}", notes[0].end);
    }

    #[test]
    fn refuses_something_that_is_not_a_midi_file() {
        assert!(parse(b"this is not a midi file at all").is_err());
    }

    #[test]
    fn a_key_never_lifted_still_sounds() {
        let mut track: Vec<u8> = Vec::new();
        track.extend([0x00, 0x90, 60, 64]);
        track.extend([0x60, 0xff, 0x2f, 0x00]);

        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend(b"MThd");
        bytes.extend(6u32.to_be_bytes());
        bytes.extend(0u16.to_be_bytes());
        bytes.extend(1u16.to_be_bytes());
        bytes.extend(96u16.to_be_bytes());
        bytes.extend(b"MTrk");
        bytes.extend((track.len() as u32).to_be_bytes());
        bytes.extend(track);

        let notes = parse(&bytes).unwrap();
        assert_eq!(notes.len(), 1);
        assert!((notes[0].end - 0.5).abs() < 1e-9);
    }
}
