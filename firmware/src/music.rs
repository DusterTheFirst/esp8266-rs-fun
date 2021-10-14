use crate::{note::MidiNote, notes};

pub static MEGALOVANIA: &[MidiNote] = notes! [
    D   4 for 50 yield 50,
    D   4 for 100,
    D   5 for 100 yield 100,
    A   4 for 200 yield 100,
    Gsh 4 for 100 yield 100,
    G   4 for 100 yield 100,
    F   4 for 200,
    D   4 for 100,
    F   4 for 100,
    G   4 for 100
];

pub static JERK_IT_OUT: &[MidiNote] = notes! [
    Gsh 4 for 500,
    B   4 for 250,
    Gsh 4 for 250,
    B   4 for 200 yield 50,
    B   4 for 200 yield 50,
    Gsh 4 for 250,
    B   4 for 250,
    Fsh 4 for 500,
    Ash 4 for 250,
    Fsh 4 for 250,
    E   4 for 500 yield 1000
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
