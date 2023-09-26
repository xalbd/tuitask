use crate::{
    app::{App, AppReturn},
    key::Key,
};

use super::TextBox;

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    let name_text = &mut app.name_edit;

    match key {
        Key::Number(c) | Key::Char(c) => {
            let mut proposed_text = name_text.text.clone();
            proposed_text.insert(name_text.index, c);

            if proposed_text.len() <= name_text.max_length {
                *name_text = TextBox {
                    text: proposed_text,
                    index: name_text.index + 1,
                    ..*name_text
                };
            }
        }
        Key::Left => {
            if name_text.index > 0 {
                name_text.index -= 1;
            }
        }
        Key::Right => {
            if name_text.index < name_text.text.len() {
                name_text.index += 1;
            }
        }
        Key::Backspace => {
            if name_text.index > 0 {
                name_text.text.remove(name_text.index - 1);
                name_text.index -= 1;
            }
        }
        _ => {}
    }

    AppReturn::Continue
}

pub fn initialize(app: &mut App) -> AppReturn {
    let starting_name = if app.editing_category {
        app.categories[app.category_list_state.selected().unwrap()]
            .name
            .to_string()
    } else {
        "".to_string()
    };

    app.name_edit = TextBox {
        index: starting_name.len(),
        text: starting_name,
        ..app.name_edit
    };

    AppReturn::Continue
}
