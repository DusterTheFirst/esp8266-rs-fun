use defmt::Format;
use micromath::F32Ext;
use num_rational::Ratio;

#[macro_export]
macro_rules! notes {
    (
        $($note:ident $octave:literal for $sustain:literal $(/ $sustain_frac:literal)? $(yield $rest:literal $(/ $rest_frac:literal)?)?,)*
    ) => {
        &[
            $(
                $crate::note::MusicalNote {
                    letter: $crate::note::NoteLetter::$note,
                    octave: $octave,
                    sustain: num_rational::Ratio::new_raw($sustain, 1 $( - 1 + $sustain_frac)?),
                    rest: num_rational::Ratio::new_raw(0 $(+ $rest)?, 1 $($( - 1 + $rest_frac)?)?)
                }
            ),*
        ]
    };
}

#[derive(Clone, Copy)]
pub struct MusicalNote {
    /// The octave of the note
    pub octave: u32,
    /// The letter of the note
    pub letter: NoteLetter,
    /// Beats to sustain for
    pub sustain: Ratio<u32>,
    /// Rest, if any, that appears after this note
    pub rest: Ratio<u32>,
}

impl Format for MusicalNote {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "MusicalNote {{ octave: {=u32}, letter: {} sustain: {=u32}/{=u32}, rest: {=u32}/{=u32} }}",
            self.octave,
            self.letter,
            self.sustain.numer(),
            self.sustain.denom(),
            self.rest.numer(),
            self.rest.denom()
        );
    }
}

#[derive(Clone, Copy, Format)]
pub struct Music {
    pub title: &'static str,
    pub bpm: u32,
    pub track_1: &'static [MusicalNote],
    pub track_2: &'static [MusicalNote],
}

impl MusicalNote {
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
        defmt::write!(fmt, "{=str}", self.name());
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
}
