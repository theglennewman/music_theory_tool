
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

// Natural represents a natural note, or a white key on a piano.
struct Natural {
    letter: char,    //letter representing the note
    hstep_idx: i16,  //half step index, starting with A at 0
}

impl Natural {
    const A: Self = Self {
        letter: 'A',
        hstep_idx: 0,
    };
    const B: Self = Self {
        letter: 'B',
        hstep_idx: 2,
    };
    const C: Self = Self {
        letter: 'C',
        hstep_idx: 3, //no black keys between B and C
    };
    const D: Self = Self {
        letter: 'D',
        hstep_idx: 5,
    };
    const E: Self = Self {
        letter: 'E',
        hstep_idx: 7,
    };
    const F: Self = Self {
        letter: 'F',
        hstep_idx: 8, //no black keys between E and F
    };
    const G: Self = Self {
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
    symbol: char,
    hstep_mod: i16,
}

impl Accidental {
    const DOUBLEFLAT: Self = Self {
        symbol: '𝄫',
        hstep_mod: -2,
    };
    const FLAT: Self = Self {
        symbol: '♭',
        hstep_mod: -1,
    };
    const NATURAL: Self = Self {
        symbol: '♮',
        hstep_mod: 0, //natural note's pitch does not get modified
    };
    const SHARP: Self = Self {
        symbol: '♯',
        hstep_mod: 1,
    };
    const DOUBLESHARP: Self = Self {
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
    nat: Natural,
    acc: Accidental,
    hstep_idx: i16,
}

impl Note {
    // convention is to use an associated function "new" to create an object
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

// NEXT! let's work on "distance"
// - i think two notes have a distance...
// - also a built note has a distance between itself and another note

// NEXT
// function for "from a note, get a note that's a half step up"

pub fn debug_note() {
    println!("[note.rs - debug_note()]");

    let mynat = Natural::G;
    println!("\n{:#?}", mynat);
    println!("{}", mynat);
    
    let myacc = Accidental::DOUBLEFLAT;
    println!("\n{:#?}", myacc);
    println!("{}", myacc);

    let mynote = Note::new(mynat, myacc);
    println!("\n{:#?}", mynote);
    println!("{}", mynote);

}

