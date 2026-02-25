use chord::Chord;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static ALL_CHORDS: &'static [Chord] = &[
    // NOTE: major is omitted such that A ≡ A major by default
    // keyboard1
    Chord::new(&["C"], "ceg", "", &["C"]),
    Chord::new(&["C7"], "cegA", "", &["C 7ᵗʰ"]),
    Chord::new(&["Cmaj7"], "cegb", "", &["C raised 7ᵗʰ"]),
    Chord::new(&["Csus"], "cfg", "", &["C suspended"]),
    Chord::new(&["C6"], "cega", "", &["C 6ᵗʰ"]),
    Chord::new(&["Cadd2"], "cdeg", "", &["C added 2ⁿᵈ"]),
    Chord::new(&["Cm"], "cDg", "", &["C minor"]),
    Chord::new(&["Cm7"], "cDgA", "", &["C minor 7ᵗʰ"]),
    // Chord::new(&["Cm7b5"], "", "", &[""]),
    Chord::new(&["Cdim"], "cDF", "", &["C diminished"]),
    Chord::new(&["Cdim7"], "cDFa", "", &["C diminished 7ᵗʰ"]),
    Chord::new(&["C+"], "ceG", "", &["C augmented"]),
    Chord::new(&["C#"], "CfG", "", &["C♯ [D♭]"]),
    Chord::new(&["C#7"], "CfGb", "", &["C♯ 7ᵗʰ [D♭ 7ᵗʰ]"]),
    Chord::new(&["D"], "dFa", "", &["D"]),
    Chord::new(&["D7"], "dFa0", "", &["D 7ᵗʰ"]),
    Chord::new(&["Dmaj7"], "dFa1", "", &["D raised 7ᵗʰ"]),
    Chord::new(&["Dsus"], "dga", "", &["D suspended"]),
    Chord::new(&["D6"], "dFga", "", &["D 6ᵗʰ"]),
    Chord::new(&["Dadd2"], "deFa", "", &["D added 2ⁿᵈ"]),
    Chord::new(&["Dm"], "dfa", "", &["D minor"]),
    Chord::new(&["Dm7"], "dfa0", "", &["D minor 7ᵗʰ"]),
    Chord::new(&["E"], "eGb", "", &["E"]),
    Chord::new(&["E7"], "eGb2", "", &["E 7ᵗʰ"]),
    Chord::new(&["Emaj7"], "eGb3", "", &["E raised 7ᵗʰ"]),
    Chord::new(&["Esus"], "eab", "", &["E suspended"]),
    Chord::new(&["E6"], "eGb1", "", &["E 6ᵗʰ"]),
    Chord::new(&["Eadd2"], "eFGb", "", &["E added 2ⁿᵈ"]),
    Chord::new(&["Em"], "egb", "", &["E minor"]),
    Chord::new(&["F"], "fa0", "", &["F"]),
    Chord::new(&["Fm"], "fG0", "", &["F minor"]),
    // keyboard2
    Chord::new(&["F#"], "", "FAC", &["F♯"]),
    Chord::new(&["G"], "", "gbd", &["G"]),
    Chord::new(&["G7"], "", "gbd0", &["G 7ᵗʰ"]),
    Chord::new(&["Gm"], "", "gAd", &["G minor"]),
    Chord::new(&["A"], "", "aCe", &["A"]),
    Chord::new(&["Am"], "", "ace", &["A minor"]),
    Chord::new(&["B"], "", "bD1", &["B"]),
    Chord::new(&["Bm"], "", "bd1", &["B minor"]),
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
    // NOTE: useful idiom - importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_single_pattern() {
        for chord in ALL_CHORDS {
            assert!(chord.pattern1.is_empty() || chord.pattern2.is_empty())
        }
    }
}
