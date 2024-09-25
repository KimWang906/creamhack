/// This module contains the event handler for the application
/// But experimental features
mod mouse {
    use crossterm::event::{MouseEvent, MouseEventKind};
    use ratatui::layout::{Position, Rect};

    use crate::{custom_widgets::button::ButtonState, render::WARGAME_BLOCK_SIZE, termui::*};

    impl App {
        fn get_current_selected_index(&self, event: MouseEvent, area: Rect) -> usize {
            let offset = self.ui_state.challenges.state.offset();
            let index = (((event.row - area.y) as usize) / WARGAME_BLOCK_SIZE) + offset;

            index
        }

        pub(crate) fn handle_search_mouse_event(&mut self, event: MouseEvent, area: Rect) {
            if self.ui_state.popup_state != PopupState::None {
                return;
            }

            match event.kind {
                MouseEventKind::Down(_)
                    if area.contains(Position::new(event.column, event.row)) =>
                {
                    // Handle click event
                    #[cfg(debug_assertions)]
                    log::info!("Mouse clicked on Search tab");
                    self.ui_state.current_tab.set_tab(Tabs::Search);
                }
                _ => {}
            }
        }

        pub(crate) fn handle_options_mouse_event(
            &mut self,
            event: MouseEvent,
            area: Rect,
            option_index: usize,
        ) {
            if self.ui_state.popup_state != PopupState::None {
                return;
            }

            match event.kind {
                MouseEventKind::Down(_)
                    if area.contains(Position::new(event.column, event.row)) =>
                {
                    self.ui_state.current_tab.set_tab(Tabs::Options);
                    self.ui_state.options.clear_button_state();

                    let buttons = self.ui_state.options.get_mut_buttons();
                    buttons[option_index].set_state(ButtonState::Selected);

                    self.ui_state.options.set_buttons_index(option_index);

                    match option_index {
                        0 => {
                            #[cfg(debug_assertions)]
                            log::info!("Mouse clicked on button {}", option_index);
                            self.ui_state
                                .options
                                .get_mut_popup()
                                .set_state(OptionsPopupState::CategoryPopup);
                        }
                        1 => {
                            #[cfg(debug_assertions)]
                            log::info!("Mouse clicked on button {}", option_index);
                            self.ui_state
                                .options
                                .get_mut_popup()
                                .set_state(OptionsPopupState::DifficultyPopup);
                        }
                        2 => {
                            #[cfg(debug_assertions)]
                            log::info!("Mouse clicked on button {}", option_index);
                            self.ui_state
                                .options
                                .get_mut_popup()
                                .set_state(OptionsPopupState::StatusPopup);
                        }
                        3 => {
                            #[cfg(debug_assertions)]
                            log::info!("Mouse clicked on button {}", option_index);
                            self.ui_state
                                .options
                                .get_mut_popup()
                                .set_state(OptionsPopupState::OrderPopup);
                        }
                        _ => {}
                    }
                    self.ui_state.popup_state = PopupState::Options
                }
                _ => {}
            }
        }

        pub(crate) fn handle_wargames_mouse_event(&mut self, event: MouseEvent, area: Rect) {
            if self.ui_state.popup_state != PopupState::None {
                return;
            }

            match event.kind {
                MouseEventKind::ScrollUp => {
                    self.ui_state.current_tab.set_tab(Tabs::WargameList);
                    self.ui_state.challenges.select_previous();
                }
                MouseEventKind::ScrollDown => {
                    self.ui_state.current_tab.set_tab(Tabs::WargameList);
                    self.ui_state.challenges.select_next();
                }
                MouseEventKind::Down(_)
                    if area.contains(Position::new(event.column, event.row)) =>
                {
                    self.ui_state.current_tab.set_tab(Tabs::WargameList);
                    let offset = self.ui_state.challenges.state.offset();
                    #[cfg(debug_assertions)]
                    {
                        log::info!("Current Mouse X: {}, Y: {}", event.column, event.row);
                        log::info!("Current Offset: {}", offset);
                    }

                    //
                    let index = self.get_current_selected_index(event, area);
                    self.ui_state.challenges.state.select(Some(index));
                    log::info!("Selected index: {}", index);
                }
                _ => {}
            }
        }

        pub(crate) fn handle_enter_flag_mouse_event(&mut self, event: MouseEvent, area: Rect) {
            if self.ui_state.popup_state != PopupState::None {
                return;
            }

            match event.kind {
                MouseEventKind::Down(_)
                    if area.contains(Position::new(event.column, event.row)) =>
                {
                    #[cfg(debug_assertions)]
                    log::info!("Mouse clicked on Enter Flag Input");
                    self.ui_state.current_tab.set_tab(Tabs::WargameDetails);
                    self.ui_state.wargame_details_index = 0;
                }
                _ => {}
            }
        }

        pub(crate) fn handle_challenge_features_mouse_event(
            &mut self,
            event: MouseEvent,
            area: Rect,
            button_index: usize,
        ) {
            if self.ui_state.popup_state != PopupState::None {
                return;
            }

            match event.kind {
                MouseEventKind::Down(_)
                    if area.contains(Position::new(event.column, event.row)) =>
                {
                    self.ui_state.current_tab.set_tab(Tabs::WargameDetails);
                    self.ui_state.wargame_details_index = button_index;
                    match self.ui_state.wargame_details_index {
                        1 => {
                            #[cfg(debug_assertions)]
                            log::info!("Mouse clicked on Download Challenges Button");
                            self.handle_download_file();
                        }
                        2 => {
                            #[cfg(debug_assertions)]
                            log::info!("Mouse clicked on Create VM Button");
                            self.handle_create_vm();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}

mod keyboard {
    use std::{
        ops::{AddAssign, SubAssign},
        path::PathBuf,
    };

    use anyhow::Context;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

    use crate::{
        custom_widgets::button::ButtonState,
        fs_tree::build_tree,
        termui::{App, CursorState, OptionsPopupState, PopupState, Tabs},
    };

    impl App {
        pub fn handle_key(&mut self, key: KeyEvent) {
            if key.kind != KeyEventKind::Press {
                return;
            }

            match self.ui_state.popup_state {
                PopupState::None => match self.ui_state.current_tab {
                    Tabs::Search => self.handle_search_input(key),
                    Tabs::Options => self.handle_options_input(key),
                    Tabs::WargameList => self.handle_wargame_list_input(key),
                    Tabs::WargameDetails => self.handle_wargame_details_input(key),
                },
                PopupState::FsTreeView => {
                    self.handle_fs_tree_popup_input(key);
                }
                PopupState::Options => {
                    self.handle_options_input(key);
                }
            }

            #[cfg(debug_assertions)]
            log::info!("Key pressed: {:?}", key);
        }

        fn handle_search_input(&mut self, key: KeyEvent) {
            #[cfg(debug_assertions)]
            log::info!("Handle search input");
            self.ui_state.cursor_state = CursorState::Search;
            match key.code {
                KeyCode::Char('q') => self.should_exit = true,
                KeyCode::Enter => self.start_search(),
                KeyCode::Char(to_insert) => self.ui_state.search.enter_char(to_insert),
                KeyCode::Backspace => self.ui_state.search.delete_char(),
                KeyCode::Left => self.ui_state.search.move_cursor_left(),
                KeyCode::Right => self.ui_state.search.move_cursor_right(),
                _ => {}
            }
        }

        fn handle_options_input(&mut self, key: KeyEvent) {
            let buttons_index = self.ui_state.options.get_buttons_index();
            if self.ui_state.popup_state == PopupState::Options {
                let popup_items_len = self.get_popup_items_length();
                let items = self.ui_state.options.get_mut_popup().get_mut_items();
                #[cfg(debug_assertions)]
                log::info!("Handle options popup input");
                match key.code {
                    KeyCode::Up => {
                        if items[buttons_index].get_index() > 0 {
                            items[buttons_index].get_index().sub_assign(1);
                        }
                    }
                    KeyCode::Down => {
                        if items[buttons_index].get_index() < popup_items_len - 1 {
                            items[buttons_index].get_index().add_assign(1);
                        }
                    }
                    KeyCode::Enter => {
                        self.apply_popup_selection();
                        self.ui_state.popup_state = PopupState::None;
                        self.ui_state
                            .options
                            .get_mut_popup()
                            .set_state(OptionsPopupState::None);
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        self.ui_state.popup_state = PopupState::None;
                        self.ui_state
                            .options
                            .get_mut_popup()
                            .set_state(OptionsPopupState::None);
                    }
                    _ => {
                        #[cfg(debug_assertions)]
                        log::info!("Unhandled key in popup: {:?}", key.code);
                    }
                }
            } else {
                let buttons = self.ui_state.options.get_mut_buttons();
                #[cfg(debug_assertions)]
                log::info!("Handle options input");

                match key.code {
                    KeyCode::Char('q') => self.ui_state.popup_state = PopupState::None,
                    KeyCode::Left => {
                        if buttons_index > 0 {
                            buttons[buttons_index].set_state(ButtonState::Normal);
                            buttons[buttons_index].set_state(ButtonState::Selected);
                            self.ui_state.options.get_buttons_index().sub_assign(1);
                        }
                    }
                    KeyCode::Right => {
                        if buttons_index < buttons.len() - 1 {
                            buttons[buttons_index].set_state(ButtonState::Normal);
                            buttons[buttons_index].set_state(ButtonState::Selected);
                            self.ui_state.options.get_buttons_index().add_assign(1);
                        }
                    }
                    KeyCode::Enter => {
                        match buttons_index {
                            0 => self
                                .ui_state
                                .options
                                .get_mut_popup()
                                .set_state(OptionsPopupState::CategoryPopup),
                            1 => self
                                .ui_state
                                .options
                                .get_mut_popup()
                                .set_state(OptionsPopupState::DifficultyPopup),
                            2 => self
                                .ui_state
                                .options
                                .get_mut_popup()
                                .set_state(OptionsPopupState::StatusPopup),
                            3 => self
                                .ui_state
                                .options
                                .get_mut_popup()
                                .set_state(OptionsPopupState::OrderPopup),
                            _ => {}
                        }
                        self.ui_state.popup_state = PopupState::Options;
                    }
                    _ => {}
                }
            }
        }

        fn handle_wargame_list_input(&mut self, key: KeyEvent) {
            #[cfg(debug_assertions)]
            log::info!("Handle wargame list input");
            match key.code {
                KeyCode::Char('q') => self.should_exit = true,
                KeyCode::Char('h') | KeyCode::Esc => self.ui_state.challenges.select_none(),
                KeyCode::Char('j') | KeyCode::Down => self.ui_state.challenges.select_next(),
                KeyCode::Char('k') | KeyCode::Up => self.ui_state.challenges.select_previous(),
                KeyCode::Char('g') | KeyCode::Home => self.ui_state.challenges.select_first(),
                KeyCode::Char('G') | KeyCode::End => self.ui_state.challenges.select_last(),
                KeyCode::Char('l') | KeyCode::Right => self.next_page(),
                KeyCode::Char('u') | KeyCode::Left => self.previous_page(),
                _ => {}
            }
        }

        fn handle_wargame_details_input(&mut self, key: KeyEvent) {
            #[cfg(debug_assertions)]
            log::info!("Handle wargame details input");
            match key.code {
                KeyCode::Char('q') => self.should_exit = true,
                KeyCode::Char('k') | KeyCode::Up => {
                    if self.ui_state.wargame_details_index > 0 {
                        self.ui_state.wargame_details_index -= 1;
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if self.ui_state.wargame_details_index < 2 {
                        self.ui_state.wargame_details_index += 1;
                    }
                }
                _ => {}
            }

            match self.ui_state.wargame_details_index {
                0 => {
                    self.ui_state.cursor_state = CursorState::EnterFlag;
                    match key.code {
                        KeyCode::Enter => {}
                        KeyCode::Char(to_insert) => self.ui_state.enter_flag.enter_char(to_insert),
                        KeyCode::Backspace => self.ui_state.enter_flag.delete_char(),
                        KeyCode::Left => self.ui_state.enter_flag.move_cursor_left(),
                        KeyCode::Right => self.ui_state.enter_flag.move_cursor_right(),
                        _ => {}
                    }
                }
                1 => {
                    if key.code == KeyCode::Enter {
                        #[cfg(debug_assertions)]
                        log::info!(
                            "Selected item: {:?}",
                            self.ui_state.challenges.state.selected()
                        );

                        self.handle_download_file();
                    }
                }
                2 => {
                    if key.code == KeyCode::Enter {
                        self.handle_create_vm();
                    }
                }
                _ => {}
            }
        }

        fn handle_fs_tree_popup_input(&mut self, key: KeyEvent) {
            #[cfg(debug_assertions)]
            log::info!("Handle fs tree popup input");
            match key.code {
                KeyCode::Char('q') => {
                    self.ui_state.popup_state = PopupState::None;
                    false
                }
                KeyCode::Char('\n' | ' ') => self.fs_state.tree_state.toggle_selected(),
                KeyCode::Enter => {
                    let selected_workdir = self
                        .fs_state
                        .tree_state
                        .selected()
                        .first()
                        .context("Failed to get selected item")
                        .unwrap();

                    #[cfg(debug_assertions)]
                    log::info!("Selected workdir: {}", selected_workdir);

                    self.fs_state.workdir = PathBuf::from(selected_workdir);
                    self.fs_state.tree_items = build_tree(&self.fs_state.workdir)
                        .context("Failed to build tree")
                        .unwrap();
                    self.ui_state.popup_state = PopupState::None;
                    true
                }
                KeyCode::Left => self.fs_state.tree_state.key_left(),
                KeyCode::Right => self.fs_state.tree_state.key_right(),
                KeyCode::Down => self.fs_state.tree_state.key_down(),
                KeyCode::Up => self.fs_state.tree_state.key_up(),
                KeyCode::Esc => self.fs_state.tree_state.select(Vec::new()),
                KeyCode::Home => self.fs_state.tree_state.select_first(),
                KeyCode::End => self.fs_state.tree_state.select_last(),
                KeyCode::PageDown => self.fs_state.tree_state.scroll_down(3),
                KeyCode::PageUp => self.fs_state.tree_state.scroll_up(3),
                _ => false,
            };
        }
    }
}
