use std::string::String;
use std::vec::Vec;

fn is_palindrome(input_string: &String) -> bool {
    let mut left = 0;
    let mut right = input_string.len() - 1;
    while left < right {
        if input_string.as_bytes()[left] != input_string.as_bytes()[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

pub(crate) fn main() {
    let input_strings: Vec<String> = vec![
        String::from("RACEACAR"),
        String::from("A"),
        String::from("ABCDEFGFEDCBA"),
        String::from("ABC"),
        String::from("ABCBA"),
        String::from("ABBA"),
        String::from("RACEACAR"),
    ];

    for (i, input_string) in input_strings.iter().enumerate() {
        println!("Test Case # {}", i + 1);
        println!("{}", "-".repeat(100));
        println!(
            "The input string is '{}' and the length of the string is {}.",
            input_string,
            input_string.len()
        );
        let result = is_palindrome(input_string);
        println!(
            "\nIs it a palindrome?..... {}",
            if result { "True" } else { "False" }
        );
        println!("{}", "-".repeat(100));
    }
}
