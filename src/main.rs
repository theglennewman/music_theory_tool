
// include note.rs + interval.rs as modules here
// ... I *think* this means, take the .rs file and include the module in-line here?
mod note;
mod interval;

fn main() {
    println!("\nmusic_theory_tool!\n");
    println!("[START main.rs main()]");

    interval::debug_interval();

    println!("[DONE  main.rs main()]");
}

// rules for determining generic intervals
// then specific intervals
// and then specific intervals are used to build chords / triads / sevenths
// (maybe even scales...)

// so the intial plan is to...

// implement how to determine notes / moving up and down specific number of
// half steps
//   --> may skip this idea

// implement intervals - root + specific number of steps
//   --> done with generic intervals!

// implement triads using two intervals

// implement chords / sevenths using triads and intervals

// implement scales which are also intervals between notes

// implement ways to create these things based on "root input" + "rule input"
//   --> for example: generate_interval("A#", "perfect fifth")

// I also want to have a way to input two notes, tool will list out potential
// triads that the notes could appear in
// ... or scales: I put in four notes, tool displays scales that it appears in

// Also alternatives:
// If I put in a specific seventh chord (major seventh)
// Give me the alternative forms with that root
// - what's the dominant seventh?
// - what's the half diminished, or fully diminished?

// finally... can we set up reporting for each thing?
// - interval report
// - triad report
// - chord report
// - scale report
//
// potentially use yaml settings to "turn on" or "turn off" specific sections
// of reports?
// if a triad report has sections like "notes in the triad" and "alternative
// triads" then maybe a user can specify skipping the "alternative" section
// other ideas?
// - this interval is XY
// - this minor triad is ABC (made up of a minor third and a perfect fifth interval, here are the notes: AB#C)

// woohoo! lots of ideas
