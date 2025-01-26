use std::fs::File;
use std::io::prelude::*;


fn main() {
    let charset: [char; 62] = get_charset();
    let mut file = File::create("./output/res.txt").expect("Error creating file");
    
    iterate(charset, 3, &mut file);
}

// fn recursive<T, const A:usize>(index: usize, max_length: usize, charset: [T; A], permutations: Vec<String>) -> Vec<String> {
//     let mut permutations: Vec<String>;
//     return permutations;

// }

fn iterate<const A: usize>(charset: [char; A], max_length: usize, file: &mut File) {
    let mut permutation: Vec<char> = vec!['0'; max_length]; // Vec of length `max_length` to hold the current working permutation
    let mut tracker: Vec<usize> = vec![0; max_length]; // Helper vec to keep track of the index of each character permuation

    let mut line_i: usize = 0;
    let mut underway: bool = true;

    while underway {
        permutation[line_i] = charset[tracker[line_i]]; // Sets the current permutation character based on the tracker value

        if line_i == max_length - 1 {
            // If this is a complete permutation, write it to file and increment the character tracker index
            write!(file, "{}\n", Vec::from_iter(permutation.iter().map(|i| i.to_string())).join("")).expect("Failed to write");

            // results.push(permutation.clone());
            tracker[line_i] += 1;
        } else {
            // Othewise start working with the next character position
            line_i += 1;
        }

        // While loop to track back character positions for every character that is the last in the character set
        while tracker[line_i] == A {
            tracker[line_i] = 0;

            if line_i == 0 {
                // If we stack all the way back to the first line, generation has completed
                underway = false;
            } else {
                // Step back a character position and increment the new character position index
                line_i -= 1;
                tracker[line_i] += 1;
            }
        }
    }
}

fn concat_arrays<T, const A: usize, const B: usize, const C: usize>(
    a: [T; A],
    b: [T; B],
) -> [T; C] {
    assert_eq!(A + B, C);
    let mut iter = a.into_iter().chain(b);
    std::array::from_fn(|_| iter.next().unwrap())
}

fn get_charset<const A: usize>() -> [char; A] {
    const LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    const UPPER: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    const NUMBER: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let charset1: [char; 52] = concat_arrays(LOWER, UPPER);
    concat_arrays(charset1, NUMBER)
}
