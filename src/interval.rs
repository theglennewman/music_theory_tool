
//import our note module (note.rs)
// ... I *think* the 'crate' part of this means "use note from the crate we're in"
use crate::note;

pub fn debug_interval() {
    let func_str = "interval.rs / debug_interval.rs()";
    println!("[START {}]", func_str);

    println!("calling something from note...");
    note::debug_note();

    println!("[DONE  {}]", func_str);
}