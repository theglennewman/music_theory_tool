
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
#[derive(Copy, Clone)]
struct Natural {
    letter: char,    //letter representing the note
    hstep_idx: i16,  //half step index, starting with A at 0
}

const ALL_NATS: [Natural; 7] = [
    Natural::A,
    Natural::B,
    Natural::C,
    Natural::D,
    Natural::E,
    Natural::F,
    Natural::G,
];

struct GenericInterval {
    notes: [Natural; 2]
}

impl GenericInterval {
    // when creating a new GenericInterval, you cannot specify more than
    // an OCTAVE for the interval's "spec"
    pub fn new(root: Natural, spec: i8) -> Self {
        if spec > GenericInterval::OCTAVE {
            let func_name = "GenericInterval::new()";
            println!("\nERROR: Max spec in {} is:", func_name);
            dbg!(GenericInterval::OCTAVE);
            panic!("{} => got spec: {}", func_name, spec);
        }
        println!("GenericInterval new() -> {} / {}", root, spec);
        // iterate through ALL_NATS until we find root specified
        let mut cur_pos = 0;
        let root_pos = loop {
            let cur_nat = ALL_NATS[cur_pos];
            println!("pos {} -> nat: {}", cur_pos, cur_nat);
            if root.letter == cur_nat.letter {
                break cur_pos;
            }
            cur_pos += 1;
        };

        // start at correct position for given root in ALL_NATS and move "spec" number of notes
        cur_pos = root_pos;
        println!("\nstart at: {} -> move {} notes through ALL_NATS", cur_pos, spec);
        let mut found_interval = false;
        while !found_interval {
            // do stuff? I dunno, need a break
            found_interval = true;
            cur_pos += 1;
        }
        
        GenericInterval { 
            notes: [Natural::A, Natural::B]
        }
    }
    const FIRST:   i8 = 0; //firsts: same position on staff, same note
    const SECOND:  i8 = 1; //seconds: one note apart
    const THIRD:   i8 = 2; //thirds: two notes apart
    const FOURTH:  i8 = 3; //etc...
    const FIFTH:   i8 = 4;
    const SIXTH:   i8 = 5;
    const SEVENTH: i8 = 6;
    const OCTAVE:  i8 = 7; //octave is also the max, but it's the same note
}

impl fmt::Debug for GenericInterval {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GenericInterval: {} {}", self.notes[0], self.notes[1])
    }
}

impl fmt::Display for GenericInterval {
    fn fmt (&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.notes[0], self.notes[1])
    }
}

// // 0 1 2 3 4 5 6 7 -> None
    // a b c d e f g a
    // c d e f g a b c
    // -> any time g to a happens, have to loop back around

// I think I need to write it out carefully... I'm not sure what the "loop" counts vs what the internal struct counts

// want an interval a "first" apart = same line / space
// C to C
// interval.next() -> None

// A to B -> a second
// something needs to say START AT A
// interval.next() -> B
// interval.next() -> None

// E to A -> a fourth
// START E
// interval.next() -> F
// interval.next() -> G
// interval.next() -> has to loop back to A!
// interval.next() -> None

// maybe... another way is to just use a list of Naturals
// the tricky part is looping from the end of that array?

// example from rust docs:
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// or this example from rust docs:
// https://doc.rust-lang.org/stable/std/iter/index.html

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

// 

/////////////

// I guess the problem now is...
// - 1 way works: I can go from a Note to a half step index
// - but how do I go from a half step index to a Note?
// - I suppose we have to look at iterators... right?

// Idea: perhaps we do it this way:
// - since you generally figure out the "generic interval" first,
//   you should be able to loop over all the natural notes
// - so determine the "generic" distance to go, then layer on top the accidentals needed
// - this model is similar to Naturals and Accidentals (start with a Natural base, then modify)

// TODO: start developing TEST cases for everything... need to
// get the standard way of doing that

// TODO: function for "get a note that's a half step up from this note"?

// TODO: function for calculating "distance" between two notes?

pub fn debug_note() {
    println!("[note.rs - debug_note()]");

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

    // bad: should panic
    //let my_interval = GenericInterval::new(mynat, 8);
    // CORRECT
    let my_interval = GenericInterval::new(mynat, GenericInterval::OCTAVE);
    println!("\nmy interval...\n{:#?}", my_interval);
    println!("{}", my_interval);
}

