use players::*;
use utilities::*;
use menu::*;
use std::collections::HashMap;

type StateType = HashMap<String, Player>;
const COMPUTER_NAME_LENGTH_LIMIT: usize  = 10;

pub fn run() {
    let mut state: StateType = HashMap::new();

    let leave_option = 
        menu_option::<&mut StateType> {
            command: "l",
            description: "leave",
            action: menu_option_action::leave
        };

    let options = 
        vec![
            menu_option {
                command: "p",
                description: "play a game or train a machine",
                action: menu_option_action::sub_menu(
                    menu_section {
                        options: 
                            vec![
                                menu_option {
                                    command: "t",
                                    description: "thing",
                                    action: menu_option_action::sub_menu(
                                        menu_section {
                                            options: 
                                                vec![
                                                    menu_option {
                                                        command: "t",
                                                        description: "thing",
                                                        action: menu_option_action::callback(remove_a_player)
                                                    },
                                                    leave_option.clone()
                                                ],
                                            preamble_generator: player_hashmap_display
                                        }
                                    )
                                },
                                leave_option.clone()
                            ],
                            preamble_generator: player_hashmap_display
                    }
                )
            },
            menu_option {
                command: "a",
                description: "add/remove players",
                action: menu_option_action::sub_menu(
                    menu_section {
                        options: 
                            vec![
                                menu_option {
                                    command: "r",
                                    description: "remove a player",
                                    action: menu_option_action::callback(remove_a_player)
                                },
                                menu_option {
                                    command: "s",
                                    description: "save a computer learner",
                                    action: menu_option_action::callback(save_a_computer_player)
                                },
                                menu_option {
                                    command: "a",
                                    description: "add a player",
                                    action: menu_option_action::sub_menu(
                                        menu_section {
                                            options: 
                                                vec![
                                                    menu_option {
                                                        command: "n",
                                                        description: "new computer learner",
                                                        action: menu_option_action::callback(new_learner)
                                                    },
                                                    menu_option {
                                                        command: "ll",
                                                        description: "load a computer learner",
                                                        action: menu_option_action::callback(load_learner)
                                                    },
                                                    menu_option {
                                                        command: "lp",
                                                        description: "load a computer player",
                                                        action: menu_option_action::callback(load_comp_player)
                                                    },
                                                    leave_option.clone()
                                                ],
                                            preamble_generator: player_hashmap_display
                                        }
                                    )
                                },
                                menu_option {
                                    command: "s",
                                    description: "toggle the learning status of a computer learner",
                                    action: menu_option_action::callback(toggle_learning)
                                },
                                leave_option.clone()
                            ],
                            preamble_generator: player_hashmap_display
                    }
                )
                },
                leave_option.clone()
        ];
        
    let menu = menu_section {
        options,
        preamble_generator: player_hashmap_display
    };

        show_menu(&menu, &mut state);
}
    
fn new_learner(state: &mut StateType) -> &mut StateType {
    let name = get_user_input_line("What do you want to call it?");
    state.insert(name, Player::ComputerLearner(ComputerLearner::new(true)));

    state
}

fn load_learner(state: &mut StateType) -> &mut StateType {
    let name = get_user_input_line("What do you want to call it?");
    let filename = get_user_input_line("What file do you want to load from?");
    state.insert(name, Player::ComputerLearner(ComputerLearner::load(filename.as_str(), true)));

    state
}

fn load_comp_player(state: &mut StateType) -> &mut StateType {
    let name = get_user_input_line("What do you want to call it?");
    let filename = get_user_input_line("What file do you want to load from?");
    state.insert(name, Player::ComputerPlayer(ComputerPlayer::load(filename.as_str())));

    state
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