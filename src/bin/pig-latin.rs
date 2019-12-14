use exercises::pig_latin::translate;

fn main() {
    let sentence = "first apple";
    let pig_latin = translate(sentence);
    println!("{} -> {}", sentence, pig_latin);

    let sentence = "  First\taPpLe!  ";
    let pig_latin = translate(sentence);
    println!("{} -> {}", sentence, pig_latin);

    let sentence = "When I say my first apple tree, I almost got my first heart attack!";
    let pig_latin = translate(sentence);
    println!("{} -> {}", sentence, pig_latin);
}
