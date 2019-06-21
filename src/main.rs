// Kostya Yatsuk - 2019
/*
This code will accept a file which contains a numerical code and it will try to decode it, outputing possible results into an output text file.
Stretch: feed the output file into a grammar check to determine which of the possible outputs is the more likely.
The decoder will accept a .txt file with code-numbers being represented by numbers separated by spaces, and individual words being separated by "/r/n"
Next, a hash-table will be built up using the coded letters, and then applied to the other coded words. Then the decoder will loop through all the coded words,
trying to fill in their code-numbers. When a word is created this way, the decoder checks if this word exists in the word bank. If not, it rolls back changes
made to the HashMap and tries to match the next candidate word to the code
*/

use std::env;
use std::fs;
use std::fs::File;
use std::collections::HashMap;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    // Get the code from the file passed in
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("The file was not read correctly");


    // { Convert contents of file into a Vec of Vec }
    let split = contents.split("\r\n"); // Since i'm doing this for myself, i'm assuming a windows-encoded txt file

    let mut contents_vec: Vec<Vec<i32>> = Vec::new();

    for s in split {
        let temp_itr = s.split(" ");
        let mut line: Vec<i32> = Vec::new();

        for val in temp_itr {
            let int_val: i32 = val.parse::<i32>().unwrap();
            line.push(int_val);
        }
        contents_vec.push(line);
    }

    // In this file, each word will be separated by a newline
    let words_filename = "words.txt";          // CHANGE THIS TO "words.txt" WHEN DONE TESTING
    let words_string = fs::read_to_string(words_filename)
        .expect("The file was not read correctly");

    let split_words = words_string.split("\r\n");

    // Load all the words from the words_filename file into all_words
    let mut all_words: Vec<String> = Vec::new();

    for s in split_words {
        all_words.push(s.to_string());
    }

    // Create an empty HashMap and file
    let mut encode_hash: HashMap<i32, char> = HashMap::new();
    let mut file = File::create("out.txt")?;

    // Start decoding the string
    decode_remainder(&contents_vec, &mut encode_hash, &all_words, &mut file, &contents_vec);

    Ok(())
}

/* Attempts to recursively decode the remainder of the encoded message and prints successful results to |file|
   Accepts: {
       remain_code: &[Vec<i32>]: A vector of vectors of i32 representing the remaining part of the encoded message
       encode_hash: &mut HashMap<i32, char>: A mutable HashMap representing the letter associated with each number
       all_words: &Vec<String>: A vector of all the words in the dictionary
       file: &mut File: The file where the output will be stored
       pure_code: &[Vec<i32>]: A vector of vectors of i32 representing the original encoded message
   } */
fn decode_remainder(remain_code: &[Vec<i32>], encode_hash: &mut HashMap<i32, char>, all_words: &Vec<String>, file: &mut File, pure_code: &[Vec<i32>]) {

    if remain_code.len() == 0 {
        // Successfully decoded a message, write decoded message to file
        let f = file.write_fmt(format_args!("{}", apply_hash(pure_code, encode_hash)));
        let _f = match f {
            Ok(file) => file,
            Err(e) => { panic!("{}", e) },
        };
        let f = file.write(b"\n");
        let _f = match f {
            Ok(file) => file,
            Err(e) => { panic!("{}", e) },
        };

    } else {
        let possible_words: Vec<String> = match_length(&remain_code[0], all_words);

        for word in possible_words {
            let check: (bool, Vec<i32>) = check_match(&word, &remain_code[0], encode_hash);
            if check.0 {
                // call decode_remainder on the rest of the code and add the current encoding to the hash table
                let mut next_code: Vec<Vec<i32>> = remain_code.to_vec();
                next_code.remove(0);
                decode_remainder(&next_code, encode_hash, all_words, file, pure_code);
                clean_hash(&check.1, encode_hash);
            }
        }
    }
}


/* Find all the words in the dictionary with the same length as the given encoded word
Accepts {
    target_word: &Vec<<i32>: a vector of i32 values representing the encoded word
    words: &Vec<String>: a vector of all words
}
Returns: A Vector of Strings containing all words of matching length */
fn match_length(target_word: &Vec<i32>, words: &Vec<String>) -> Vec<String> {
    let mut return_vec: Vec<String> = Vec::new();
    for word in words {
        if word.len() == target_word.len() {
            return_vec.push(word.to_string());
        }
    }
    return_vec
}


/* Check if the given word is a possible decoding of the code_word
Accepts {
    word: &String: A string representation of the possible word
    code_word: &[i32]: An array representing the current encoded word
    encode_hash: &mut HashMap<i32, char>:  A mutable HashMap representing the letter associated with each number
}
Returns: tuple: (bool, Vec<i32>): the boolean represents if the word matched, and the Vec<i32> represents the changes made to the HashMap */ 
fn check_match(word: &String, code_word: &[i32], encode_hash: &mut HashMap<i32, char>) -> (bool, Vec<i32>) {
    let mut change: Vec<i32> = Vec::new();

    for i in 0..word.len() {
        let curr_let: char = word.chars().nth(i).unwrap();
        let curr_code: i32 = code_word[i];

        if encode_hash.contains_key(&curr_code) {
            let encode_let = *encode_hash.get(&curr_code).unwrap();
            // If the current letter doesn't match the intended encoded letter, return false
            if encode_let != curr_let {
                clean_hash(&change, encode_hash);
                return (false, Vec::new());
            }
        } else {
            // If the current letter is already assigned to an integer, return false
            if encode_hash.values().any(|&x| x==curr_let) {
                clean_hash(&change, encode_hash);
                return (false, Vec::new());
            }
            // Otherwise add it into the HashMap
            change.push(curr_code);
            encode_hash.insert(curr_code, curr_let);
        }
    }
    (true, change)
}

/* Removes all the keys from the HashMap which do not work with the current encoding
Accepts {
    change: &Vec<i32>: A vector of all keys to be removed from the HashMap
    encode_hash: &mut HashMap<i32, char>: The HashMap to be cleaned
}
*/
fn clean_hash(change: &Vec<i32>, encode_hash: &mut HashMap<i32, char>) {
    for key in change {
        encode_hash.remove(&key);
    }
}


/* Converts the code and hash into a string
Accepts {
    code: &[Vec<i32>]: A vector of vectors of i32 representing the original encoded message
    encode_hash: &HashMap<i32, char>: A HashMap representing the letter associated with each
Returns: A string representing the decoded message */
fn apply_hash(code: &[Vec<i32>], encode_hash: &HashMap<i32, char>) -> String {
    let mut return_str: String = String::new();

    for word in code {
        
        for letter in word {
            if encode_hash.contains_key(&letter) {
                let curr_let: char = *encode_hash.get(&letter).unwrap();
                return_str.push(curr_let);
            }
        }
        return_str.push_str(" ");
    }
    return_str
}