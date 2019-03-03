use std::env;
use std::fs;

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Filename: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("The file was not read correctly");

    println!("Text:\n{}", contents);

    // At this point contents contains the code
    // Convert contents into an array of arrays -> [ [1, 2, 3], [4, 5] ]
    // From here build up a hashmap assigning numbers to letters.
    // Using that hashmap and that array of arrays, build a string for each word and check if its in the "dictionary"
    // If not, restart the process picking new letters for the hashmap
    // If yes, keep going until all the words are checked, then add the resulting sentence and hashmap to the output txt 

    // Process:
    // Decoder will accept a .txt file as a parameter in command line
    // Text file is of format "1 2 3 4 12 \n 2 8 10 \n" with spaces representing new letters and newlines representing new words
    // In output it will generate a .txt file as a list of all possible sentences

    // Separate the encoded text into a hash-map which matches numbers to letters
    // Essentially the program will plug various hashmaps into the code. It will then loop through every word in the code and check if 
    // its a word that exists in the additional words.txt file. If all the words pass, save this as a potential sentence into the output text file.
    
    // { Stretch }
    // If everything in this project is done, add a grammar-checker at the end to filter out sentences which are not grammatically or syntactically correct
}
