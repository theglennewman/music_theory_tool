
// example using a module
// this code calls the "note" module, which is in "note.rs" file
// further down we call a function from that file: "note::debug_note()"
mod note;

fn main() {
    println!("music_theory_tool\n");
    println!("[main.rs main()]");
    note::debug_note();
}

// musical intervals have rules
// those interval definitions 
// are used to build chords / triads / sevenths

// implement how to determine notes / moving up and down specific number of
// half steps

// implement intervals - root + specific number of steps

// implement triads using two intervals

// implement chords / sevenths using triads and intervals

// implement scales which build further off these ideas

// implement ways to generate these things based on "root input" + "rule input"
// also to identify / display the "rule" / "name" / "potential options" based
// on "notes input"
// for example:
//   - input a note and perfect fifth -> display the two notes
//   - input two notes -> display the potential triads it could appear in

// implement reporting for each component - a nice way to display each thing
// and be able to break down "this interval is XYZ" and "this minor triad is
// made up of a minor third and a perfect fifth interval, here are the notes
// in this triad"

