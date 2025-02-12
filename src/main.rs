use std::env;
use std::fs::File;
use std::io::{prelude::*, BufWriter};

fn main() -> std::io::Result<()> {
    let (max_length, do_iterate, incremental, filepath) = handle_args();

    let charset: [char; 62] = get_charset();
    let mut file = BufWriter::new(File::create(filepath).unwrap());

    if do_iterate {
        if incremental {
            for i in 1..max_length + 1 {
                iterate(charset, i, &mut file)?;
            }
        } else {
            iterate(charset, max_length, &mut file)?;
        }
    } else {
        if incremental {
            for i in 1..max_length + 1 {
                recursive(0, 0, charset, vec!['0'; i], i, &mut file)?;
            }
        } else {
            recursive(0, 0, charset, vec!['0'; max_length], max_length, &mut file)?;
        }
        
    }
    Ok(())
}

fn recursive<const CHARSET_LENGTH:usize>(char_index: usize, line_index: usize, charset: [char; CHARSET_LENGTH], mut working_perm: Vec<char>, max_length: usize, file: &mut BufWriter<File>) -> std::io::Result<()> {
    if line_index >= max_length || char_index >= CHARSET_LENGTH {
        return Ok(());
    }

    if line_index == max_length-1 {
        working_perm[line_index] = charset[char_index];
        for char in &working_perm {
            write!(file, "{}", char)?;
        }
        write!(file, "\n")?;
        recursive(char_index+1, line_index, charset, working_perm, max_length, file)?;
    } else {
        working_perm[line_index] = charset[char_index];
        recursive(0, line_index+1, charset, working_perm.clone(), max_length, file)?;
        recursive(char_index+1, line_index, charset, working_perm, max_length, file)?;
    }

    Ok(())
}

fn iterate<const CHARSET_SIZE: usize>(
    charset: [char; CHARSET_SIZE],
    max_length: usize,
    file: &mut BufWriter<File>,
) -> std::io::Result<()> {
    let mut permutation: Vec<char> = vec!['0'; max_length]; // Vec of length `max_length` to hold the current working permutation
    let mut tracker: Vec<usize> = vec![0; max_length]; // Helper vec to keep track of the index of each character permuation

    let mut line_i: usize = 0;
    'primary: loop {
        permutation[line_i] = charset[tracker[line_i]]; // Sets the current permutation character based on the tracker value

        if line_i == max_length - 1 {
            // If this is a complete permutation, write it to file and increment the character tracker index
            for char in &permutation {
                write!(file, "{}", char)?;
            }
            write!(file, "\n")?;
            tracker[line_i] += 1;
        } else {
            // Othewise start working with the next character position
            line_i += 1;
        }

        // While loop to track back character positions for every character that is the last in the character set
        while tracker[line_i] == CHARSET_SIZE {
            tracker[line_i] = 0;

            if line_i == 0 {
                // If we stack all the way back to the first line, generation has completed
                break 'primary;
            } else {
                // Step back a character position and increment the new character position index
                line_i -= 1;
                tracker[line_i] += 1;
            }
        }
    }
    Ok(())
}

fn concat_arrays<T, const A: usize, const B: usize, const C: usize>(
    a: [T; A],
    b: [T; B],
) -> [T; C] {
    assert_eq!(A + B, C);
    let mut iter = a.into_iter().chain(b);
    std::array::from_fn(|_| iter.next().unwrap())
}

fn get_charset<const SIZE: usize>() -> [char; SIZE] {
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

fn handle_args() -> (usize, bool, bool, String) {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("Missing length argument");
    }

    let max_length = args[1].parse::<usize>().expect("Failed to parse length");
    let mut do_iterate = true;
    let mut incremental = false;
    let mut filepath: &str = "./output/res.txt";

    for i in 2..args.len() {
        match args[i].as_str() {
            "-r" => do_iterate = false, // Toggles iterative or recusrive methods
            "-i" => incremental = true, // Toggles whether just to create permutations of max length, or include all lengths from 0 to max length
            "-f" => filepath = &args[i + 1],
            _ => (),
        }
    }

    (max_length, do_iterate, incremental, filepath.to_string())
}