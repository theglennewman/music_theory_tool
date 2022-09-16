
// Implementing musical notes by imagining a piano and representing each note's
// position with a "half step index."
// With this idea in mind:
// - hstep_idx starts with A natural at 0
// - hstep_idx of B natural is 2 because it is two half steps up from A on the piano
// - there is no black key between B and C, so the hstep_idx of C is 3
// - etc...
// - starting at G and moving up two half-steps would loop around and land at A
// - starting at A and moving down two half-steps would loop back to G

use std::fmt;

// import our interval module (interval.rs)
// ... I *think* the 'crate' part of this means "use interval from the crate we're in"
use crate::interval;

// Natural represents a natural note, or a white key on a piano.
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Natural {
    pub letter: char,    //letter representing the note
    pub hstep_idx: i16,  //half step index, starting with A at 0
}

impl Natural {
    pub const A: Self = Self {
        letter: 'A',
        hstep_idx: 0,
    };
    pub const B: Self = Self {
        letter: 'B',
        hstep_idx: 2,
    };
    pub const C: Self = Self {
        letter: 'C',
        hstep_idx: 3, //no black keys between B and C
    };
    pub const D: Self = Self {
        letter: 'D',
        hstep_idx: 5,
    };
    pub const E: Self = Self {
        letter: 'E',
        hstep_idx: 7,
    };
    pub const F: Self = Self {
        letter: 'F',
        hstep_idx: 8, //no black keys between E and F
    };
    pub const G: Self = Self {
        letter: 'G',
        hstep_idx: 10,
    };
    // there is one half step (black key) between G and A... so that needs to be
    // handled when looping from G to A or vice versa. Two half steps are needed
    // to move from one or the other.
}

impl fmt::Debug for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Natural: {}[{}]", self.letter, self.hstep_idx)
    }
}

impl fmt::Display for Natural {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.letter)
    }
}

// Accidental is a modification to a Natural - it gets a symbol and the
// number of half steps up or down to modify the Natural
struct Accidental {
    pub symbol: char,
    pub hstep_mod: i16,
}

impl Accidental {
    pub const DOUBLEFLAT: Self = Self {
        symbol: '𝄫',
        hstep_mod: -2,
    };
    pub const FLAT: Self = Self {
        symbol: '♭',
        hstep_mod: -1,
    };
    pub const NATURAL: Self = Self {
        symbol: '♮',
        hstep_mod: 0,
    };
    pub const SHARP: Self = Self {
        symbol: '♯',
        hstep_mod: 1,
    };
    pub const DOUBLESHARP: Self = Self {
        symbol: '𝄪',
        hstep_mod: 2,
    };
}

impl fmt::Debug for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Accidental: {}[{}]", self.symbol, self.hstep_mod)
    }
}

impl fmt::Display for Accidental {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

// Note has a Natural and an Accidental. The Natural's "half step index"
// is modified by the "half step modification" of its Accidental which
// results in its "final half step index."
struct Note {
    pub nat: Natural,
    pub acc: Accidental,
    pub hstep_idx: i16,
}

impl Note {
    pub fn new(nat: Natural, acc: Accidental) -> Self {
        let hstep_idx = nat.hstep_idx + acc.hstep_mod;
        Self { nat, acc, hstep_idx }
    }
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Note: {}{}[{}]", self.nat, self.acc, self.hstep_idx)
    }
}

impl fmt::Display for Note {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.nat, self.acc)
    }
}

// TODO: create test cases / asserts for every struct here
// - what's the rust convention for where you put all the asserts?
// - in another file for test cases?

pub fn debug_note() {
    let func_str = "note.rs / debug_note()";
    println!("[START {}]", func_str);

    //let my_var = Natural::B;
    let mynat = Natural::F;
    println!("\n{:#?}", mynat);
    println!("{}", mynat);
    
    let myacc = Accidental::SHARP;
    println!("\n{:#?}", myacc);
    println!("{}", myacc);

    let mynote = Note::new(mynat, myacc);
    println!("\n{:#?}", mynote);
    println!("{}", mynote);

    println!("[DONE  {}]", func_str);
}

