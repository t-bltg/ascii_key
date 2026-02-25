use clap::{Parser, Subcommand};
use stitcher::NameStyle;

use crate::{chords, stitcher};
use crate::chord::Chord;
use std::fs;

// a CLI to show you how to play a piano chord
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

impl Args {
    pub fn run(self) {
        match self.command {
            Command::Get(get_args) => get_args.run(),
            Command::List(list_args) => list_args.run(),
            Command::All(all_args) => all_args.run(),
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Get(GetArgs),
    List(ListArgs),
    All(AllArgs),
}

#[derive(Parser, Debug)]
pub struct GetArgs {
    // name of the chord
    #[clap()]
    name: String,
}

impl GetArgs {
    fn run(self) {
        let name_uppercase = self.name.to_ascii_uppercase();
        match chords::ALL_CHORDS.iter().find(|&chord| {
            chord
                .short_names
                .iter()
                .any(|&name| name.to_ascii_uppercase() == name_uppercase)
        }) {
            None => println!("Unknown chord '{}'", self.name),
            Some(chord) => println!(
                "This is how you play '{}' chord: \n{}",
                self.name,
                chord.keyboard()
            ),
        }
    }
}

#[derive(Parser, Debug)]
pub struct AllArgs {
    #[clap(short, long)]
    save: bool,
}
impl AllArgs {
    fn run(self) {
        let mut buffer: String = String::default();
        buffer += "# All Supported Chords";

        for chord in chords::ALL_CHORDS {
            buffer += &format!("\n## {}\n", chord.both_names()).to_string();
            buffer += &format!("```\n{}\n```", chord.keyboard()).to_string();
        }

        if self.save {
            fs::write("./ALL_CHORDS.md", &buffer).expect("Unable to write file");
        }
        println!("{}\n", buffer);
    }
}

#[derive(Parser, Debug)]
pub struct ListArgs {
    // names of the chords
    #[clap()]
    names: Vec<String>,

    // in the output, which name to include
    #[clap(value_enum, long="style", default_value_t=NameStyle::ShortNames)]
    name_style: NameStyle,

    // in the output, how many spaces for padding between chords
    #[clap(short, long, default_value_t = 4)]
    padding: u8,
}

impl ListArgs {
    fn run(self) {
        // we would like to keep the order that 'names' are passed in
        let chords: Vec<Chord<'static>> = self
            .names
            .iter()
            .map(|name| -> Vec<Chord<'static>> {
                // find the ones that matches the chord name
                match chords::ALL_CHORDS_BY_SHORT_NAMES.get(&name.to_ascii_lowercase()) {
                    Some::<&Vec<&'static Chord<'static>>>(matched_chords) => matched_chords
                        .into_iter()
                        .map(|chord: &&'static Chord<'static>| -> Chord<'static> { (*chord).clone() })
                        .collect(),
                    None => {
                        println!("Unknown chord '{}'", name);
                        vec![]
                    }
                }
            })
            .flatten()
            .collect();
        println!("{}", stitcher::row(chords, self.name_style));
    }
}
