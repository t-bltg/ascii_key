use itertools::join;

// cdefgab
pub const KEYBOARD1: &str = "\
┌─┬─┬┬─┬─┬─┬─┬┬─┬┬─┬─┐
│ └┬┘└┬┘ │ └┬┘└┬┘└┬┘ │
└──┴──┴──┴──┴──┴──┴──┘";

// fgabcde
pub const KEYBOARD2: &str = "\
┌─┬─┬┬─┬┬─┬─┬─┬─┬┬─┬─┐
│ └┬┘└┬┘└┬┘ │ └┬┘└┬┘ │
└──┴──┴──┴──┴──┴──┴──┘";

#[derive(Debug, Clone)]
pub struct Chord<'a> {
    pub short_names: &'a [&'a str],
    // cdefgab and CDEFGAB ≡ c♯d♯e♯f♯g♯a♯b♯
    pub pattern1: &'a str,
    pub pattern2: &'a str,
    pub names: &'a [&'a str],
}

impl<'a> Chord<'a> {
    pub const fn new(
        short_names: &'a [&'a str],
        pattern1: &'a str,
        pattern2: &'a str,
        names: &'a [&'a str],
    ) -> Self {
        Self {
            short_names: short_names,
            pattern1: pattern1,
            pattern2: pattern2,
            names: names,
        }
    }

    pub fn both_names(&self) -> String {
        format!(
            "{} ({})",
            join(self.names, " | "),
            join(self.short_names, " | ")
        )
    }

    pub fn keyboard1(&self) -> String {
        if self.pattern1.trim().is_empty() {
            return String::from("");
        }

        let mut board: Vec<char> = KEYBOARD1.chars().collect();
        let width: usize = board.iter().position(|c| *c == '\n').expect("newline") + 1;

        for ch in self.pattern1.chars() {
            let idx: usize = match ch {
                'c' => 2 * width + 1,
                'd' => 2 * width + 4,
                'e' => 2 * width + 7,
                'f' => 2 * width + 10,
                'g' => 2 * width + 13,
                'a' => 2 * width + 16,
                'b' => 2 * width + 19,
                'C' => 3,
                'D' => 6,
                'F' => 12,
                'G' => 15,
                'A' => 18,
                _ => 0,
            };
            board[idx] = '●';
        }

        board.iter().collect()
    }

    pub fn keyboard2(&self) -> String {
        if self.pattern2.trim().is_empty() {
            return String::from("");
        }

        let mut board: Vec<char> = KEYBOARD2.chars().collect();
        let width: usize = board.iter().position(|c| *c == '\n').expect("newline") + 1;

        for ch in self.pattern2.chars() {
            let idx: usize = match ch {
                'f' => 2 * width + 1,
                'g' => 2 * width + 4,
                'a' => 2 * width + 7,
                'b' => 2 * width + 10,
                'c' => 2 * width + 13,
                'd' => 2 * width + 16,
                'e' => 2 * width + 19,
                'F' => 3,
                'G' => 6,
                'A' => 9,
                'C' => 15,
                'D' => 18,
                _ => 0,  // panic!()
            };
            board[idx] = '●';
        }

        board.iter().collect()
    }

    pub fn keyboard(&self) -> String {
        let kb1 = self.keyboard1();
        if kb1.is_empty() { self.keyboard2() } else { kb1 }
    }
}
