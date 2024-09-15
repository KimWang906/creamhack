use std::{env, fs::File, io::Write, path::PathBuf};

use crate::{
    custom_widgets::{button::*, input::Input, popup::PopupItem, state_list::*},
    dreamhack::{auth::Auth, challenge::*, options::*, vm_info::MachineInfo},
    fs_tree::build_tree,
};
use anyhow::Context;
use color_eyre::Result;
use crossterm::event;
use handle::*;
use keyring::Entry;
use palette::tailwind::*;
use ratatui::{crossterm::event::*, layout::*, style::*, widgets::*, DefaultTerminal, Frame};
use tui_tree_widget::{TreeItem, TreeState};

pub(crate) const CREAMHACK_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
pub(crate) const NORMAL_ROW_BG: Color = SLATE.c950;
pub(crate) const ALT_ROW_BG_COLOR: Color = SLATE.c900;
pub(crate) const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
pub(crate) const TEXT_FG_COLOR: Color = SLATE.c200;

const OPTIONS: ([Button; 4], OptionsData, usize) = (
    [
        Button {
            label: "Category",
            state: ButtonState::Normal,
        },
        Button {
            label: "Difficulty",
            state: ButtonState::Normal,
        },
        Button {
            label: "Status",
            state: ButtonState::Normal,
        },
        Button {
            label: "Order",
            state: ButtonState::Normal,
        },
    ],
    OptionsData {
        cat: Category::All,
        diff: Difficulty::All,
        status: Status::All,
        order: Orderings::Newist,
    },
    0,
);

pub(crate) struct App {
    pub(crate) should_exit: bool,
    pub(crate) auth: Auth,
    pub(crate) popup_state: PopupState,
    pub(crate) cursor_state: CursorState,
    pub(crate) challenges: StateList<Challenge>,
    pub(crate) options: Options,
    pub(crate) current_page: PageInfo,
    pub(crate) current_tab: Tabs,
    pub(crate) search: Input,
    pub(crate) enter_flag: Input,
    pub(crate) wargame_details_index: usize,
    pub(crate) workdir: PathBuf,
    pub(crate) fs_tree_state: TreeState<String>,
    pub(crate) fs_tree_items: Vec<TreeItem<'static, String>>,
    pub(crate) vm_info: MachineInfo,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            auth: Auth::default(),
            popup_state: PopupState::None,
            // cursor_flag: false,
            cursor_state: CursorState::Search,
            challenges: StateList {
                items: Vec::new(),
                state: ListState::default(),
            },
            options: Options::default(),
            current_page: PageInfo::default(),
            current_tab: Tabs::Search,
            search: Input::default(),
            enter_flag: Input::default(),
            wargame_details_index: 0,
            workdir: env::current_dir()
                .context("Failed to get current directory")
                .unwrap(),
            fs_tree_state: TreeState::default(),
            fs_tree_items: Vec::new(),
            vm_info: MachineInfo::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Options {
    buttons: Vec<Button>,
    buttons_index: usize,
    items: OptionsData,
    popup: OptionsPopup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OptionsPopupState {
    None,
    CategoryPopup,
    DifficultyPopup,
    StatusPopup,
    OrderPopup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OptionsPopup {
    items: Vec<OptionInfo>,
    state: OptionsPopupState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OptionInfo {
    index: usize,
    size: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub(crate) struct OptionsData {
    cat: Category,
    diff: Difficulty,
    status: Status,
    order: Orderings,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CursorState {
    // None,
    Search,
    EnterFlag,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PopupState {
    None,
    Options,
    FsTreeView,
}

#[derive(Default, PartialEq, Eq)]
pub(crate) enum Tabs {
    #[default]
    Search,
    Options,
    WargameList,
    WargameDetails,
}

impl Default for OptionsData {
    fn default() -> Self {
        Self {
            cat: Category::All,
            diff: Difficulty::All,
            status: Status::All,
            order: Orderings::Newist,
        }
    }
}

impl OptionInfo {
    #![allow(dead_code)]

    pub(crate) fn get_index(&self) -> usize {
        self.index
    }

    pub(crate) fn get_size(&self) -> usize {
        self.size
    }
}

impl OptionsPopup {
    pub(crate) fn get_items(&self) -> &Vec<OptionInfo> {
        &self.items
    }

    pub(crate) fn get_state(&self) -> OptionsPopupState {
        self.state
    }
}

impl OptionsData {
    pub(crate) fn get_category(&self) -> &Category {
        &self.cat
    }

    pub(crate) fn get_difficulty(&self) -> &Difficulty {
        &self.diff
    }

    pub(crate) fn get_status(&self) -> &Status {
        &self.status
    }

    pub(crate) fn get_order(&self) -> &Orderings {
        &self.order
    }
}

impl Options {
    pub(crate) fn get_buttons(&self) -> &Vec<Button> {
        &self.buttons
    }

    pub(crate) fn get_buttons_index(&self) -> usize {
        self.buttons_index
    }

    pub(crate) fn get_items(&self) -> &OptionsData {
        &self.items
    }

    pub(crate) fn get_popup(&self) -> &OptionsPopup {
        &self.popup
    }

    pub(crate) fn get_selected_index(&self) -> usize {
        self.buttons
            .iter()
            .position(|btn| btn.get_state() == ButtonState::Selected)
            .unwrap_or(0)
    }
}

impl Default for Options {
    fn default() -> Self {
        Self {
            buttons: Vec::new(),
            buttons_index: 0,
            items: OptionsData::default(),
            popup: OptionsPopup {
                items: Vec::new(),
                state: OptionsPopupState::None,
            },
        }
    }
}

impl App {
    pub fn run(
        mut self,
        mut terminal: DefaultTerminal,
        email_entry: Entry,
        password_entry: Entry,
    ) -> Result<()> {
        // let mut last_cursor_toggle = Instant::now();
        let mut request = RequestChallengeList::new();
        (self.challenges.items, self.current_page) = request.send_request().unwrap_or_default();
        self.options = Options {
            buttons: OPTIONS.0.into_iter().collect(),
            buttons_index: OPTIONS.2,
            items: OPTIONS.1,
            popup: OptionsPopup {
                items: vec![
                    OptionInfo {
                        index: 0,
                        size: Category::variants().len(),
                    },
                    OptionInfo {
                        index: 0,
                        size: Difficulty::variants().len(),
                    },
                    OptionInfo {
                        index: 0,
                        size: Status::variants().len(),
                    },
                    OptionInfo {
                        index: 0,
                        size: Orderings::variants().len(),
                    },
                ],
                state: OptionsPopupState::None,
            },
        };

        self.fs_tree_items = build_tree(&self.workdir)
            .context("Failed to build tree")
            .unwrap();

        let email = String::from_utf8_lossy(&email_entry.get_secret().unwrap()).into_owned();
        let password = password_entry.get_password().unwrap();
        self.auth = Auth::send_login(&email, &password, false).unwrap();

        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            keycode if keycode == KeyCode::Char('w') && key.modifiers == KeyModifiers::CONTROL => {
                self.popup_state = PopupState::FsTreeView;
            }
            KeyCode::Tab => {
                self.next_tab();
            }
            _ => {}
        }

        if self.popup_state == PopupState::None {
            match self.current_tab {
                Tabs::Search => self.handle_search_input(key),
                Tabs::Options => self.handle_options_input(key),
                Tabs::WargameList => self.handle_wargame_list_input(key),
                Tabs::WargameDetails => self.handle_wargame_details_input(key),
            }
        } else if self.popup_state == PopupState::FsTreeView {
            self.handle_fs_tree_popup_input(key);
        }
    }

    fn handle_search_input(&mut self, key: KeyEvent) {
        self.cursor_state = CursorState::Search;
        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Enter => self.start_search(),
            KeyCode::Char(to_insert) => self.search.enter_char(to_insert),
            KeyCode::Backspace => self.search.delete_char(),
            KeyCode::Left => self.search.move_cursor_left(),
            KeyCode::Right => self.search.move_cursor_right(),
            _ => {}
        }
    }

    fn handle_options_input(&mut self, key: KeyEvent) {
        if self.popup_state == PopupState::Options {
            match key.code {
                KeyCode::Up => {
                    if self.options.popup.items[self.options.buttons_index].index > 0 {
                        self.options.popup.items[self.options.buttons_index].index -= 1;
                    }
                }
                KeyCode::Down => {
                    if self.options.popup.items[self.options.buttons_index].index
                        < self.get_popup_items_length() - 1
                    {
                        self.options.popup.items[self.options.buttons_index].index += 1;
                    }
                }
                KeyCode::Enter => {
                    self.apply_popup_selection();
                    self.popup_state = PopupState::None;
                    self.options.popup.state = OptionsPopupState::None;
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.popup_state = PopupState::None;
                    self.options.popup.state = OptionsPopupState::None;
                }
                _ => {}
            }
        } else {
            match key.code {
                KeyCode::Char('q') => self.should_exit = true,
                KeyCode::Left => {
                    if self.options.buttons_index > 0 {
                        self.options.buttons[self.options.buttons_index]
                            .set_state(ButtonState::Normal);
                        self.options.buttons_index -= 1;
                        self.options.buttons[self.options.buttons_index]
                            .set_state(ButtonState::Selected);
                    }
                }
                KeyCode::Right => {
                    if self.options.buttons_index < self.options.buttons.len() - 1 {
                        self.options.buttons[self.options.buttons_index]
                            .set_state(ButtonState::Normal);
                        self.options.buttons_index += 1;
                        self.options.buttons[self.options.buttons_index]
                            .set_state(ButtonState::Selected);
                    }
                }
                KeyCode::Enter => {
                    match self.options.buttons_index {
                        0 => self.options.popup.state = OptionsPopupState::CategoryPopup,
                        1 => self.options.popup.state = OptionsPopupState::DifficultyPopup,
                        2 => self.options.popup.state = OptionsPopupState::StatusPopup,
                        3 => self.options.popup.state = OptionsPopupState::OrderPopup,
                        _ => {}
                    }
                    self.popup_state = PopupState::Options;
                }
                _ => {}
            }
        }
    }

    fn handle_wargame_list_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Char('h') | KeyCode::Esc => self.challenges.select_none(),
            KeyCode::Char('j') | KeyCode::Down => self.challenges.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.challenges.select_previous(),
            KeyCode::Char('g') | KeyCode::Home => self.challenges.select_first(),
            KeyCode::Char('G') | KeyCode::End => self.challenges.select_last(),
            KeyCode::Char('l') | KeyCode::Right => self.next_page(),
            KeyCode::Char('u') | KeyCode::Left => self.previous_page(),
            _ => {}
        }
    }

    fn handle_wargame_details_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.should_exit = true,
            KeyCode::Char('k') | KeyCode::Up => {
                if self.wargame_details_index > 0 {
                    self.wargame_details_index -= 1;
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if self.wargame_details_index < 2 {
                    self.wargame_details_index += 1;
                }
            }
            _ => {}
        }

        match self.wargame_details_index {
            0 => {
                self.cursor_state = CursorState::EnterFlag;
                match key.code {
                    KeyCode::Enter => {}
                    KeyCode::Char(to_insert) => self.enter_flag.enter_char(to_insert),
                    KeyCode::Backspace => self.enter_flag.delete_char(),
                    KeyCode::Left => self.enter_flag.move_cursor_left(),
                    KeyCode::Right => self.enter_flag.move_cursor_right(),
                    _ => {}
                }
            }
            1 => {
                if key.code == KeyCode::Enter {
                    if let Some(selected_item) = self.challenges.state.selected() {
                        self.start_download(&format!(
                            "{}/{}.zip",
                            self.workdir
                                .to_str()
                                .context("Failed to get workdir")
                                .unwrap(),
                            self.challenges.items[selected_item]
                                .get_metadata()
                                .get_repository()
                        ));
                    }
                }
            }
            2 => {
                if key.code == KeyCode::Enter {
                    if let Some(selected_item) = self.challenges.state.selected() {
                        if self.challenges.items[selected_item].create_vm(&self.auth) {
                            // VM created
                            self.vm_info =
                                self.challenges.items[selected_item].get_vm_info(&self.auth);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_fs_tree_popup_input(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                self.popup_state = PopupState::None;
                false
            }
            KeyCode::Char('\n' | ' ') => self.fs_tree_state.toggle_selected(),
            KeyCode::Enter => {
                let selected_workdir = self
                    .fs_tree_state
                    .selected()
                    .first()
                    .context("Failed to get selected item")
                    .unwrap();

                self.workdir = PathBuf::from(selected_workdir);
                self.fs_tree_items = build_tree(&self.workdir)
                    .context("Failed to build tree")
                    .unwrap();
                self.popup_state = PopupState::None;
                true
            }
            KeyCode::Left => self.fs_tree_state.key_left(),
            KeyCode::Right => self.fs_tree_state.key_right(),
            KeyCode::Down => self.fs_tree_state.key_down(),
            KeyCode::Up => self.fs_tree_state.key_up(),
            KeyCode::Esc => self.fs_tree_state.select(Vec::new()),
            KeyCode::Home => self.fs_tree_state.select_first(),
            KeyCode::End => self.fs_tree_state.select_last(),
            KeyCode::PageDown => self.fs_tree_state.scroll_down(3),
            KeyCode::PageUp => self.fs_tree_state.scroll_up(3),
            _ => false,
        };
    }
}

impl App {
    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [header_area, search_area, current_tab, options_area, show_options, main_area, footer_area] =
            Layout::vertical([
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(1),
            ])
            .areas(area);

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(main_area);

        App::render_header(header_area, frame);
        App::render_footer(footer_area, frame);
        self.render_search(search_area, frame);
        self.render_options(options_area, frame);
        self.render_options_value(show_options, frame);
        self.render_current_tab(current_tab, frame);
        self.render_list(list_area, frame);
        self.render_selected_item(item_area, frame);

        match self.popup_state {
            PopupState::Options => self.render_options_popup(frame),
            PopupState::FsTreeView => self.render_fs_tree_view_popup(frame),
            PopupState::None => {}
        }
    }
}

impl App {
    fn next_page(&mut self) {
        self.current_page.next_page();

        let mut request = RequestChallengeList::new();
        request.set_page(self.current_page.get_page_idx());
        (self.challenges.items, self.current_page) = request.send_request().unwrap();
    }

    fn previous_page(&mut self) {
        self.current_page.previous_page();

        let mut request = RequestChallengeList::new();
        request.set_page(self.current_page.get_page_idx());
        (self.challenges.items, self.current_page) = request.send_request().unwrap();
    }
}

impl App {
    fn next_tab(&mut self) {
        match self.current_tab {
            Tabs::Search => self.current_tab = Tabs::Options,
            Tabs::Options => self.current_tab = Tabs::WargameList,
            Tabs::WargameList => self.current_tab = Tabs::WargameDetails,
            Tabs::WargameDetails => self.current_tab = Tabs::Search,
        }
    }
}

impl App {
    fn apply_popup_selection(&mut self) {
        match self.options.popup.state {
            OptionsPopupState::CategoryPopup => {
                self.options.items.cat =
                    Category::from_index(self.options.popup.items[self.options.buttons_index].index)
            }
            OptionsPopupState::DifficultyPopup => {
                self.options.items.diff = Difficulty::from_index(
                    self.options.popup.items[self.options.buttons_index].index,
                )
            }
            OptionsPopupState::StatusPopup => {
                self.options.items.status =
                    Status::from_index(self.options.popup.items[self.options.buttons_index].index)
            }
            OptionsPopupState::OrderPopup => {
                self.options.items.order = Orderings::from_index(
                    self.options.popup.items[self.options.buttons_index].index,
                )
            }
            OptionsPopupState::None => {}
        }
    }

    fn get_popup_items_length(&self) -> usize {
        match self.options.popup.state {
            OptionsPopupState::CategoryPopup => Category::variants().len(),
            OptionsPopupState::DifficultyPopup => Difficulty::variants().len(),
            OptionsPopupState::StatusPopup => Status::variants().len(),
            OptionsPopupState::OrderPopup => Orderings::variants().len(),
            OptionsPopupState::None => 0,
        }
    }
}

impl<'a> From<&'a Challenge> for ListItem<'a> {
    fn from(value: &'a Challenge) -> Self {
        let line = value.to_simple_info();
        ListItem::new(line)
    }
}

impl App {
    fn start_search(&mut self) {
        let mut request = RequestChallengeList::new();
        request.set_search(self.search.input.clone());
        request.set_category(self.options.items.cat);
        request.set_difficulty(self.options.items.diff);
        request.set_status(self.options.items.status);
        request.set_ordering(self.options.items.order);

        (self.challenges.items, self.current_page) = request
            .send_request()
            .context("Failed to send request")
            .unwrap();

        self.search.reset_cursor();
    }

    fn start_download(&mut self, path: &str) {
        if let Some(selected_index) = self.challenges.state.selected() {
            let challenge = &self.challenges.items[selected_index];
            let challenge_data = challenge.download_challenge();
            let f = File::create_new(PathBuf::from(path));

            match f {
                Ok(mut file) => {
                    file.write_all(challenge_data.as_slice())
                        .context("Failed to write file")
                        .unwrap();
                }
                Err(_e) => {
                    // Error handling needed
                    // dbg!(e);
                }
            }
        }
    }
}