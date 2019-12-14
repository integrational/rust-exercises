use State::{OutsideWord, WithinWord};

pub fn translate(s: &str) -> String {
    let mut lat = PigLatinizer::new();
    for c in s.chars() {
        lat.accept(c);
    }
    lat.emit()
}

enum State {
    OutsideWord,
    WithinWord(char), // Within a word with the given first char
}

struct PigLatinizer {
    result: String,
    state: State,
}

impl PigLatinizer {
    fn new() -> PigLatinizer {
        PigLatinizer {
            result: String::new(),
            state: OutsideWord,
        }
    }

    fn accept(&mut self, c: char) {
        // FIXME fishy combination of mutable and immutable borrows of self
        match (&self.state, is_word_char(c)) {
            (OutsideWord, false) => self.push_current_char(c),
            (OutsideWord, true) => {
                self.push_first_char(c);
                self.state = WithinWord(c);
            }
            (WithinWord(_), true) => self.push_current_char(c),
            (WithinWord(firstc), false) => {
                self.push_word_suffix(*firstc);
                self.push_current_char(c);
                self.state = OutsideWord;
            }
        }
    }

    fn emit(&mut self) -> String {
        // also ends the current word
        if let WithinWord(firstc) = self.state {
            self.push_word_suffix(firstc);
            self.state = OutsideWord;
        }
        self.result.clone() // clone required because of self borrow issues
    }

    fn push_current_char(&mut self, currentc: char) {
        self.result.push(currentc);
    }

    fn push_first_char(&mut self, firstc: char) {
        if is_vowel(firstc) {
            self.result.push(firstc);
        }
    }

    fn push_word_suffix(&mut self, firstc: char) {
        self.result.push('-');
        self.result
            .push(if is_vowel(firstc) { 'h' } else { firstc });
        self.result.push_str("ay");
    }
}

fn is_word_char(c: char) -> bool {
    c.is_alphanumeric()
}

fn is_vowel(c: char) -> bool {
    "aeiouAEIOUäöüÄÖÜ".contains(c) // TODO generalize
}
