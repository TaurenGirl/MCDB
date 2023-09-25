use clearscreen;
use colored::Colorize;
use cfonts::{ say, Options, Fonts, Colors };
use cfonts::font::{load_all_fonts, get};
use crate::utilities::{self, Collection};

pub enum Screen {
    Main,
    Edit,
    Search
}

pub fn start() {
    let mut options = Options::default();
    options.font = Fonts::Font3d;
    
    say(Options {
        text: String::from("Marvel Champions Database"),
        font: options.font,
        colors: vec![Colors::YellowBright, Colors::Cyan],
        ..Options::default()
    });
    let mut collection = utilities::populate("<Card>", "</Card>", "./src/info.xml".to_string());
    start_menu(&collection);
}

pub fn start_menu(collection: &Collection) {
    loop {
        
        println!("{}", "Menu".underline().bold().bright_yellow());
        println!("{} {}", "1.".bright_yellow(), "View Collection".cyan());
        println!("{} {}", "2.".bright_yellow(), "Edit Collection".cyan());
        println!("{} {}", "3.".bright_yellow(), "Search Collection".cyan());
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line.");
        let correct_line = line.trim();
        match correct_line {
            "1" => view_collection(collection),
            "2" => edit_menu(),
            "3" => search_menu(),
            _ => println!("{}", "Invalid Input".to_string())
        }
    }
}

pub fn edit_menu() {
    loop {
        clearscreen::clear().expect("Failed to clear screen.");
        println!("{}", "Edit".underline().bold().bright_yellow());
        println!("{} {}", "1.".bright_yellow(), "Add Card".cyan());
        println!("{} {}", "2.".bright_yellow(), "Remove Card".cyan());
        println!("{} {}", "b.".bright_yellow(), "Back".cyan());
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line.");
        let correct_line = line.trim();
        match correct_line {
            "b" => { 
                clearscreen::clear().expect("Failed to clear screen.");
                break;
            },
            _ => println!("{}", "Invalid Input".to_string())
        }
    }
}

pub fn search_menu() {
    loop {
        clearscreen::clear().expect("Failed to clear screen.");
        println!("{}", "Search".underline().bold().bright_yellow());
        println!("{} {}", "1.".bright_yellow(), "Search by Name".cyan());
        println!("{} {}", "2.".bright_yellow(), "Search by Aspect".cyan());
        println!("{} {}", "3.".bright_yellow(), "Search by Hero".cyan());
        println!("{} {}", "b.".bright_yellow(), "Back".cyan());
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("Failed to read line.");
        let correct_line = line.trim();
        match correct_line {
            "b" => {
                clearscreen::clear().expect("Failed to clear screen.");
                break;
            },
            _ => println!("{}", "Invalid Input".to_string())
        }
    }
}

pub fn view_collection(collection: &Collection) {
    clearscreen::clear().expect("Failed to clear screen.");
    for (i, el) in collection.contents.iter().enumerate() {
        println!("{}{} {:<25} {} {:<15} {} {:<15}", (i + 1).to_string().bright_yellow(), ".".bright_yellow(), collection.contents[i].name.cyan(), "|".bright_yellow(), collection.contents[i].aspect.cyan(), "|".bright_yellow(), collection.contents[i].owner.cyan());
    }
}