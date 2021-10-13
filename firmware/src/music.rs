use crate::{note::MidiNote, notes};

// static MEGALOVANIA: &[MidiNote] = &[
//     MidiNote {
//         freq: Hertz(293),
//         sustain: Milliseconds(50),
//         delay: Milliseconds(50),
//     },
//     MidiNote {
//         freq: Hertz(293),
//         sustain: Milliseconds(100),
//         delay: Milliseconds(0),
//     },
//     MidiNote {
//         freq: Hertz(587),
//         sustain: Milliseconds(100),
//         delay: Milliseconds(100),
//     },
//     MidiNote {
//         freq: Hertz(440),
//         sustain: Milliseconds(200),
//         delay: Milliseconds(100),
//     },
//     MidiNote {
//         freq: Hertz(415),
//         sustain: Milliseconds(100),
//         delay: Milliseconds(100),
//     },
//     MidiNote {
//         freq: Hertz(392),
//         sustain: Milliseconds(100),
//         delay: Milliseconds(100),
//     },
//     MidiNote {
//         freq: Hertz(349),
//         sustain: Milliseconds(200),
//         delay: Milliseconds(0),
//     },
//     MidiNote {
//         freq: Hertz(293),
//         sustain: Milliseconds(100),
//         delay: Milliseconds(0),
//     },
//     MidiNote {
//         freq: Hertz(349),
//         sustain: Milliseconds(100),
//         delay: Milliseconds(0),
//     },
//     MidiNote {
//         freq: Hertz(392),
//         sustain: Milliseconds(100),
//         delay: Milliseconds(0),
//     },
// ];

pub static JERK_IT_OUT: &[MidiNote] = notes! [
    Gsh 4 for 500,
    B   4 for 250,
    Gsh 4 for 250,
    B   4 for 200 yield for 50,
    B   4 for 200 yield for 50,
    Gsh 4 for 500,
    Fsh 4 for 500,
    Ash 4 for 250,
    Fsh 4 for 250,
    E   4 for 500 yield for 1000
];

pub static THE_GOOD_LIFE: &[MidiNote] = notes! [
    Dsh 5 for 250,
    Fsh 5 for 250,
    Fsh 5 for 250,
    Fsh 5 for 250,
    F   5 for 250,
    F   5 for 250,
    Dsh 5 for 250,
    F   5 for 500,
    Dsh 5 for 500,
    Ash 4 for 500,
    Gsh 4 for 250,
    Gsh 4 for 500,
    F   5 for 250,
    F   5 for 250,
    F   5 for 250,
    F   5 for 250,
    Dsh 5 for 250,
    Dsh 5 for 250,
    Csh 5 for 250,
    Dsh 5 for 500,
    Csh 5 for 500,
    Ash 4 for 500,
    Gsh 4 for 250,
    Fsh 4 for 500
];
