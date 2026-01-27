fn main() {
    println!("{}",piggy("first"));
    println!("{}",piggy("apple"));
}

fn piggy(word: &str) -> String {
    let vowels: [char;12] = ['A','a','E','e','I','i','O','o','U','u','Y','y'];
    let hay: String = String::from("-hay");
    for letter in word.chars() {
        if vowels.contains(&letter) {
            return word.to_owned() + &hay;
        }
        else {
            let new_word = &word[1..];
            return format!("{new_word}-{letter}ay");
        }
    }
    return hay;
}
