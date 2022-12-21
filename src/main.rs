use std::io;
use colored::*;

const WIDTH: usize = 9;
const HEIGHT: usize = 7;

type Board = [[i32; WIDTH]; HEIGHT];

fn print_board(board: &Board) {
    // print!("{}[2J", 27 as char);
    clearscreen::clear().unwrap();
    println!("{}", "             CONNECT 4               ".reversed().blue().bold());
    for row in board.iter() {
        println!("+---+---+---+---+---+---+---+---+---+");
        for value in row.iter() {
            print!("| ");
            if *value == 1 {
                print!("{}", "●".red().bold());
            } else if *value == 2 {
                print!("{}", "●".yellow().bold());
            }
            else {
                print!(" ");
            }
            print!(" ");
        }
        println!("|");
    }
    println!("+---+---+---+---+---+---+---+---+---+");
    println!("{}","  1   2   3   4   5   6   7   8   9  ".italic().bold())
}

/**
Returns the row that the piece was placed on
*/
fn place_piece(board: &mut Board, column: usize, piece: i32) -> bool {
    let mut row: usize = HEIGHT - 1;
    while board[row][column] > 0 {
        if row == 0 {
            return false;
        }
        row -= 1;
    }
    board[row][column] = piece;
    true
}

fn check_winner_at_position(board: &Board, row: usize, col: usize) -> bool {
  let piece = board[row][col];
  let mut won = true;
  // first check if this position starts a win from left to right
  if col <= WIDTH - 4 { // which is only possible if we are far enough to the left
    for i in 1..4 {
      if board[row][col + i] != piece {
        won = false;
        break;
      } 
    }
  }
  else {
    won = false;
  }
  if won {
    return true;
  }
  won = true;
  // now check top down
  if row <= HEIGHT - 4 {
    for i in 1..4 {
      if board[row + i][col] != piece {
        won = false;
        break;
      }
    }
  }
  else {
    won = false;
  }
  if won {
    return true;
  }
  won = true;
  // check for a diagonal to the right
  if row <= HEIGHT - 4 && col <= WIDTH - 4 {
    for i in 1..4 {
      if board[row + i][col + i] != piece {
        won = false;
        break;
      }
    }
  }
  else {
    won = false;
  }
  if won {
    return true;
  }
  won = true;
  // finally check a diagonal to the left
  if row <= HEIGHT - 4 && col >= 3 {
    for i in 1..4 {
      if board[row + i][col - i] != piece {
        won = false;
        break;
      }
    }
  }
  else {
    won = false;
  }
  if won {
    return true;
  }
  false
}

fn check_winner(board: &Board, piece: i32) -> bool {
  for i in 0..HEIGHT {
    for j in 0..WIDTH {
      if board[i][j] == piece && check_winner_at_position(board, i, j) {
        println!("{} {}", i, j);
        return true;
      }
    }
  }
  false
}

fn get_display_name(player: i32) -> ColoredString {
  if player == 1 { 
    "Red".red().bold() 
  } else { 
    "Yellow".yellow().bold() 
  }
}

// needs a decent terminal to not look horrible
fn animate_board(board: &mut Board) {
    let mut turn = 1;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
        place_piece(board, j, turn);
        turn = turn % 2 + 1;
        print_board(board);
        std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

fn calculate_pattern_score(pattern: &Vec<i32>, player: i32) -> i32 {
  let mut last = 0;
  let mut rank = 0;
  let mut cur = 1;
  let mut scale = 0;
  for p in pattern.iter() {
    if *p == last {
      cur *= 2;
    }
    else {
      rank += cur * scale;
      cur = 1;
    }
    last = *p;
    if last == 0 {
      scale = 0;
    }
    else if last == player {
      scale = 1;
    }
    else {
      scale = -1;
    }
  }
  rank += cur * scale;
  rank
}

fn calculate_score(board: &Board, player: i32) -> i32 {
    // long patterns of the given player are good
    let mut score = 0;
    // collect patterns along each row
    for i in 0..HEIGHT {
        let mut pattern = Vec::new();
        for j in 0..WIDTH {
        pattern.push(board[i][j]);
        }
        score += calculate_pattern_score(&pattern, player);
    }
    for i in 0..WIDTH {
        let mut pattern = Vec::new();
        for j in 0..HEIGHT {
        pattern.push(board[j][i]);
        }
        score += calculate_pattern_score(&pattern, player);
    }
    for i in 0..HEIGHT {
        let mut pattern = Vec::new();
        for j in 0..(i + 1) {
        pattern.push(board[HEIGHT - 1 - i + j][j]);
        }
        score += calculate_pattern_score(&pattern, player);
    }
    for i in 0..HEIGHT {
        let mut pattern = Vec::new();
        for j in 0..(i + 1) {
        pattern.push(board[HEIGHT - 1 - i + j][WIDTH - 1 - j]);
        }
        score += calculate_pattern_score(&pattern, player);
    }
    for i in 1..WIDTH {
        let mut pattern = Vec::new();
        for j in 0..std::cmp::min(WIDTH - i, HEIGHT) {
        pattern.push(board[j][i + j]);
        }
        score += calculate_pattern_score(&pattern, player);
    }
    for i in 1..WIDTH {
        let mut pattern = Vec::new();
        for j in 0..std::cmp::min(WIDTH - i, HEIGHT) {
        pattern.push(board[j][WIDTH - 1 - (i + j)]);
        }
        score += calculate_pattern_score(&pattern, player);
    }
    
    score
}

fn game_score(board: &Board) -> i32 {
    calculate_score(board, 2)
}

// returns the score and column to play that maximizes the score
fn minmax(board: &Board, maximizing: bool, depth: i32, mut alpha: i32, mut beta: i32) -> [i32; 2] {
    if depth == 0 {
        return [game_score(board), 0];
    }
    let player = if maximizing { 2 } else { 1 };
    if maximizing {
        let mut max = [-9999, 0];
        for i in 0..WIDTH {
            if board[0][i] == 0 { // if there is space left in this column
                let mut boardCopy = board.to_owned();
                place_piece(&mut boardCopy, i, player);
                let [score, _] = minmax(&boardCopy, false, depth - 1, alpha, beta);
                if score > max[0] {
                    max = [score, i.try_into().unwrap()];
                }
                alpha = std::cmp::max(alpha, score);
                if beta <= alpha {
                    break;
                }
            }
        }
        return max;
    }
    else {
        let mut min = [9999, 0];
        for i in 0..WIDTH {
            if board[0][i] == 0 {
                let mut boardCopy = board.to_owned();
                place_piece(&mut boardCopy, i, player);
                let [score, _] = minmax(&boardCopy, true, depth - 1, alpha, beta);
                if score < min[0] {
                    min = [score, i.try_into().unwrap()];
                }
                beta = std::cmp::min(beta, score);
                if beta <= alpha {
                    break;
                }
            }
        }
        return min;
    }
}

fn main() {
    let mut board: Board = [[0; 9]; 7];
    let mut turn = 1;
    let mut game_over = false;
    // animate_board(&mut board);
    while !game_over {
        print_board(&board);
        // calculate score
        println!("{}: {} | {}: {}", get_display_name(1), calculate_score(&board, 1), get_display_name(2), calculate_score(&board, 2));
        println!("{}'s turn: ", get_display_name(turn));

        if turn == 2 {
            println!("Calculating...");
            let [score, pos] = minmax(&board, true, 6, -9999, 9999);
            // std::thread::sleep(std::time::Duration::from_millis(500));
            place_piece(&mut board, pos.try_into().unwrap(), turn);
            if check_winner(&board, turn) {
                print_board(&board);
                println!("{} Won!", get_display_name(turn));
                game_over = true;
            }
            turn = turn % 2 + 1;
        }
        else {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error reading line");

            let trimmed = input.trim();
            match trimmed.parse::<usize>() {
                Ok(i) => {
                    if i < 1 || i > WIDTH {
                        println!("Enter a valid column");
                    }
                    else {
                        place_piece(&mut board, i - 1, turn);
                        if check_winner(&board, turn) {
                            print_board(&board);
                            println!("{} Won!", get_display_name(turn));
                            game_over = true;
                        }
                        turn = turn % 2 + 1;
                    }
                }
                Err(..) => {
                    println!("Enter a valid column");
                }
            }
        }
    }
}
