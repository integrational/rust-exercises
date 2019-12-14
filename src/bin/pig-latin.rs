use State::{OutsideWord, WithinWord};

fn main() {
    let sentence = "first apple";
    let pig_latin = PigLatinizer::translate(sentence);
    println!("{} -> {}", sentence, pig_latin);

    let sentence = "  First\taPpLe!  ";
    let pig_latin = PigLatinizer::translate(sentence);
    println!("{} -> {}", sentence, pig_latin);

    let sentence = "When I say my first apple tree, I almost got my first heart attack!";
    let pig_latin = PigLatinizer::translate(sentence);
    println!("{} -> {}", sentence, pig_latin);
}

struct PigLatinizer {
    result: String,
    state: State,
}

enum State {
    OutsideWord,
    WithinWord(char), // Within a word with the given first char
}

impl PigLatinizer {
    fn translate(s: &str) -> String {
        let mut lat = PigLatinizer::new();
        for c in s.chars() {
            lat.accept(c);
        }
        lat.emit()
    }

    fn new() -> PigLatinizer {
        PigLatinizer {
            result: String::new(),
            state: OutsideWord,
        }
    }

    fn is_word_char(c: char) -> bool {
        c.is_alphanumeric()
    }

    fn is_vowel(c: char) -> bool {
        "aeiouAEIOUäöüÄÖÜ".contains(c) // TODO generalize
    }

    fn accept(&mut self, c: char) {
        // FIXME fishy combination of mutable and immutable borrows of self
        match (&self.state, PigLatinizer::is_word_char(c)) {
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
        self.result.clone()
    }

    fn push_current_char(&mut self, currentc: char) {
        self.result.push(currentc);
    }

    fn push_first_char(&mut self, firstc: char) {
        if PigLatinizer::is_vowel(firstc) {
            self.result.push(firstc);
        }
    }

    fn push_word_suffix(&mut self, firstc: char) {
        self.result.push('-');
        if PigLatinizer::is_vowel(firstc) {
            self.result.push('h');
        } else {
            self.result.push(firstc);
        }
        self.result.push_str("ay");
    }
}
