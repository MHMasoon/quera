use std::io::stdin;

fn main() {
    // get words count and words from user
    let mut words_count = String::new();
    stdin().read_line(&mut words_count).unwrap();

    let mut words = String::new();
    stdin().read_line(&mut words).unwrap();
    let words = words.trim();

    // reverse words and store them in reveresed_words
    let mut reversed_words = String::new();
    for word in words.split(' ').rev() {
        reversed_words.push_str(word);
        reversed_words.push(' ');
    }
    
    // delete one extra space at the end
    reversed_words.pop();

    // print final resutl
    println!("{}", reversed_words);
}
