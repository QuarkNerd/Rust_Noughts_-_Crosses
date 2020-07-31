
use crate::utilities::*;

pub enum menu_option_action<'a, T> {
    sub_menu(menu_section<'a, T>),
    callback(fn(state: T) -> T),
    leave
}

impl<'a, T> Clone for menu_option_action<'a, T> {
    fn clone(&self) -> Self {
        match self {
            menu_option_action::sub_menu(sub) => menu_option_action::sub_menu(sub.clone()),
            menu_option_action::callback(f) => menu_option_action::callback(*f),
            menu_option_action::leave => menu_option_action::leave,
        }   
    }
}

pub struct menu_option<'a, T> {
    pub command: &'a str,
    pub description: &'a str,
    pub action: menu_option_action<'a, T>,
}

impl<'a, T> Clone for menu_option<'a, T>{
    fn clone(&self) -> Self {
        menu_option {
            command: self.command.clone(),
            description: self.description.clone(),
            action: self.action.clone(),
        }
    }
}

pub struct menu_section<'a, T> {
    pub options: Vec<menu_option<'a, T>>,
    pub preamble_generator: fn(state: T) -> (T, String)
}

impl<'a, T> Clone for menu_section<'a, T> {
    fn clone(&self) -> Self {
        menu_section {
            options: self.options.to_vec(),
            preamble_generator: self.preamble_generator
        }
    }
}

pub fn show_menu<'a, T>(menu: &menu_section<'a, &'a mut T>, state: &'a mut T) -> &'a mut T{
    let mut state = state;
    let options = &menu.options;

    loop {
        let preamble_return = (menu.preamble_generator)(state);
        state = preamble_return.0;
        let mut prompt = preamble_return.1;
        prompt.push_str("\n\n");

        for menu_item in options.iter() {
            prompt.push_str(&format!("{}: {} \n", &menu_item.command, &menu_item.description));
        }

        let mut input = String::new();
        let mut selected_option: Option<&menu_option<&mut T>> = None;

        while selected_option.is_none() {
           input = get_user_input_line(&prompt);
           selected_option = options.iter().find(|&x| x.command == input);
        }

        match &selected_option.unwrap().action {
            menu_option_action::sub_menu(sub_menu) => state = show_menu(sub_menu, state),
            menu_option_action::callback(callback) => state = callback(state),
            menu_option_action::leave => break,
        }
    }

    return state
}
