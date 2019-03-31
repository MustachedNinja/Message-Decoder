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


fn main() {
    
    // Get the code from the file passed in
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Filename: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("The file was not read correctly");

    println!("Text:\n{}", contents);

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

    println!("{:?}", contents_vec);

    // { Find the word with the most unique code-numbers and create a has using that words code-numbers }
    let most_var = most_variety(&contents_vec);

    println!("{}", most_var);

}


// Finds the word in the given |code_vec| which has the most unique code-numbers
// Accepts: a vector of vectors containing i32 values: [ [ i32, i32 ], [ i32, i32, i32 ] ]
// Returns: an i32 value representing the index of the most unique word 
fn most_variety(code_vec: &[Vec<i32>]) -> i32 {
    let mut greatest_index: i32 = 0;
    let mut greatest_variety = 0;

    for x in 0..code_vec.len() {
        let variety = unique(&code_vec[x]);
        if variety > greatest_variety {
            greatest_index = x as i32;
            greatest_variety = variety;
        }
    }

    greatest_index
}

// Counts the number of unique code-numbers in a given |vec|
// Accepts: a vector containing i32 values: [ i32, i32, i32 ]
// Returns: an i32 value representing the number of unique values in the given |vec|
fn unique(vec: &[i32]) -> i32 {
    // counts the number of occurrences of a code number in an array
    let mut count: i32 = 0;
    let mut unique_nums = Vec::new(); // Create a vector (dynamic array)

    for letter in vec.iter() {
        if !(unique_nums.contains(letter)) {
            count = count + 1;
            unique_nums.push(*letter);
        };
    };

    count
}

mod test;