use std::io;

pub fn transition() {
    loop {
        println!("请输入英文单词");

        let mut word = String::new();

        io::stdin().read_line(&mut word).expect("Failed to read line");

        let word: String = word.trim().to_string();

        if word.len() == 0 {
            continue;
        }

        to_pig_latin(&word);
    }
}

fn to_pig_latin(word: &String) -> () {
    let mut result = String::with_capacity(word.len() + 4);
    let mut chars = word.chars();
    let first_char = chars.next().unwrap();

    if is_vowel(first_char) {
        result.push_str(format!("{}-hay", word).as_str());
    } else {
        result.push_str(format!("{}-{}ay", chars.as_str(), first_char).as_str());
    }

    println!("结果是：{}", result);
}

fn is_vowel(char: char) -> bool {
    String::from("aeiouAEIOU").contains(char)
}