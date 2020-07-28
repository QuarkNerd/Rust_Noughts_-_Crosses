
use crate::utilities::*;
pub enum menu_option_action<'a, T> {
    sub_menu(Vec<menu_option<'a, T>>),
    callback(fn(state: T) -> T),
}
pub struct menu_option<'a, T> {
    pub command: &'a str,
    pub description: &'a str,
    pub action: menu_option_action<'a, T>,
}

pub fn show_menu<'a, T>(options: &Vec<menu_option<&'a mut T>>, state: &'a mut T) -> &'a mut T{
    let mut state = state;
    let mut prompt = "Please select from the following options \n\n".to_string();

    for menu_item in options.iter() {
        prompt.push_str(&format!("{}: {} \n", &menu_item.command, &menu_item.description));
    }

    let mut input = String::new();
    let mut selected_option: Option<&menu_option<&mut T>> = None;

    while selected_option.is_none() {
        input = get_user_input(&prompt);
        selected_option = options.iter().find(|&x| x.command == input);
    }

    match &selected_option.unwrap().action {
        menu_option_action::sub_menu(sub_menu) => state = show_menu(&sub_menu, state),
        menu_option_action::callback(callback) => state = callback(state),
    }

    return state
}
