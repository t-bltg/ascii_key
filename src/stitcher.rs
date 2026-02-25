use crate::chord::Chord;
use clap::ValueEnum;
use itertools::{join, Itertools};

#[derive(Debug, ValueEnum, Clone)]
pub enum NameStyle {
    ShortNames,
    FullNames,
    BothNames,
}

pub fn row<'a>(chords: Vec<Chord<'a>>, name_style: NameStyle) -> String {
    let names: Vec<String> = chords
        .iter()
        .map(|chord| match name_style {
            NameStyle::ShortNames => join(chord.short_names.iter(), "|"),
            NameStyle::FullNames => join(chord.names.iter(), "|"),
            NameStyle::BothNames => chord.both_names(),
        })
        .collect();
    let keyboard: Vec<String> = chords
        .iter()
        .map(|chord| chord.keyboard())
        .collect();

    let lines: Vec<String> = names.into_iter().interleave(keyboard.into_iter()).collect();
    lines.join("\n")
}
