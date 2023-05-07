use std::{io};

// game data structures
struct GameState {
    has_cell_key: bool,
    have_weapon: bool,
    inv: Vec<PlayerItem>
}

impl Default for GameState {
    fn default() -> GameState {
        GameState { 
            has_cell_key: false, 
            have_weapon: false, 
            inv: Vec::new() }
    }
}


enum PlayerAction{
    North,
    South,
    East,
    West,
    Action,
    Inv,
    Help,
    Quit,
    BadAction
}

#[derive(PartialEq)]
enum PlayerItem {
    CellKey,
    Dagger
}

impl std::fmt::Display for PlayerItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PlayerItem::CellKey => write!(f, "Cell Key"),
            PlayerItem::Dagger => write!(f, "Dagger")
        }
    }
}


// main
fn main() {
    println!("Welcome to your adventure");
    println!("-------------------------");
    println!("{} and {}", PlayerItem::Dagger, PlayerItem::CellKey);

    let mut game_state = GameState {
        ..Default::default()
    };

    start_scene(&mut game_state);
}

// helper functions
fn get_player_action() -> PlayerAction {
    println!("-------------------------");
    println!("What is your action?");

    let mut player_action = String::new();

    io::stdin()
        .read_line(&mut player_action)
        .expect("Not a valid input");

    println!("-------------------------");


    match player_action.trim() {
        "n" | "north" => PlayerAction::North,
        "s" | "south" => PlayerAction::South,
        "e" | "east" => PlayerAction::East,
        "w" | "west" => PlayerAction::West,
        "a" | "action" => PlayerAction::Action,
        "i" | "inv" => PlayerAction::Inv,
        "h" | "help" | "?" => PlayerAction::Help,
        "q" | "quit" => PlayerAction::Quit,
        _ => PlayerAction::BadAction 
    }
}

fn list_commands() {
    println!("List of commands {:?}", ["n", "s", "e", "w", "a", "inv", "q"]);
}

fn list_inventory(state: &mut GameState) {
    for item in &state.inv {
        println!("{item}");
    }
}

fn check_in_inv(inv: &Vec<PlayerItem>, item: &PlayerItem) -> bool{
    inv.contains(item)
}

fn do_common_action(action: PlayerAction, state: &mut GameState) -> bool {
    match action {
        PlayerAction::Inv => {
            list_inventory(state);
            return false
        } ,
        PlayerAction::Help => {
            list_commands();
            return false
        },
        PlayerAction::Quit => {
            println!("Goodbye");
            return true
        }
        _ => {
            println!("You cannot do that.");
            return false
        } 
    }
}

// scenes
// how do we reduce copied code for player actions?

fn start_scene(state: &mut GameState){

    // Description of room
    println!("You start in a cold dark cell.");

    if !state.has_cell_key {
        println!("You see a key on the floor.");
    }

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
                let test = check_in_inv(&state.inv, &PlayerItem::CellKey);
                println!("You have the key {test}");
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
                    state.inv.push(PlayerItem::CellKey);
                } else {
                    println!("There is nothing to do in this room.");
                }
            },
            _ => {
                if do_common_action(action, state) {
                    break;
                }
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
                    state.inv.push(PlayerItem::Dagger);
                }
                else {
                    println!("There is nothing to interact with.");
                }
            },
            _ => {
                 if do_common_action(action, state) {
                    break;
                 }             
            }
        }
    }

}
