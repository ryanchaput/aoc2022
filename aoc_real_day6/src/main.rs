use std::fs;

fn main() {
    let file_path = "real_day6_puzzle.txt";

    let contents: String = fs::read_to_string(file_path).expect("Should've read the input.");

    let result: usize = find_start14(contents);

    //let s = "aaabcdd".to_owned();

    println!("{}", result);
}

// Return the index where the starting message marker is first detected
// Marker is a group of four consecutive unique characters, index starts at 1
fn find_start(input: String) -> usize {
    let chars: Vec<char> = input.chars().collect();

    println!("{:?}", chars);

    // If not even four chars, return -1 as error
    if chars.len() < 4 {
        return 0
    }

    let mut window: [char; 4] = [chars[0], chars[1], chars[2], chars[3]];
    /*for i in 0..4 {
        window[i] = chars[i];
    }*/

    // Start the index at 4, the lowest possible for a marker
    let mut index: usize = 4;

    /*
    if distinct_chars(window) {
        return index;
    } else {
        index += 1;
        for j in 0..3 {
            window[j] = window[j + 1];
        }
        window[3] = chars[index];
    }*/
    while !distinct_chars(window) {
        println!("Called distinct chars.");
        
        for j in 0..3 {
            window[j] = window[j + 1];
        }
        window[3] = chars[index];
        index += 1;
    }

    index
}

fn find_start14(input: String) -> usize {
    let chars: Vec<char> = input.chars().collect();

    println!("{:?}", chars);

    // If not even fourteen chars, return -1 as error
    if chars.len() < 4 {
        return 0
    }

    //let mut window: [char; 14] = [chars[0], chars[1], chars[2], chars[3]];
    let mut window: [char; 14] = ['0'; 14];
    for j in 0..14 {
        window[j] = chars[j];
    }
    
    // Start the index at 4, the lowest possible for a marker
    let mut index: usize = 14;

    while !distinct_chars14(window) {
        println!("Called distinct chars.");
        
        for j in 0..13 {
            window[j] = window[j + 1];
        }
        window[13] = chars[index];
        index += 1;
    }

    index
}

// Returns True if all the chars in the array are distinct
fn distinct_chars(window: [char; 4]) -> bool {
    let mut i: usize = 0;
    while i < 4 {
        let ch1 = window[i];
        for j in (i + 1)..4 {
            
            // If characters are not distinct, this is not the marker
            if ch1 == window[j] {
                //println!("{ch1}");
                return false;
            }
        }
        i += 1;
    }
    // If the false statement never hits, this is the marker
    true
}


fn distinct_chars14(window: [char; 14]) -> bool {
    let mut i: usize = 0;
    while i < 14 {
        let ch1 = window[i];
        for j in (i + 1)..14 {
            
            // If characters are not distinct, this is not the marker
            if ch1 == window[j] {
                //println!("{ch1}");
                return false;
            }
        }
        i += 1;
    }
    // If the false statement never hits, this is the marker
    true
}