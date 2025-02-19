```rust
//! Pattern note types
#![allow(
    dead_code,
    unused_imports,
)]

use crate::core::backends::settings::{Octave, Tuning};
use crate::core::pattern::effects::{Effect, EffectArgs};

/// Used to define frequencies for each note based on Tuning
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BaseNote {
    /// Use default tuning A440
    tuning: bool,
}
// impl BaseNote {
//     fn set_a_440(t: self::tuning) -> u32 {
//         if t {
//             440
//         } else {
//             440
//         }
//     }
//     fn set_a_432(t: self::tuning) -> u32 {
//         if !t {
//             432
//         } else {
//             440
//         }
//     }
// }
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Key {

}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Coordinates {
    // x: u32,
    // y: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Note {
    coord: Coordinates,
    key: Key,
    oct: Octave,
    vol: u32,
    fx: Effect,
    fxargs: EffectArgs,
}
/// Keyboard scale related to Tuning
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Scale {
    NoteC,
    NoteCSharp,
    NoteD,
    NoteDSharp,
    NoteE,
    NoteF,
    NoteFSharp,
    NoteG,
    NoteGSharp,
    NoteA,
    NoteASharp,
    NoteB,
}
impl Scale {
    pub const VARIANTS: [Scale; 12] = [
        Scale::NoteC,
        Scale::NoteCSharp,
        Scale::NoteD,
        Scale::NoteDSharp,
        Scale::NoteE,
        Scale::NoteF,
        Scale::NoteFSharp,
        Scale::NoteG,
        Scale::NoteGSharp,
        Scale::NoteA,
        Scale::NoteASharp,
        Scale::NoteB,
    ];
}
impl Default for Scale {
    fn default() -> Self {
        Scale::NoteC
    }
}
```
