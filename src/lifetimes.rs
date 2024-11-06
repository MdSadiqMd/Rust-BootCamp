// Function that finds the longest word in a sentence
fn longest_word<'a>(sentence: &'a str) -> &'a str {
    // Split the sentence into words, and return the longest one
    let mut longest = "";
    for word in sentence.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest
}

// Struct that holds a reference to a string slice
// We use a lifetime parameter to make sure that the 'text' reference is valid
struct Sentence<'a> {
    text: &'a str,
}

impl<'a> Sentence<'a> {
    // Method that returns the longest word in the sentence
    // The returned reference will have the same lifetime as the 'Sentence' struct
    fn get_longest_word(&self) -> &'a str {
        longest_word(self.text)
    }
}

pub fn main() {
    // Original sentence string
    let my_sentence = String::from("Learning Rust is really rewarding and challenging");

    // Create a 'Sentence' instance holding a reference to 'my_sentence'
    let sentence = Sentence { text: &my_sentence };

    // Find the longest word in the sentence using the method in the struct
    let word = sentence.get_longest_word();
    println!("The longest word is: {}", word);

    // Note: If 'my_sentence' goes out of scope, 'sentence' and 'word' would be invalid
    // This is why Rust enforces lifetimes, to avoid such scenarios.
}
