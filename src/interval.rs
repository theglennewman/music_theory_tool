
use std::fmt;

//import our note module (note.rs)
// ... I *think* the 'crate' part of this means "use note from the crate we're in"
use crate::note;

use note::Natural;

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

// an interval is a SPECIFIC interval... 
// which is determined by:
//   1. find the second note by the generic interval (position on staff)
//   2. modify the second note with an accidental... by moving up or down
//    to reach the specific number of half steps for the specified interval
//
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

pub fn debug_interval() {
    let func_str = "interval.rs / debug_interval.rs()";
    println!("[START {}]", func_str);

    test_generic_intervals();
    print_some_generic_intervals();

    println!("calling something from note...");
    note::debug_note();

    println!("[DONE  {}]", func_str);
}