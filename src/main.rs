/*
Author: Saad-py
Time: 3 hours
Project Name: KWIC ( keyword in context ) index


output:

This program prints out the KWIC index of a text from a txt file
But the KWIC is not aligned nor Alphabetical

 */



// use standard files lib to read txt
use std::fs;

fn main() {
    // first we create a list of stop words
    let stop_words = ["but","has","the", "is", "a", "an", "they", "and", "I", "in", "there", "was", "who", "The", "know", "it"];

    // This Vector will hold everything case sensitive because stop words will be lowercase and Titles will be CAPS
    let mut m = Vec::new();

    // Make string for filename
    // We need to add src\\ because cargo project will think the main dir is KWIC_index but it's actually src\Text.txt
    let f_name = "src\\Text.txt";

    // read it and convert to string
    let file = (fs::read_to_string(f_name).expect("Couldn't read file")).to_string();
    // We read file successfully 👏

    // Now lets split the file by each new lines
    let lines = file.lines();
    // Basically cloning for later
    let l_a = file.lines();

    // Iterate through every line in the text
    for i in lines {

        // Basically words in a sentence
        let sentences = i.split(" ");

        // for every word in a sentence which is a Split<&str>
        for a in sentences {

            // Every word is a String
            let mut word = a.to_string();

            // I stop words contain that word
            if stop_words.contains(&a) {

                // Then convert it to lowercase
                word = a.to_lowercase();

            } else {
                // turn it into uppercase
                word = a.to_uppercase();

            }
            // we append the word in to a vector
            m.push(word);
        }
    }

    // Printing the result
    // Making a var index for finding where the line ends
    let mut index = 0;

    // For i in every line in text
    for i in l_a {

        // sne is basically splitting every line
        let sne = i.split(" ");

        // This vector is for getting where is the last word and this is related to var index
        let mut las_word = Vec::new();

        // For every word in sentence append it into the vector
        for l in i.split(" ") {las_word.push(l)}

        // For every word in sentence
        for a in sne {

            // We print the word from Vector m at index "index"
            print!("{} ",m[index]);
            index+=1;

            // now we check if the word is the last word in sentence
            if a == las_word[las_word.len()-1] {

                // If yes then we print a new line
                print!("\n")
            }
        }
    }

    // THAT IS IT

}
// MY OWN CODE
