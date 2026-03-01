fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word = String::new();
    let mut max_count: usize = 0;

    let mut i: usize = 0;
    while i < words.len() {
        let current = words[i];

        let mut count: usize = 0;
        let mut j: usize = 0;
        while j < words.len() {
            if words[j] == current {
                count += 1;
            }
            j += 1;
        }

        if count > max_count {
            max_count = count;
            max_word = current.to_string();
        }

        i += 1;
    }

    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}