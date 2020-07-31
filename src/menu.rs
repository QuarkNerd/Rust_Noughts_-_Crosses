
use crate::utilities::*;

pub enum MenuOptionAction<'a, T> {
    SubMenu(MenuSection<'a, T>),
    Callback(fn(state: T) -> T),
    Leave
}

impl<'a, T> Clone for MenuOptionAction<'a, T> {
    fn clone(&self) -> Self {
        match self {
            MenuOptionAction::SubMenu(sub) => MenuOptionAction::SubMenu(sub.clone()),
            MenuOptionAction::Callback(f) => MenuOptionAction::Callback(*f),
            MenuOptionAction::Leave => MenuOptionAction::Leave,
        }   
    }
}

pub struct MenuOption<'a, T> {
    pub command: &'a str,
    pub description: &'a str,
    pub action: MenuOptionAction<'a, T>,
}

impl<'a, T> Clone for MenuOption<'a, T>{
    fn clone(&self) -> Self {
        MenuOption {
            command: self.command.clone(),
            description: self.description.clone(),
            action: self.action.clone(),
        }
    }
}

pub struct MenuSection<'a, T> {
    pub options: Vec<MenuOption<'a, T>>,
    pub preamble_generator: fn(state: T) -> (T, String)
}

impl<'a, T> Clone for MenuSection<'a, T> {
    fn clone(&self) -> Self {
        MenuSection {
            options: self.options.to_vec(),
            preamble_generator: self.preamble_generator
        }
    }
}

pub fn show_menu<'a, T>(menu: &MenuSection<'a, &'a mut T>, state: &'a mut T) -> &'a mut T{
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

        let mut input;
        let mut selected_option: Option<&MenuOption<&mut T>> = None;

        while selected_option.is_none() {
           input = get_user_input_line(&prompt);
           selected_option = options.iter().find(|&x| x.command == input);
        }

        match &selected_option.unwrap().action {
            MenuOptionAction::SubMenu(sub_menu) => state = show_menu(sub_menu, state),
            MenuOptionAction::Callback(callback) => state = callback(state),
            MenuOptionAction::Leave => break,
        }
    }

    return state
}
