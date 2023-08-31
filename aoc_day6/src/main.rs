use std::fs;
use std::iter::Iterator;

struct Move(usize, usize, usize);

fn main() {
    let mut s1: Vec<char> = vec!['Z', 'T', 'F', 'R', 'W', 'J', 'G'];
    let mut s2: Vec<char> = vec!['G', 'W', 'M'];
    let mut s3: Vec<char> = vec!['J', 'N', 'H', 'G'];
    let mut s4: Vec<char> = vec!['J', 'R', 'C', 'N', 'W'];
    let mut s5: Vec<char> = vec!['W', 'F', 'S', 'B', 'G', 'Q', 'V', 'M'];
    let mut s6: Vec<char> = vec!['S', 'R', 'T', 'D', 'V', 'W', 'C'];
    let mut s7: Vec<char> = vec!['H', 'B', 'N', 'C', 'D', 'Z', 'G', 'V'];
    let mut s8: Vec<char> = vec!['S', 'J', 'N', 'M', 'G', 'C'];
    let mut s9: Vec<char> = vec!['G', 'P', 'N', 'W', 'C', 'J', 'D', 'L'];

    let mut stacks: [&mut Vec<char>; 9] = [&mut s1, &mut s2, &mut s3, &mut s4, &mut s5, &mut s6, &mut s7, &mut s8, &mut s9];
    //let stack_ref = &mut stacks;

    let file_path = "day6_puzzle.txt";
    let _contents = fs::read_to_string(file_path).expect("Could not read the file.");

    let moves_list = Iterator::last(_contents.split("\n\n")).expect("Couldn't find last element.");
    //let moves: Vec<&str> = moves_list.lines().collect();

    //println!("{:?}", moves);
    for line in moves_list.lines() {
        let turn = decipher_move(line);
        handle_move(turn, &mut stacks);
    }

    let mut result: Vec<char> = Vec::new();

    for list in stacks {
        let c = list.pop().expect("Expected a char");
        result.push(c);
    }

    let s: String = result.into_iter().collect();

    println!("{s}");
}

// Deciphers each move, returns a tuple
fn decipher_move(m: &str) -> Move {
    let _nums: Vec<&str> = m.split(" ").collect();
    let num1 = _nums[1].parse::<usize>().expect("Expected a num");
    let num2 = _nums[3].parse::<usize>().expect("Expected a num");
    let num3 = _nums[5].parse::<usize>().expect("Expected a num");
    let values = Move(num1, num2, num3);
    values
}

// Handle each move with the three nums and the stacks array
fn handle_move(turn: Move, stacks: &mut [&mut Vec<char>; 9]) -> () {
    let mut i = 0;
    
    while i < turn.0 {
        let from: &mut Vec<char> = stacks[turn.1 - 1];
        let e: char = from.pop().expect("Expected a char.");
        let to: &mut Vec<char> = stacks[turn.2 - 1];
        to.push(e);
        i += 1;
    }
}