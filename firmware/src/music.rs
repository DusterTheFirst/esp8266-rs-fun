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

// static JERK_IT_OUT: &[MidiNote] = notes! {
//     (
//         frequency = Hertz,
//         time = Milliseconds
//     )
//     [
//         415 for 500;
//         493 for 250;
//         415 for 250;
//         493 for 200 yield for 50;
//         493 for 200 yield for 50;
//         415 for 500;
//         370 for 500;
//         466 for 250;
//         370 for 250;
//         330 for 500 yield for 1000;
//         // 415 for 200 yield for 50;
//         // 415 for 200;
//     ]
// };

pub static THE_GOOD_LIFE: &[MidiNote] = notes! [
    Fsh 5 for 500,
    Fsh 5 for 500,
    F   5 for 500,
    F   5 for 500,
    Dsh 5 for 500,
    Fsh 5 for 1000,
    Dsh 5 for 1000,
    Ash 4 for 1000,
    Gsh 4 for 500,
    Fsh 4 for 500
];
