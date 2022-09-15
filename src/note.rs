
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

// TODO: we could also refer to this as "staff position"
// since we're using these to generate GenericIntervals.
// C-C# is still a first (when measuring generic intervals)
// and the docs say that "only staff position matters"
const ALL_NATS: [Natural; 7] = [
    Natural::A,
    Natural::B,
    Natural::C,
    Natural::D,
    Natural::E,
    Natural::F,
    Natural::G,
];

#[derive(Copy, Clone, PartialEq, Eq)]
struct GenericInterval {
    notes: [Natural; 2]
}

// TODO: need to really clean up these debug prints once we're happy with new()
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
        // println!("GenericInterval new() -> {} / {}", root, spec);

        // we don't know which natural to START at until we find the position
        // in ALL_NATS which is the root specified
        let mut aa = 0; //current index as we loop thru ALL_NATS
        let mut ss = 0; //number of steps we've taken after finding root

        //have we determined root's pos in ALL_NATS yet?
        let mut still_searching_root = true; 

        // determine the interval (second note) by finding the root note's
        // index (aa) in ALL_NATS
        // then incrementing "spec" number of steps (ss) through ALL_NATS
        let interval = loop {
            // print!("\nSTART LOOP: aa {} - ss {}", aa, ss);
            let cur_nat = ALL_NATS[aa];
            // println!(" - cur_nat: {}",cur_nat);                  
            
            if still_searching_root && root.letter == cur_nat.letter {
                // println!("** found root {} ... aa {} - ss {}", cur_nat, aa, ss);
                still_searching_root = false;
                // println!("we will start incrementing ss");
            }

            if !still_searching_root {
                // println!("is ss {} equal to spec {} ??", ss, spec);
                if ss == spec {
                    // println!("YES! breaking here with nat: {}", cur_nat);
                    break cur_nat;
                }

                // print!(" [increment ss] ");
                ss += 1;
            }

            // print!(" [increment aa] ");
            aa += 1;
            if aa == ALL_NATS.len() {
                // print!(" [aa ({}) == ALL_NATS.len() -> resetting 0] ", aa);
                aa = 0;
            }
        };
        
        GenericInterval { notes: [root, interval] }
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
        hstep_mod: 0,
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

// intervals start with a generic interval
// then you determine the specific number of half steps to go up and down
// examples:
// * major second = two half steps
//      C-D since it's a generic second on staff and two half steps on the keyboard
//      E-F#
// * major third = four half steps
//      C-E
//      E-G#
// * perfect fourth = five half steps
//      C-F
//      F-Bb
// * perfect fifth = seven half steps
//      C-G
//      B-F#
// * major sixth = nine half steps
//      C-A
//      Eb-C
// * major seventh = eleven half steps
//      C-B
//      D-C#
// * perfect eighth (or perfect octave) = twelve half steps
//      C-C
struct Interval {

}

// TODO: next part is tricky...
// - need to generate specific intervals
// - start with generic interval to get the letter we want
//   (because sometimes you notate as D## even when you actually play E...)
// - then each "name of interval" is a ruleset for moving a specific number of
//   half steps up or down (this is where the halfstep index idea comes in)
// - intervals such as: major second, perfect fourth, major sixth, etc...

// TODO: are other functions helpful? probably only need to implement these if
//       we need em...
// - from NOTE1, return NOTE2 which is one half step up?
// - return distance between two notes specified

// TODO: create test cases / asserts for every struct here
// - what's the rust convention for where you put all the asserts?
// - in another file for test cases?

pub fn test_generic_intervals() {
    let func_str = "note.rs / test_generic_intervals()";
    println!("[START {}]", func_str);

    // set up some naturals and intervals for the tests
    let a = Natural::A;
    let b = Natural::B;
    let c = Natural::C;
    let d = Natural::D;
    let e = Natural::E;
    let f = Natural::F;
    let g = Natural::G;
    let aa = GenericInterval { notes: [a, a] };
    let cb = GenericInterval { notes: [c, b] };
    let cc = GenericInterval { notes: [c, c] };
    let cd = GenericInterval { notes: [c, d] };
    let ce = GenericInterval { notes: [c, e] };
    let cf = GenericInterval { notes: [c, f] };
    let dc = GenericInterval { notes: [d, c] };
    let dd = GenericInterval { notes: [d, d] };
    let df = GenericInterval { notes: [d, f] };
    let ea = GenericInterval { notes: [e, a] };
    let ed = GenericInterval { notes: [e, d] };
    let ef = GenericInterval { notes: [e, f] };
    let eg = GenericInterval { notes: [e, g] };
    let fa = GenericInterval { notes: [f, a] };
    let gg = GenericInterval { notes: [g, g] };

    // helper vars
    let mut my_spec;
    let mut my_itvl;
    let mut asrt;
    let errstr = "intervals not equal? {} and {}";

    // these are firsts...
    my_spec = GenericInterval::FIRST;

    my_itvl = GenericInterval::new(Natural::C, my_spec);
    asrt = cc;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::D, my_spec);
    asrt = dd;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    //... but also they're octaves
    my_spec = GenericInterval::OCTAVE;

    my_itvl = GenericInterval::new(Natural::C, my_spec);
    asrt = cc;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::D, my_spec);
    asrt = dd;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::G, my_spec);
    asrt = gg;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::A, my_spec);
    asrt = aa;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    // these are seconds
    my_spec = GenericInterval::SECOND;

    my_itvl = GenericInterval::new(Natural::C, my_spec);
    asrt = cd;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::E, my_spec);
    asrt = ef;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    // these are thirds
    my_spec = GenericInterval::THIRD;

    my_itvl = GenericInterval::new(Natural::C, my_spec);
    asrt = ce;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::D, my_spec);
    asrt = df;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::E, my_spec);
    asrt = eg;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::F, my_spec);
    asrt = fa;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    // these are fourths
    my_spec = GenericInterval::FOURTH;

    my_itvl = GenericInterval::new(Natural::C, my_spec);
    asrt = cf; //correct: c / f = fourth
    //asrt = cd; //intentionally wrong test case
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::E, my_spec);
    asrt = ea;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    // these are sevenths
    my_spec = GenericInterval::SEVENTH;

    my_itvl = GenericInterval::new(Natural::C, my_spec);
    asrt = cb;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::D, my_spec);
    asrt = dc;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    my_itvl = GenericInterval::new(Natural::E, my_spec);
    asrt = ed;
    assert!(my_itvl == asrt, "{} != {}  ?!", my_itvl, asrt);

    println!("[DONE  {} ... asserts did not cause panics! all good!]", func_str);
}

pub fn print_some_generic_intervals() {
    let func_str = "note.rs / print_some_generic_intervals()";
    println!("[START {}]", func_str);

    // bad: should panic
    //let my_interval = GenericInterval::new(mynat, 8);
    // D, fifth = D A
    // ? E, fifth = E B
    // ? F, fifth = F C
    let my_root = Natural::G;
    let my_interval = GenericInterval::new(my_root, GenericInterval::OCTAVE);
    println!("\nmy interval...\n{:#?}", my_interval);
    println!("{}", my_interval);

    let my_2nd_root = Natural::G;
    let my_2nd_interval = GenericInterval::new(my_2nd_root, GenericInterval::FIRST);
    println!("\nmy interval...\n{:#?}", my_2nd_interval);
    println!("{}", my_2nd_interval);

    println!("[DONE  {}]", func_str);
}

pub fn debug_note() {
    let func_str = "note.rs / debug_note()";
    println!("[START {}]", func_str);

    test_generic_intervals();
    print_some_generic_intervals();

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

