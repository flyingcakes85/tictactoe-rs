use std::{io, process::exit};

/// Prints the board in a pretty format
///
/// # Arguments
///
/// * `board` : A 3x3 char array
fn print_board(board: &[[char; 3]; 3]) {
    println!(
        "
  {} | {} | {}
 ---|---|---
  {} | {} | {}
 ---|---|---
  {} | {} | {}
 ",
        board[0][0],
        board[0][1],
        board[0][2],
        board[1][0],
        board[1][1],
        board[1][2],
        board[2][0],
        board[2][1],
        board[2][2],
    );
}

/// Check if a player has won yet
///
/// # Arguments
///
/// * board : a 3x3 char array
///
/// # Return
///
/// The character which has won or
/// `_` if nobody has won
fn check_win(board: &[[char; 3]; 3]) -> char {
    // horizontal checks
    if board[0][0] == board[0][1] && board[0][1] == board[0][2] {
        return board[0][0];
    } else if board[1][0] == board[1][1] && board[1][1] == board[1][2] {
        return board[1][0];
    } else if board[2][0] == board[2][1] && board[2][1] == board[2][2] {
        return board[2][0];
    }
    // vectical checks
    else if board[0][0] == board[1][0] && board[1][0] == board[2][0] {
        return board[0][0];
    } else if board[0][1] == board[1][1] && board[1][1] == board[2][1] {
        return board[0][1];
    } else if board[0][2] == board[1][2] && board[1][2] == board[2][2] {
        return board[0][2];
    }
    // diagonals
    else if (board[0][0] == board[1][1] && board[1][1] == board[2][2])
        || (board[0][2] == board[1][1] && board[1][1] == board[2][0])
    {
        return board[1][1];
    }

    // nobody wins yet
    '_'
}

/// The glue of our program ;-)
fn main() {
    // grid to store stato of each cell
    let mut board = [['0', '1', '2'], ['3', '4', '5'], ['6', '7', '8']];

    // whose turn is it?
    // false for player 1
    // true for player 2
    let mut turn = true;

    // vector to store filled cells
    // to avoid filling it twice
    let mut filled_cells: Vec<i8> = vec![];

    for _ in 0..9 {
        print_board(&board);

        // loops till valid input is given
        loop {
            println!(
                "Player {} enter number of cell to mark ",
                if turn { "1" } else { "2" }
            );

            // Reads input for cell to fill
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Colud not read input");

            // If we have error, then 10 is stored.
            // It will fail the final if statement,
            // and loop will run again.
            let cell: i8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 10,
            };

            // Check if cell hasn't been filled alredy
            if filled_cells.iter().any(|&val| val == cell) {
                println!("[WARN] Cell {} is already filled.", cell);
                continue;
            }

            // If valied cell then mark it
            // and break out from the loop
            if cell < 9 && cell >= 0 {
                board[(cell / 3) as usize][(cell % 3) as usize] = if turn { 'x' } else { 'o' };
                turn = !turn;
                filled_cells.push(cell);
                break;
            }
        }

        // If someone has won then inform and exit
        match check_win(&board) {
            'x' => {
                print_board(&board);
                println!("Player 1 wins");
                exit(0);
            }
            'o' => {
                print_board(&board);
                println!("Player 2 wins");
                exit(0);
            }
            _ => continue,
        }
    }

    // if we reach here, means nobody won
    println!("Looks like nobody won");
}
