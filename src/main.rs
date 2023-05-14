use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

const ROWS: usize = 7;
const COLS: usize = 8;
const COLORS: &[char] = &['r', 'g', 'b', 'y', 'k'];
const DEPTH: i32 = 4; //minimax depth

type Grid = [[(char, Option<char>); COLS]; ROWS];

struct Game {
    grid: Grid,
    player1: char,
    player2: char,
    current_player: char,
}

impl Clone for Game {
    fn clone(&self) -> Game {
        let mut cloned_grid: Grid = [[('\0', None); COLS]; ROWS];

        for (row_index, row) in self.grid.iter().enumerate() {
            for (col_index, &(color, owner)) in row.iter().enumerate() {
                cloned_grid[row_index][col_index] = (color, owner);
            }
        }

        Game {
            grid: cloned_grid,
            player1: self.player1,
            player2: self.player2,
            current_player: self.current_player,
        }
    }
}


impl Game {
    
    fn new(random: bool) -> Game {
        if random {
            let mut rng = thread_rng();
            let mut grid: Grid = [[('\0', None); COLS]; ROWS];

            for row in 0..ROWS {
                for col in 0..COLS {
                    let color = *COLORS.choose(&mut rng).unwrap();
                    grid[row][col] = (color, None);
                }
            }

            grid[ROWS - 1][0].1 = Some('X'); // Player 1 owns the bottom-left corner cell
            grid[0][COLS - 1].1 = Some('O'); // Player 2 owns the top-right corner cell

            Game {
                grid,
                player1: 'X',
                player2: 'O',
                current_player: 'X',
            }
        } else {
            Game::from_manual_input()
        }
    }

    fn from_manual_input() -> Game {
        let mut grid: Grid = [[('\0', None); COLS]; ROWS];

        println!("Working row-wise, enter {} colors for each row starting from the top row.", COLS);
    
        for row in 0..ROWS {
            for col in 0..COLS {
                loop {
                    let mut input = String::new();
                    print!("Enter a color for cell [{}, {}]: ", row, col);
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut input).unwrap();
                    let color = input.trim().chars().next();
    
                    match color {
                        Some(c) => {
                            if COLORS.contains(&c) {
                                grid[row][col] = (c, None);
                                break;
                            } else {
                                println!("Invalid color. Please try again.");
                            }
                        }
                        None => {
                            println!("No input detected. Please try again.");
                        }
                    }
                }
            }
        }
    
        grid[ROWS - 1][0].1 = Some('X'); // Player 1 owns the bottom-left corner cell
        grid[0][COLS - 1].1 = Some('O'); // Player 2 owns the top-right corner cell
    
        Game {
            grid,
            player1: 'X',
            player2: 'O',
            current_player: 'X',
        }
    }

    fn print_grid(&self) {
        let color_codes = &["\x1b[31m", "\x1b[32m", "\x1b[34m", "\x1b[33m", "\x1b[90m"];
    
        for row in &self.grid {
            for &(color, owner) in row {
                let symbol = match owner {
                    Some(player) => if player == 'X' { 'X' } else { 'O' },
                    None => 'N',
                };
    
                let color_index = COLORS.iter().position(|&c| c == color);
                let colored_char = match color_index {
                    Some(index) => format!("{}{}\x1b[0m", color_codes[index], symbol),
                    None => format!("{}", symbol),
                };
    
                print!("{} ", colored_char);
            }
            println!();
        }

        println!();

        let (player1_count, player2_count) = count_owned_cells(&self);
        
        println!("Player X: {}", player1_count);
        println!("Player O: {}", player2_count);
    }
    
    
    
    
    
    fn make_move(&mut self, color: char) {
        let mut owned_cells: Vec<(usize, usize)> = Vec::new();
        let mut border_cells: Vec<(usize, usize)> = Vec::new();
    
        for row in 0..ROWS {
            for col in 0..COLS {
                if let (_, Some(player)) = self.grid[row][col] {
                    if player == self.current_player {
                        owned_cells.push((row, col));
                    }
                } else if self.is_adjacent_to_owned_cell(row, col) && self.grid[row][col].0 == color {
                    border_cells.push((row, col));
                }
            }
        }
    
        for &(row, col) in owned_cells.iter() {
            self.grid[row][col].0 = color;
        }
    
        for &(row, col) in border_cells.iter() {
            if self.grid[row][col].1.is_none() {
                self.grid[row][col].1 = Some(self.current_player);
            }
        }
        self.switch_player();
    }
    
    fn is_adjacent_to_owned_cell(&self, row: usize, col: usize) -> bool {
        let neighbors: Vec<(isize, isize)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    
        for (dx, dy) in neighbors.iter() {
            let new_row = (row as isize + dx) as usize;
            let new_col = (col as isize + dy) as usize;
    
            if new_row < ROWS && new_col < COLS && self.grid[new_row][new_col].1 == Some(self.current_player) {
                return true;
            }
        }
    
        false
    }

    fn is_valid_move(&self, color: char) -> bool {
        //to be valid, chosen color must not be the same as the current player's color or the opponent's color

        let player1_color = self.grid[ROWS - 1][0].0;
        let player2_color = self.grid[0][COLS - 1].0;

        if color == player1_color || color == player2_color || !COLORS.contains(&color) {
            return false;
        }

        true
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == self.player1 {
            self.player2
        } else {
            self.player1
        };
    }

    fn is_game_over(&self) -> bool {
        for row in 0..ROWS {
            for col in 0..COLS {
                if let (_, None) = self.grid[row][col] {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    println!("Welcome to the Filler game!");
    println!();

    let mut input = String::new();
    print!("Enter 'M' for manual grid input or 'R' for random grid: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    let random = match input.trim().to_ascii_uppercase().as_str() {
        "M" => false,
        "R" => true,
        _ => {
            println!("Invalid input. Random grid will be used.");
            true
        }
    };

    let mut game = Game::new(random);

    while !game.is_game_over() {
        game.print_grid();
        println!("It's player {}'s turn.", game.current_player);

        if game.current_player == 'X' {
            // Human player's turn
            let mut input = String::new();
            print!("Choose a color: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            let color = input.trim().chars().next().unwrap();

            //player 1 color is the bottom left corner
            //player 2 color is the top right corner
            let player1_color = game.grid[ROWS - 1][0].0;
            let player2_color = game.grid[0][COLS - 1].0;

            if color == player1_color || color == player2_color || !COLORS.contains(&color) {
                println!("Invalid color. Please try again.\n");
                continue;
            }

            game.make_move(color);
        } else {
            // Computer player's turn
            let mut max_eval = std::i64::MIN;
            let mut best_color = COLORS[0];

            for color in COLORS {
                if game.is_valid_move(*color) {
                    let mut cloned_game = game.clone();
                    cloned_game.make_move(*color);
                    let eval = minimax(&mut cloned_game, DEPTH, true);
                    if eval > max_eval {
                        max_eval = eval;
                        best_color = *color;
                    }
                }
            }

            game.make_move(best_color);
        }

        println!();
    }

    game.print_grid();
    let (player1_count, player2_count) = count_owned_cells(&game);

    if player1_count > player2_count {
        println!("Player X wins!");
    } else if player2_count > player1_count {
        println!("Player O wins!");
    } else {
        println!("It's a tie!");
    }
}

fn count_owned_cells(game: &Game) -> (usize, usize) {
    let mut player1_count = 0;
    let mut player2_count = 0;

    for row in 0..ROWS {
        for col in 0..COLS {
            if let (_, Some(player)) = game.grid[row][col] {
                if player == game.player1 {
                    player1_count += 1;
                } else if player == game.player2 {
                    player2_count += 1;
                }
            }
        }
    }

    (player1_count, player2_count)
}

fn minimax(game: &mut Game, depth: i32, maximizing_player: bool) -> i64 {
    if depth == 0 || game.is_game_over() {
        return evaluate(game);
    }

    let mut best_eval = if maximizing_player {
        std::i64::MIN
    } else {
        std::i64::MAX
    };

    for color in COLORS {
        if game.is_valid_move(*color) {
            let mut cloned_game = game.clone();
            cloned_game.make_move(*color);
            let eval = minimax(&mut cloned_game, depth - 1, !maximizing_player);

            if maximizing_player {
                best_eval = best_eval.max(eval);
            } else {
                best_eval = best_eval.min(eval);
            }
        }
    }

    best_eval
}

fn evaluate(game: &Game) -> i64 {
    let (player1_count, player2_count) = count_owned_cells(game);

    println!("Player X: {}", player1_count);
    println!("Player O: {}", player2_count);

    (player2_count as i64) - (player1_count as i64)
}
