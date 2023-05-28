mod console;

use std::io;
use std::num::ParseIntError;
use std::ops::RangeInclusive;

fn num_error() -> String {
    String::from("Expected a number between 1 and 9.")
}

#[derive(PartialEq, Debug)]
enum PlayerInputError {
    OutOfRange,
    ParseInt(ParseIntError),
    ReadLineError,
    AlreadySelected,
}

impl PlayerInputError {
    fn from_parseint(err: ParseIntError) -> PlayerInputError {
        PlayerInputError::ParseInt(err)
    }
}

fn draw(game: &Game) {
    let v = &game.board;
    print!("{}", console::term::clear_screen());
    println!("-------------");
    println!("| {} | {} | {} |", v[0], v[1], v[2]);
    println!("|---|---|---|");

    println!("| {} | {} | {} |", v[3], v[4], v[5]);

    println!("|---|---|---|");
    println!("| {} | {} | {} |", v[6], v[7], v[8]);
    println!("-------------");
    println!("\n{}", game.round_message);
}

const NUM_RANGE: RangeInclusive<usize> = 1..=9;

fn player_input() -> Result<usize, PlayerInputError>{
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    let err = stdin.read_line(&mut buffer);

    match err {
        Ok(_) => (),
        Err(_) => return Err(PlayerInputError::ReadLineError),
    }

    let input = buffer
        .trim()
        .parse::<usize>().map_err(PlayerInputError::from_parseint)?;
    if !NUM_RANGE.contains(&input) {
        Err(PlayerInputError::OutOfRange)
    } else {
        Ok(input-1)
    }
}

fn has_winner(values: &[&str]) -> bool {
    let winner_position = vec![
        vec![0,1,2],
        vec![3,4,5],
        vec![6,7,8],
        vec![0,3,6],
        vec![1,4,7],
        vec![2,5,8],
        vec![0,4,8],
        vec![2,4,6],
    ];

    for x in winner_position {
        if values[x[0]] == values[x[1]] && values[x[0]] == values[x[2]] {
            return true;
        }
    }
    false
}

fn switch_player(player: &str) -> &str {
    if player == "X" { "O"} else { "X"}
}

fn update_board<'a>(num: usize, player: &'a str, values: &[&'a str]) -> Result<Vec<&'a str>, PlayerInputError> {
    let mut current_map = values.to_owned();
    match current_map[num] {
        "X" | "O" => Err(PlayerInputError::AlreadySelected),
        _ => {

            current_map[num] = player;
            Ok(current_map)
        },
    }
}

fn play_round(game: Game) -> Game {
    let input = player_input();
    if input.is_err() {
        return Game{
            round_message: num_error(),
            ..game
        };
    }
    let new_values = update_board(input.unwrap(), game.next_player, &game.board);
    if let Err(PlayerInputError::AlreadySelected) = new_values {
        return Game{
            round_message: format!("Already selected. Still player {} turn", game.next_player),
            ..game
        };
    } else if let Ok(values) = new_values {
        if has_winner(&values) {
            return Game{
                board: values,
                winner: true,
                round_message: format!("Player {} wins!", game.next_player),
                ..game
            };
        } else {
            let player = switch_player(game.next_player);
            return Game{
                board: values,
                round_message: format!("Player {} turn", player),
                next_player: player,
                ..game
            };
        }
    };
    game
}

#[derive(Debug)]
struct Game<'a> {
    board: Vec<&'a str>,
    round_message: String,
    round: i32,
    winner: bool,
    next_player: &'a str,
}

impl Game<'_> {
    fn new() -> Game<'static> {
        let starting_player = "X";
        Game{
            board: vec!["1","2","3","4","5","6","7","8","9"],
            round_message: format!("Player {} turn.", starting_player),
            round: 0,
            winner: false,
            next_player: starting_player,
        }
    }
}

fn run(game: Game) {
    let round_result = play_round(game);

    if round_result.round >= 9 {
        draw(&Game{
            round_message: "Players tie".to_string(),
            ..round_result
        });
    } else if round_result.winner {
        draw(&round_result);
    } else {
        draw(&round_result);
        run(round_result)
    }
}

fn main() {
    let game = Game::new();
    draw(&game);
    run(game);
}

