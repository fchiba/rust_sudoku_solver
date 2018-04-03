use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Board = Vec<Option<u32>>;
type Candidate = [[bool; 9]; 81];

fn create_board(filename : &str) -> Result<Board, String> {
    let mut board = Vec::new();
    let file = match File::open(&filename) {
        Err(why) => {
            return Err(format!("couldn't open {}: {}", filename, Error::description(&why)));
        },
        Ok(file) => file
    };
    
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut count = 0;
    for line in lines {
        let line = line.unwrap();
        if line.len() != 9usize {
            return Err(format!("Invalid line length: {}", line));
        }
        board.extend(line.chars().map(|c| match c.to_digit(10) {
            Some(n) => Some(n - 1),
            None => None
        }));
        
        count += 1;
    }
    if count != 9 {
        return Err("Invalid line nubmer".to_string());
    }
    Ok(board)
}

fn check_unique(board: &mut Board, candidates: &Candidate) -> Vec<(usize, u32)> {
    let mut founds = Vec::new();
    for (idx, num) in board.iter().enumerate() {
        if let &None = num {
            // filter candidates
            let nums : Vec<usize> = candidates[idx].iter().enumerate().filter_map(|(num,val)| if *val {Some(num)} else {None}).collect();
            
            // check number
            if nums.len() == 1 {
                let num = nums[0];
                founds.push((idx, num as u32));
            }
            
            for num in nums {
                let row = idx / 9;
                let col = idx % 9;
                let boxnum = (row / 3) * 3 * 9 + (col / 3) * 3;
                
                // check row
                let mut count = 0;
                for i in 0..9 {
                    let target = row * 9 + i;
                    if candidates[target][num] {
                        count += 1;
                    }
                }
                if count == 1 {
                    founds.push((idx, num as u32));
                    continue;
                }
                
                // check col
                let mut count = 0;
                for i in 0..9 {
                    let target = col + i * 9;
                    if candidates[target][num] {
                        count += 1;
                    }
                }
                if count == 1 {
                    founds.push((idx, num as u32));
                    continue;
                }
                
                // check box
                let mut count = 0;
                for j in 0..3 {
                    for i in 0..3 {
                        let target = boxnum + i + j * 9;
                        if candidates[target][num] {
                            count += 1;
                        }
                    }
                }
                if count == 1 {
                    founds.push((idx, num as u32));
                    continue;
                }
            }
        }
    }
    println!("{:?}", founds);
    founds
}

fn put_number(idx: usize, number: u32, candidates: &mut Candidate) {
    println!("putting {} into {}", number, idx);
    let num: usize = number as usize;
    assert!(candidates[idx][num]);
    
    // remove candidates at idx
    for i in 0..9 {
        if i != num {
            candidates[idx][i] = false;
        }
    }
    // remove candidates in row
    let row = idx / 9;
    for i in 0..9 {
        let target = row * 9 + i;
        if target != idx {
            candidates[target][num] = false;
        }
    }
    // remove candidates in col
    let col = idx % 9;
    for i in 0..9 {
        let target = col + i * 9;
        if target != idx {
            candidates[target][num] = false;
        }
    }
    // remove candidates in box
    let boxnum = (row / 3) * 3 * 9 + (col / 3) * 3;
    for j in 0..3 {
        for i in 0..3 {
            let target = boxnum + i + j * 9;
            if target != idx {
                candidates[target][num] = false;
            }
        }
    }
    // for (i,x) in candidates.iter().enumerate() {
    //     println!("{}: {:?}", i, x);
    // }
}

fn print_board(board : &Board) {
    for j in 0..9 {
        for i in 0..9 {
            print!("{}", match board[j*9+i] {
                Some(n) => (n+1).to_string(),
                None => " ".to_string()
            });
        }
        println!("")
    }
}

fn fill_number_with_uniqness(mut board : &mut Board) -> bool {
    let mut candidates: Candidate = [[true; 9]; 81];
    for (idx, num) in board.iter().enumerate() {
        if let &Some(number) = num {
            put_number(idx, number, &mut candidates)
        }
    }
    loop {
        let founds = check_unique(&mut board, &candidates);
        
        if founds.len() == 0 {
            break;
        }
        
        for (idx, num) in founds {
            board[idx] = Some(num);
            put_number(idx, num, &mut candidates);
        }
    }
    false
}

fn main() {
    let filename = match env::args().nth(1) {
        Some(arg) => arg,
        None => {
            println!("Usage: {} FILENAME", env::args().next().unwrap());
            return
        }
    };
    let board = match create_board(&filename) {
        Ok(board) => board,
        Err(msg) => {
            println!("{}", msg);
            return
        }
    };
    //println!("board: {:?}", board);
    
    let mut answer = board.to_vec();
    while fill_number_with_uniqness(&mut answer) {
    }
    print_board(&answer);
}
