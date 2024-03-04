use std::fs::File;
use::std::io::{self, Write, Read, BufReader};
use std::rc::Rc;
use crate::music::{ Serializable, Song, Part, Note };
use crate::ui::choice_ui::{ui_offer_choices, Choice};
use crate::wav::WavOptions;
use rfd::FileDialog;

///
pub struct SongEditor {
    loaded_songs: Vec<Song>
}

impl SongEditor {
    pub fn new() -> Self {
        SongEditor { loaded_songs: vec![Song::default()] }
    }

    pub fn ui(&mut self) {
        println!("Hello! Welcome to Song Maker!");
        'ui: loop {
            self.ui_show_songs();
            println!("Select one of the options listed below:");
            println!("\t1. Load Song");
            println!("\t2. Save Song");
            println!("\t3. Create Song");
            println!("\t4. Edit Song");
            println!("\t5. Export Song to wav file");
            println!("\t6. Exit Song Maker");
            let mut option = String::new();
            loop {
                io::stdin().read_line(&mut option).expect("Could not read user input");
                match option.trim() {
                    "1" => {
                        self.load_ui();
                        break;
                    }
                    "2" => {
                        self.save_ui();
                        break;
                    }
                    "3" => {
                        self.create_ui();
                        break;
                    }
                    "4" => {
                        self.edit_ui();
                        break;
                    }
                    "5" => {
                        self.compile_ui();
                        break;
                    }
                    "6" | "q" | "Q" => {
                        break 'ui;
                    }
                    _ => {
                        println!("option not recognized! press q to quit or select another option!");
                    }
                }
            }
        }
        println!("Goodbye from Song Maker!")
    }

    fn ui_show_songs(&mut self) {
        println!("Available Songs:");
        for (index, song) in self.loaded_songs.iter().enumerate() {
            println!("\t{}. {}", index + 1, song.name);
        }
    }

    fn create_ui(&mut self) {
        print!("Song name: ");
        io::stdout().flush().expect("Failed to flush stdout! Exiting!");
        let mut song_name = String::new();
        io::stdin().read_line(&mut song_name).expect("Failed to read song name!");
        let song_name = song_name.trim().to_string();
        if let Some((index, _song)) = self.loaded_songs.iter().enumerate().find(|(_index, song)| song.name.as_str() == song_name) {
            println!("A song already exists with this name would you like to overwrite it? (y/n)");
            let mut overwrite = String::new();
            io::stdin().read_line(&mut overwrite).expect("Failed to read user input!");
            match overwrite.trim() {
                "y" | "yes" | "Y" | "YES" => {
                    self.loaded_songs.remove(index);
                }
                _ => {
                    println!("Create song aborted!");
                    return;
                }
            }
        }
        self.loaded_songs.push(Song::new(song_name, 120));
        println!("Created song!")
    }

    fn edit_ui(&mut self) {
        loop {
            self.ui_show_songs();
            println!("Which song would you like to edit?");
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).expect("Failed to read user input!");
            let edit_index = buf.trim();
            let song_index = edit_index.parse::<usize>().expect("Failed to parse user input as number!") - 1;
            if let Some(song) = self.loaded_songs.get_mut(song_index) {
                // Editing Song ui
                println!("Song name: {}", song.name);
                println!("Song parts:");
                let parts = &song.parts;
                for (index, part) in parts.iter().enumerate() {
                    println!("\t{}. {}", index + 1, part.name)
                }
                println!("Which part would you like to edit?");
                buf.clear();
                io::stdin().read_line(&mut buf).expect("Failed to read user input!");
                let edit_index = buf.trim();
                let part_index = edit_index.parse::<usize>().expect("Could not parse user input as number!") - 1;
                if let Some(part) = song.parts.get_mut(part_index) {
                    ui_edit_part(part);
                    println!("Done editing Song!");
                    return;
                }
                else {
                    println!("{part_index} is not a valid song index!")
                }
            } else {
                println!("{song_index} is not a valid song index!")
            }
        } 
    }

    fn load_ui(&mut self) {
        // Get file from the user
        println!("Select a .song file to load");
        // Show file dialog
        let file = FileDialog::new()
            .add_filter("songs", &["song"])
            .set_directory("/")
            .pick_file();
        match file {
            Some(file_path) => {
                if let Ok(f) = File::open(&file_path) {
                    let mut buf_reader = BufReader::new(f);
                    let mut serialized_data: Vec<u8> = Vec::new();
                    if let Err(_err) = buf_reader.read_to_end(&mut serialized_data) {
                        println!("could not read from file!");
                        return;
                    }

                    match Song::deserialize(&serialized_data) {
                        Ok(song) => {
                            self.loaded_songs.push(song);
                            println!("Loaded Song!");
                            return;
                        }
                        Err(err) => {
                            println!("{err}");
                        }
                    }
                }
            }
            None => {
                println!("No files selected or file failed to open!");
                return;
            }
        }
    }

    fn save_ui(&self) {
        loop {
            println!("Which song would you like to save?");
            let mut save_index = String::new();
            io::stdin().read_line(&mut save_index).expect("Failed to read user input!");
            let save_index = save_index.trim();
            if let Ok(index) = save_index.parse::<usize>() {
                if let Some(song) = self.loaded_songs.get(index) {
                    println!("saving song...");
                    song.write_to_song_file(song.name.clone());
                    println!("saving complete!");
                    break
                }
            }
            println!("{save_index} is not a valid song index!")
        } 
    }

    fn compile_ui(&self) {
        loop {
            println!("Which song would you like to compile?");
            let mut compile_index: String = String::new();
            io::stdin().read_line(&mut compile_index).expect("Failed to read user input!");
            let compile_index = compile_index.trim();
            if let Ok(index) = compile_index.parse::<usize>() {
                if let Some(song) = self.loaded_songs.get(index) {
                    println!("Compiling song...");
                    // TODO get wavoptions from user optionally
                    song.write_to_wav_file(song.name.clone(), &WavOptions::default());
                    println!("Compilation complete!");
                    break
                }
            }
            println!("{compile_index} is not a valid song index!")
        } 
    }
}

fn ui_select_song() {

}

fn ui_select_part() {

}

fn ui_select_note() {

}

fn ui_edit_part(part: &mut Part) {
    println!("Editing part: {}", part.name);
    println!("Notes:");
    for note in part.notes.iter() {
        println!("{}", note);
    }
    println!("What would you like to do?");
    let choices = vec![
        Choice::new("add note".to_string(), Box::from(ui_add_note)),
        Choice::new("delete note".to_string(), Box::from(ui_delete_note))
    ];
    ui_offer_choices(choices, part);
}

fn ui_add_note(part: &mut Part) {
    let mut buf = String::new();
    // Get beat to play on from user
    print!("beat to play on: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    let beat = buf.trim().parse::<f32>().expect("failed to parse user input as float!");
    // Get duration of note in beats from user
    print!("duration in beats: ");
    io::stdout().flush().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let duration = buf.trim().parse::<f32>().expect("failed to parse user input as float!");
    // Get frequency of note from user
    print!("frequency: ");
    io::stdout().flush().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let frequency = buf.trim().parse::<f32>().expect("failed to parse user input as float!");
    // Get volume of note from user
    print!("volume: ");
    io::stdout().flush().unwrap();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let volume = buf.trim().parse::<f32>().expect("failed to parse user input as float!");
    // Create note from user input
    let note = Note::new(beat, duration, frequency, volume).expect("Failed to make note!");
    // Add note to part
    part.add_note(note).expect("Could not add note to part!");
}

fn ui_delete_note(part: &mut Part) {
    println!("{part}");
    println!("Which note would you like to delete?");
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read user input!");
    let note_index = buf.trim().parse::<usize>().unwrap() - 1;
    part.notes.remove(note_index);
    println!("Successfully deleted note!")
}