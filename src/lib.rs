use players::*;
use utilities::*;
use menu::*;
use std::collections::HashMap;
use normal_game::play_game;

type StateType = HashMap<String, Player>;
const COMPUTER_NAME_LENGTH_LIMIT: usize  = 10;
const TRAINING_FOLDER: &str  = r"training_regime";

pub fn run() {
    let mut state: StateType = HashMap::new();

    let leave_option = 
        MenuOption::<&mut StateType> {
            command: "l",
            description: "leave",
            action: MenuOptionAction::Leave
        };

    let options = 
        vec![
            MenuOption {
                command: "p",
                description: "play a game or train a machine",
                action: MenuOptionAction::SubMenu(
                    MenuSection {
                        options: 
                            vec![
                                MenuOption {
                                    command: "1",
                                    description: "one player game (vs computer)",
                                    action: MenuOptionAction::Callback(one_player_game)
                                },
                                MenuOption {
                                    command: "2",
                                    description: "two player game",
                                    action: MenuOptionAction::Callback(two_player_game)
                                },
                                MenuOption {
                                    command: "t",
                                    description: "train computers",
                                    action: MenuOptionAction::Callback(train_computers)
                                },
                                leave_option.clone()
                            ],
                            preamble_generator: player_hashmap_display
                    }
                )
            },
            MenuOption {
                command: "a",
                description: "add/remove players",
                action: MenuOptionAction::SubMenu(
                    MenuSection {
                        options: 
                            vec![
                                MenuOption {
                                    command: "r",
                                    description: "remove a player",
                                    action: MenuOptionAction::Callback(remove_a_player)
                                },
                                MenuOption {
                                    command: "s",
                                    description: "save a computer learner",
                                    action: MenuOptionAction::Callback(save_a_computer_player)
                                },
                                MenuOption {
                                    command: "a",
                                    description: "add a player",
                                    action: MenuOptionAction::SubMenu(
                                        MenuSection {
                                            options: 
                                                vec![
                                                    MenuOption {
                                                        command: "n",
                                                        description: "new computer learner",
                                                        action: MenuOptionAction::Callback(new_learner)
                                                    },
                                                    MenuOption {
                                                        command: "e",
                                                        description: "load a computer learner",
                                                        action: MenuOptionAction::Callback(load_learner)
                                                    },
                                                    MenuOption {
                                                        command: "p",
                                                        description: "load a computer player",
                                                        action: MenuOptionAction::Callback(load_comp_player)
                                                    },
                                                    leave_option.clone()
                                                ],
                                            preamble_generator: player_hashmap_display
                                        }
                                    )
                                },
                                MenuOption {
                                    command: "s",
                                    description: "toggle the learning status of a computer learner",
                                    action: MenuOptionAction::Callback(toggle_learning)
                                },
                                leave_option.clone()
                            ],
                            preamble_generator: player_hashmap_display
                    }
                )
                },
                leave_option.clone()
        ];
        
    let menu = MenuSection {
        options,
        preamble_generator: player_hashmap_display
    };

        show_menu(&menu, &mut state);
}
fn one_player_game(state: &mut StateType) -> &mut StateType {
    let mut person = Player::HumanPlayer(HumanPlayer::new("Enter your name:"));
    let opponent_name = get_user_input_line("Who do you want to play?");
    let opponent = state.get_mut(&opponent_name).unwrap();
    let switch_to_second = get_user_input_line("Do you want to go first? (Yes- press enter, No-Type anything and press enter)");
    if switch_to_second == "".to_string() {
        play_game(&mut person, opponent);
    } else {
        play_game(opponent, &mut person);
    }
    state
}

fn two_player_game(state: &mut StateType) -> &mut StateType {
    let mut player_one = Player::HumanPlayer(HumanPlayer::new("Player one name?"));
    let mut player_two = Player::HumanPlayer(HumanPlayer::new("Player two name?"));
    play_game(&mut player_one, &mut player_two);

    state
}

fn train_computers(state: &mut StateType) -> &mut StateType {
    let filename = get_user_input_line("Enter filename of training regime:");
    let repeats: u32 = get_user_input_line("How many times do you want to run it?").trim().parse().unwrap();
    let path = get_file_path(&filename, TRAINING_FOLDER);
    let regime_string = load_with_relative_path(path);
    let regime_vec: Vec<Vec<&str>> = regime_string.split('\n').into_iter().map(|x| x.split(' ').collect()).collect();

    for _ in 1..repeats {
        for instruction in (&regime_vec).into_iter() {
            // temporary unsafe, needed to get 2 mutable refs to value inside Hashmap
            unsafe {
                let player_one = state.get_mut(instruction[0].trim()).unwrap() as *mut _;
                let player_two = state.get_mut(instruction[1].trim()).unwrap() as *mut _;

                play_game(&mut *player_one, &mut *player_two)
            }
        }
    }

    state
}
    
fn new_learner(state: &mut StateType) -> &mut StateType {
    let name = get_a_valid_name();
    state.insert(name, Player::ComputerLearner(ComputerLearner::new(true)));

    state
}

fn load_learner(state: &mut StateType) -> &mut StateType {
    let name = get_a_valid_name();
    let filename = get_user_input_line("What file do you want to load from?");
    state.insert(name, Player::ComputerLearner(ComputerLearner::load(filename.as_str(), true)));

    state
}

fn load_comp_player(state: &mut StateType) -> &mut StateType {
    let name = get_a_valid_name();
    let filename = get_user_input_line("What file do you want to load from?");
    state.insert(name, Player::ComputerPlayer(ComputerPlayer::load(filename.as_str())));

    state
}

fn get_a_valid_name() -> String {
    let mut name = get_user_input_line("What do you want to call it? (Max 10 character)");

    while name.chars().count() > COMPUTER_NAME_LENGTH_LIMIT ||  name.chars().count() == 0 {
        name = get_user_input_line("That is invalid, please try again");
    };

    name
}

fn toggle_learning(state: &mut StateType) -> &mut StateType {
    let name = get_user_input_line("Which player would you like to toggle?");

    match state.get_mut(&name) {
        Some(player) => {
            match player {
                Player::HumanPlayer(_) => {}
                Player::ComputerLearner(l) => {l.toggle_is_learning()}
                Player::ComputerPlayer(_) => {}
            }
        }
        None => {}
    }
    
    state
}

fn save_a_computer_player(state: &mut StateType) -> &mut StateType {
    let name = get_user_input_line("What is the name of the player?");
    let filename = get_user_input_line("What file do you want to save to?");
    let player = state.get(&name).unwrap();
    match player {
        Player::HumanPlayer(_) => {}
        Player::ComputerLearner(l) => {l.save(&filename)}
        Player::ComputerPlayer(_) => {}
    }
    state
}

fn remove_a_player(state: &mut StateType) -> &mut StateType {
    let name = get_user_input_line("What is the name of the player?");
    state.remove(&name);
    state
}

fn player_hashmap_display<'a>(state: &'a mut StateType) -> (&'a mut StateType, String) {
    let mut display = "Currently the following machine are active in the system \n\n".to_string();

    for (key, value) in state.iter(){
        display.push_str(
            format!("{:>width$} : {}\n", key, value, width = COMPUTER_NAME_LENGTH_LIMIT + 5).as_str()
        )
    }
    (state, display)
}

mod normal_game;
mod players;
mod utilities;
mod shared_definitions;
mod menu;