use chord::Chord;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ALL_CHORDS: &'static [Chord] = &[
    // NOTE: major is omitted such that A ≡ A major by default
    Chord::new(&["C"], "ceg", "", &["C"]),
    Chord::new(&["C7"], "cegA", "", &["C 7ᵗʰ"]),
    Chord::new(&["F#"], "", "FAC", &["F♯"]),
    Chord::new(&["G"], "", "gbd", &["G"]),
    // Chord {
    //     short_names: &["Esus", "Esus4"],
    //     pattern1: "022200",
    //     names: &["E suspended", "E suspended 4ᵗʰ"],
    // },
];

pub static ALL_CHORDS_BY_SHORT_NAMES: Lazy<HashMap<String, Vec<&'static Chord<'static>>>> =
    Lazy::new(|| {
        let mut map = HashMap::<_, Vec<_>>::new();

        for chord in ALL_CHORDS {
            for sn in chord.short_names {
                map.entry(sn.to_ascii_lowercase())
                    .or_default()
                    .push(chord);
            }
        }
        map
    });

#[cfg(test)]
mod tests {
    // NOTE: Useful idiom - importing names from outer (for mod tests) scope.
    use super::*;
/*
    #[test]
    fn test_chord_pattern_length() {
        for chord in ALL_CHORDS {
            assert_eq!(
                chord.pattern.chars().count(),
                6,
                "Guitar has 6 strings. This is a invalid pattern {:?}",
                chord.pattern
            )
        }
    }

    #[test]
    fn test_digit_or_x() {
        for chord in ALL_CHORDS {
            for char in chord.pattern1.chars() {
                match char.to_digit(10) {
                    None => assert_eq!(char, 'x', "Only digits or 'x' is allowed"),
                    Some(digit) => assert!(digit < 6),
                }
            }
        }
    }
*/
}
