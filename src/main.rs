use std::{io};
fn main() {
    println!("Welcome to your adventure");
    println!("-------------------------");

    let mut game_state = GameState {
        has_cell_key: false,
        have_weapon: false,
        inv: Vec::new()
    };

    start_scene(&mut game_state);
}

struct GameState {
    has_cell_key:bool,
    have_weapon: bool,
    inv: Vec<String>
}
// need fuction that will give us an action enum from player input
enum PlayerAction{
    North,
    South,
    East,
    West,
    Action,
    Inv,
    Quit,
    BadAction
}

fn get_player_action() -> PlayerAction {
    println!("-------------------------");
    println!("What is your action?");

    let mut player_action = String::new();

    io::stdin()
        .read_line(&mut player_action)
        .expect("Not a valid input");

    println!("-------------------------");

    match player_action.trim() {
        "n" => PlayerAction::North,
        "s" => PlayerAction::South,
        "e" => PlayerAction::East,
        "w" => PlayerAction::West,
        "a" => PlayerAction::Action,
        "inv" => PlayerAction::Inv,
        "q" => PlayerAction::Quit,
        _ => PlayerAction::BadAction 
    }
}

fn start_scene(state: &mut GameState){

    // Description of room
    println!("You start in a cold dark cell.");

    if !state.has_cell_key {
        println!("You see a key on the floor.");
    }

    // Description of actions
    if !state.has_cell_key {
        println!("There is a door to the south but the cell door is locked.")
    } else {
        println!("You can go South.");
    }

    // scene logic loop
    loop {
        let action = get_player_action();

        match action {
            PlayerAction::South => {
                if !state.has_cell_key {
                    println!("The cell is locked, you cannot reach the door.");
                } else {
                    println!("You go through the southern door.");
                    second_scene(state);
                    break;
                }
            },
            PlayerAction::Action => {
                if !state.has_cell_key {
                    println!("You pick up the cell key.");
                    state.has_cell_key = true;
                    state.inv.push("cell key".to_string());
                } else {
                    println!("There is nothing to do in this room.");
                }
            },
            PlayerAction::Inv => {
                for item in &state.inv {
                    println!("{item}");
                }
            }
            PlayerAction::Quit => {
                println!("Goodbye");
                break;
            },
            _ => {
                println!("You cannot do that.");
            }            
        }
    }
}

fn second_scene(state: &mut GameState) {
    println!("This is the next room");

    if !state.have_weapon {
        print!("There is a dagger on the floor.");
    }

    println!("You can only go north");


    loop {
        let action = get_player_action();
        
        match action {
            PlayerAction::North => {
                start_scene(state);
                break;
            },
            PlayerAction::Action => {
                if state.have_weapon != true {
                    println!("You pick up the weapon");
                    state.have_weapon = true;
                    state.inv.push("dagger".to_string());
                }
                else {
                    println!("There is nothing to interact with.");
                }
            },
            PlayerAction::Inv => {
                for item in &state.inv {
                    println!("{item}");
                }
            },
            PlayerAction::Quit => {
                break;
            },
            _ => {
                println!("Not a valid action");
                
            }
        }
    }

}
