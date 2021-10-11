use defmt::Format;
use micromath::F32Ext;

#[macro_export]
macro_rules! notes {
    ($($note:ident $octave:literal for $sustain:literal $(yield for $delay:literal)?),*) => {
        &[
            $(
                MidiNote {
                    tone: $crate::note::NoteLetter::$note.at_octave($octave),
                    sustain: $sustain,
                    delay: 0 $(+ $delay)?
                }
            ),*
        ]
    };
}

#[derive(Clone, Copy, Format)]
pub struct MidiNote {
    /// Underlying note to play
    pub tone: Note,
    /// Time in ms to keep the note playing
    pub sustain: u32,
    /// Time in ms to wait before playing the next note
    pub delay: u32,
}

#[derive(Clone, Copy, Format)]
pub struct Note {
    /// The octave of the note
    pub octave: u32,
    /// The letter of the note
    pub letter: NoteLetter,
}

impl Note {
    /// Calculate the semitones from C0
    pub fn semitones(&self) -> u32 {
        self.octave * 12 + self.letter.semitone()
    }

    /// Calculate the frequency using standard 2^(1/12) tuning
    pub fn frequency(&self) -> f32 {
        const TWO_TO_THE_1_12: f32 = 1.0594630944;

        440.0 * TWO_TO_THE_1_12.powi(self.semitones() as i32 - 57) // A4 is 54 semitones from C0
    }
}

#[allow(unused)]
#[rustfmt::skip]
#[derive(Clone, Copy)]
pub enum NoteLetter {
    C, Csh, D, Dsh, E, F,
    Fsh, G, Gsh, A, Ash, B,
}

impl Format for NoteLetter {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{}", self.name());
    }
}

impl NoteLetter {
    /// Relative semitone of this note to the octave
    pub const fn semitone(&self) -> u32 {
        match self {
            NoteLetter::C => 0,
            NoteLetter::Csh => 1,
            NoteLetter::D => 2,
            NoteLetter::Dsh => 3,
            NoteLetter::E => 4,
            NoteLetter::F => 5,
            NoteLetter::Fsh => 6,
            NoteLetter::G => 7,
            NoteLetter::Gsh => 8,
            NoteLetter::A => 9,
            NoteLetter::Ash => 10,
            NoteLetter::B => 11,
        }
    }

    /// String representation of the note
    pub const fn name(&self) -> &str {
        match self {
            NoteLetter::A => "A",
            NoteLetter::Ash => "A#",
            NoteLetter::B => "B",
            NoteLetter::C => "C",
            NoteLetter::Csh => "C#",
            NoteLetter::D => "D",
            NoteLetter::Dsh => "D#",
            NoteLetter::E => "E",
            NoteLetter::F => "F",
            NoteLetter::Fsh => "F#",
            NoteLetter::G => "G",
            NoteLetter::Gsh => "G#",
        }
    }

    /// Create a note from this letter at the given octave
    pub const fn at_octave(self, octave: u32) -> Note {
        Note {
            octave,
            letter: self,
        }
    }
}
