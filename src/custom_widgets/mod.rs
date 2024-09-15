#![allow(dead_code)]

pub mod button {

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
    pub enum ButtonState {
        Normal,
        Selected,
        Active,
    }

    #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Button {
        pub(crate) label: &'static str,
        pub(crate) state: ButtonState,
    }

    impl Button {
        pub fn get_state(&self) -> ButtonState {
            self.state
        }

        pub fn set_state(&mut self, state: ButtonState) {
            self.state = state;
        }
    }
}

pub mod state_list {
    use ratatui::widgets::ListState;

    #[derive(Debug, PartialEq, Eq)]
    pub struct StateList<T: Sized> {
        pub(crate) items: Vec<T>,
        pub(crate) state: ListState,
    }

    impl<T> StateList<T> {
        pub fn select_none(&mut self) {
            self.state.select(None);
        }

        pub fn select_next(&mut self) {
            self.state.select_next();
        }
        pub fn select_previous(&mut self) {
            self.state.select_previous();
        }

        pub fn select_first(&mut self) {
            self.state.select_first();
        }

        pub fn select_last(&mut self) {
            self.state.select_last();
        }
    }

    impl<T> Default for StateList<T> {
        fn default() -> Self {
            Self {
                items: Vec::new(),
                state: ListState::default(),
            }
        }
    }
}

pub mod input {
    #[derive(Default, PartialEq, Eq)]
    pub struct Input {
        pub input: String,
        character_index: usize,
    }

    impl Input {
        pub fn move_cursor_left(&mut self) {
            let cursor_moved_left = self.character_index.saturating_sub(1);
            self.character_index = self.clamp_cursor(cursor_moved_left);
        }

        pub fn move_cursor_right(&mut self) {
            let cursor_moved_right = self.character_index.saturating_add(1);
            self.character_index = self.clamp_cursor(cursor_moved_right);
        }

        pub fn enter_char(&mut self, new_char: char) {
            let index = self.byte_index();
            self.input.insert(index, new_char);
            self.move_cursor_right();
        }

        /// Returns the byte index based on the character position.
        ///
        /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
        /// the byte index based on the index of the character.
        pub fn byte_index(&self) -> usize {
            self.input
                .char_indices()
                .map(|(i, _)| i)
                .nth(self.character_index)
                .unwrap_or(self.input.len())
        }

        pub fn delete_char(&mut self) {
            let is_not_cursor_leftmost = self.character_index != 0;
            if is_not_cursor_leftmost {
                // Method "remove" is not used on the saved text for deleting the selected char.
                // Reason: Using remove on String works on bytes instead of the chars.
                // Using remove would require special care because of char boundaries.

                let current_index = self.character_index;
                let from_left_to_current_index = current_index - 1;

                // Getting all characters before the selected character.
                let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
                // Getting all characters after selected character.
                let after_char_to_delete = self.input.chars().skip(current_index);

                // Put all characters together except the selected one.
                // By leaving the selected one out, it is forgotten and therefore deleted.
                self.input = before_char_to_delete.chain(after_char_to_delete).collect();
                self.move_cursor_left();
            }
        }

        pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
            new_cursor_pos.clamp(0, self.input.chars().count())
        }

        pub fn reset_cursor(&mut self) {
            self.character_index = 0;
            self.input.clear();
        }

        pub fn get_character_index(&self) -> usize {
            self.character_index
        }

        pub fn set_character_index(&self) -> usize {
            self.character_index
        }
    }
}

pub mod popup {
    use ratatui::{layout::*, Frame};

    /// helper function to create a centered rect using up certain percentage of the available rect `r`
    pub fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    pub trait PopupItem: ToString {
        fn variants() -> Vec<Self>
        where
            Self: Sized;
        fn from_index(index: usize) -> Self
        where
            Self: Sized;
    }

    pub trait PopupOptions {
        fn popup_options<T>(&self, area: Rect, frame: &mut Frame)
        where
            T: PopupItem + PartialEq;
    }
}
