pub mod button {
    #![allow(dead_code)]

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
