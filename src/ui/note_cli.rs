use std::io::{self, Write};

use crate::music::Note;
use super::choice_ui::{Choice, self};

pub fn edit_note_ui(note: &mut Note) {
    let choices = vec![
        Choice::new("Change Starting Beat".to_string(), Box::from(change_note_start_ui)),
        Choice::new("Change Note Duration".to_string(), Box::from(change_note_duration_ui)),
        Choice::new("Change Note Pitch".to_string(), Box::from(change_note_pitch_ui)),
        Choice::new("Change Note Volume".to_string(), Box::from(change_note_volume_ui))
    ];
    loop {
        println!("Note editor\nNote: {note}");
        if let Some(_) = choice_ui::ui_offer_choices(&choices, note) {}
        else {
            break
        }
    }
    println!("You have left part editor!");
}

pub fn change_note_start_ui(note: &mut Note) {
    print!("Note starting beat: ");
    io::stdout().flush().expect("Failed to flush stdout! Exiting!");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read user input!");
    match buf.parse::<f32>() {
        Ok(beat) => {
            let old_beat = note.beat;
            note.beat = beat;
            println!("Changed starting beat from {old_beat} to {}!", note.beat);
        }
        Err(_) => {
            return;
        }
    }
}

pub fn change_note_duration_ui(note: &mut Note) {
    todo!();
}

pub fn change_note_pitch_ui(note: &mut Note) {
    todo!();
}

pub fn change_note_volume_ui(note: &mut Note) {
    todo!();
}