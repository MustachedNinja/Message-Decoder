// Kostya Yatsuk - 2019

// This code will accept a file which contains a numerical code and it will try to decode it, outputing possible results into an output text file.
// Stretch: feed the output file into a grammar check to determine which of the possible outputs is the more likely.
// The decoder will accept a .txt file with code-numbers being represented by numbers separated by spaces, and individual words being separated by "/r/n"
// It will then find a word with the most number of unique code-numbers and attempt to match it to an existing word from a word bank.
// Next, a hash-table will be built up using the coded letters, and then applied to the other coded words. Then the decoder will loop through all the coded words,
// trying to fill in their code-numbers. When a word is created this way, the decoder checks if this word exists in the word bank. If not, it scraps this hash table
// and starts again using a new word which fits the most unique word.

use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {

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

    // words will be sorted by length first, then alphabetically.

    let split_words = words_string.split("\r\n");
    let mut all_words: Vec<String> = Vec::new();

    for s in split_words {
        all_words.push(s.to_string());
    }

    let mut encode_hash: HashMap<i32, char> = HashMap::new();

    if decode_remainder(&contents_vec, &mut encode_hash, &all_words) {
        println!("Decode successful");
        println!("Decoded message: \"{}\"", apply_hash(&contents_vec, &encode_hash));
    } else {
        println!("Decode failed");
    }

    

    // decode_remainder has modified the hash

    // { Loop through encoded words }
    // { Find all words with the same length as the  }
    // { Loop through all possible words }
    // { For each word call a method func(curr_code, curr_testword) -> boolean. It will return true if the word matches}

}


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

fn decode_remainder(code: &[Vec<i32>], encode_hash: &mut HashMap<i32, char>, all_words: &Vec<String>) -> bool {

    if code.len() == 0 {
        return true;
    }

    let possible_words: Vec<String> = match_length(&code[0], all_words);

    for word in possible_words {
        if check_match(&word, &code[0], encode_hash) {
            // { call decode_remainder on the rest of the code and add the current encoding to the hash table }
            let mut next_code: Vec<Vec<i32>> = code.to_vec();
            next_code.remove(0);
            return decode_remainder(&next_code, encode_hash, all_words);
        }
    }
    false

}

// Find all the words in the dictionary with the same length as the given encoded word
// Accepts: a vector of i32 values representing the encoded word
// Returns: A Vector of Strings containing all words of matching length
fn match_length(target_word: &Vec<i32>, words: &Vec<String>) -> Vec<String> {
    let mut return_vec: Vec<String> = Vec::new();
    for word in words {
        if word.len() == target_word.len() {
            return_vec.push(word.to_string());
        }
    }
    return_vec
}

// Check if the given word is a possible decoding of the code_word
// Accepts: A String representation of the possible decoding and a vector of i32 representing the encoded word
// Returns: A boolean saying whether or not the given word is a possible candidate
// Note: This method also modifies the global hash table 
fn check_match(word: &String, code_word: &[i32], encode_hash: &mut HashMap<i32, char>) -> bool {
    let mut change: Vec<i32> = Vec::new();

    for i in 0..word.len() {
        // let i_val: i32 = i.parse::<i32>().unwrap();
        let curr_let: char = word.chars().nth(i).unwrap();
        let curr_code: i32 = code_word[i];
        if encode_hash.contains_key(&curr_code) {
            let encode_let = *encode_hash.get(&curr_code).unwrap();
            if encode_let != curr_let {
                for key in change {
                    // let key_val: char = key.parse::<char>().unwrap();
                    encode_hash.remove(&key);
                }
                return false;
            }
        } else {
            change.push(curr_code);
            encode_hash.insert(curr_code, curr_let);
        }
    }
    true
}

// Finds the word in the given |code_vec| which has the most unique code-numbers
// Accepts: a vector of vectors containing i32 values: [ [ i32, i32 ], [ i32, i32, i32 ] ]
// Returns: an i32 value representing the index of the most unique word 
// fn most_variety(code_vec: &[Vec<i32>]) -> i32 {
//     let mut greatest_index: i32 = 0;
//     let mut greatest_variety = 0;

//     for x in 0..code_vec.len() {
//         let variety = unique(&code_vec[x]);
//         if variety > greatest_variety {
//             greatest_index = x as i32;
//             greatest_variety = variety;
//         }
//     }

//     greatest_index
// }

// // Counts the number of unique code-numbers in a given |vec|
// // Accepts: a vector containing i32 values: [ i32, i32, i32 ]
// // Returns: an i32 value representing the number of unique values in the given |vec|
// fn unique(vec: &[i32]) -> i32 {
//     // counts the number of occurrences of a code number in an array
//     let mut count: i32 = 0;
//     let mut unique_nums = Vec::new(); // Create a vector (dynamic array)

//     for letter in vec.iter() {
//         if !(unique_nums.contains(letter)) {
//             count = count + 1;
//             unique_nums.push(*letter);
//         };
//     };

//     count
// }

mod test;