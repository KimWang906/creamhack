#[cfg(debug_assertions)]
use std::sync::Once;
use std::{env, fs::File, io::Write, path::PathBuf};

use crate::{
    custom_widgets::{button::*, input::Input, popup::PopupItem, state_list::*},
    dreamhack::{auth::Auth, challenge::*, options::*, vm_info::MachineInfo},
    fs_tree::build_tree,
    utils,
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
    pub(crate) config: Option<crate::Config>,
    pub(crate) should_exit: bool,
    pub(crate) events: Events,
    pub(crate) auth: Auth,
    pub(crate) ui_state: UIState,
    pub(crate) fs_state: FileSystemState,
    pub(crate) vm_state: VMState,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Events {
    pub(crate) mouse: Option<MouseEvent>,
}

pub(crate) struct UIState {
    pub(crate) popup_state: PopupState,
    pub(crate) cursor_state: CursorState,
    pub(crate) challenges: StateList<Challenge>,
    pub(crate) options: Options,
    pub(crate) current_page: PageInfo,
    pub(crate) current_tab: Tabs,
    pub(crate) search: Input,
    pub(crate) enter_flag: Input,
    pub(crate) wargame_details_index: usize,
}

pub(crate) struct FileSystemState {
    pub(crate) workdir: PathBuf,
    pub(crate) tree_state: TreeState<String>,
    pub(crate) tree_items: Vec<TreeItem<'static, String>>,
}

pub(crate) struct VMState {
    pub(crate) vm_info: MachineInfo,
}

impl Default for App {
    fn default() -> Self {
        Self {
            config: None,
            events: Events::default(),
            should_exit: false,
            auth: Auth::default(),
            ui_state: UIState {
                popup_state: PopupState::None,
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
            },
            fs_state: FileSystemState {
                workdir: env::current_dir()
                    .context("Failed to get current directory")
                    .unwrap(),
                tree_state: TreeState::default(),
                tree_items: Vec::new(),
            },
            vm_state: VMState {
                vm_info: MachineInfo::default(),
            },
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

#[derive(Default, PartialEq, Eq, Debug)]
pub(crate) enum Tabs {
    #[default]
    Search,
    Options,
    WargameList,
    WargameDetails,
}

impl Tabs {
    pub fn set_tab(&mut self, tab: Tabs) {
        *self = tab;
    }
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

    pub(crate) fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub(crate) fn get_size(&self) -> usize {
        self.size
    }
}

impl OptionsPopup {
    pub(crate) fn get_items(&self) -> &Vec<OptionInfo> {
        &self.items
    }

    pub(crate) fn get_mut_items(&mut self) -> &mut Vec<OptionInfo> {
        &mut self.items
    }

    pub(crate) fn get_state(&self) -> OptionsPopupState {
        self.state
    }

    pub(crate) fn set_state(&mut self, state: OptionsPopupState) {
        self.state = state;
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

    pub(crate) fn get_mut_buttons(&mut self) -> &mut Vec<Button> {
        &mut self.buttons
    }

    pub(crate) fn clear_button_state(&mut self) {
        for btn in self.buttons.iter_mut() {
            btn.set_state(ButtonState::Normal);
        }
    }

    pub(crate) fn get_buttons_index(&self) -> usize {
        self.buttons_index
    }

    pub(crate) fn set_buttons_index(&mut self, index: usize) {
        self.buttons_index = index;
    }

    pub(crate) fn get_items(&self) -> &OptionsData {
        &self.items
    }

    pub(crate) fn get_popup(&self) -> &OptionsPopup {
        &self.popup
    }

    pub(crate) fn get_mut_popup(&mut self) -> &mut OptionsPopup {
        &mut self.popup
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
        terminal: &mut DefaultTerminal,
        config: crate::Config,
        email_entry: Entry,
        password_entry: Entry,
    ) -> Result<()> {
        // let mut last_cursor_toggle = Instant::now();
        let mut request = RequestChallengeList::new();
        (self.ui_state.challenges.items, self.ui_state.current_page) =
            request.send_request().unwrap_or_default();
        self.ui_state.options = Options {
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

        self.config = Some(config);
        self.fs_state.tree_items = build_tree(&self.fs_state.workdir)
            .context("Failed to build tree")
            .unwrap();

        let email = String::from_utf8_lossy(&email_entry.get_secret().unwrap()).into_owned();
        let password = password_entry.get_password().unwrap();
        self.auth = Auth::send_login(&email, &password, false).unwrap();

        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;

            match event::read()? {
                Event::Key(key) => {
                    match (key.code, key.modifiers) {
                        (KeyCode::Tab, _) => self.next_tab(),
                        (KeyCode::Char('w'), KeyModifiers::CONTROL)
                            if self.ui_state.popup_state == PopupState::None =>
                        {
                            self.ui_state.popup_state = PopupState::FsTreeView;
                        }
                        _ => {}
                    }
                    self.handle_key(key)
                }
                Event::Mouse(mouse) if self.config.as_ref().unwrap().experimental_features => {
                    self.events.mouse = Some(mouse);
                }
                _ => {}
            }
        }
        Ok(())
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

        #[cfg(debug_assertions)]
        {
            static AREAS_LOGGING: Once = Once::new();
            AREAS_LOGGING.call_once(|| {
                log::info!("Area: {:?}", area);
                log::info!("Header: {:?}", header_area);
                log::info!("Search: {:?}", search_area);
                log::info!("Current tab: {:?}", current_tab);
                log::info!("Options: {:?}", options_area);
                log::info!("Show options: {:?}", show_options);
                log::info!("Main: {:?}", main_area);
                log::info!("Footer: {:?}", footer_area);
                log::info!("List: {:?}", list_area);
                log::info!("Item: {:?}", item_area);
            });
        }

        App::render_header(header_area, frame);
        App::render_footer(footer_area, frame);
        self.render_search(search_area, frame);
        self.render_options(options_area, frame);
        self.render_options_value(show_options, frame);
        self.render_current_tab(current_tab, frame);
        self.render_list(list_area, frame);
        self.render_selected_item(item_area, frame);

        match self.ui_state.popup_state {
            PopupState::Options => self.render_options_popup(frame),
            PopupState::FsTreeView => self.render_fs_tree_view_popup(frame),
            PopupState::None => {}
        }
    }
}

impl App {
    pub(crate) fn next_page(&mut self) {
        self.ui_state.current_page.next_page();

        let mut request = RequestChallengeList::new();
        request.set_page(self.ui_state.current_page.get_page_idx());
        (self.ui_state.challenges.items, self.ui_state.current_page) =
            request.send_request().unwrap();
    }

    pub(crate) fn previous_page(&mut self) {
        self.ui_state.current_page.previous_page();

        let mut request = RequestChallengeList::new();
        request.set_page(self.ui_state.current_page.get_page_idx());
        (self.ui_state.challenges.items, self.ui_state.current_page) =
            request.send_request().unwrap();
    }
}

impl App {
    fn next_tab(&mut self) {
        match self.ui_state.current_tab {
            Tabs::Search => self.ui_state.current_tab = Tabs::Options,
            Tabs::Options => self.ui_state.current_tab = Tabs::WargameList,
            Tabs::WargameList => self.ui_state.current_tab = Tabs::WargameDetails,
            Tabs::WargameDetails => self.ui_state.current_tab = Tabs::Search,
        }

        #[cfg(debug_assertions)]
        log::info!("Current tab: {:?}", self.ui_state.current_tab);
    }
}

impl App {
    pub(crate) fn apply_popup_selection(&mut self) {
        match self.ui_state.options.popup.state {
            OptionsPopupState::CategoryPopup => {
                self.ui_state.options.items.cat = Category::from_index(
                    self.ui_state.options.popup.items[self.ui_state.options.buttons_index].index,
                )
            }
            OptionsPopupState::DifficultyPopup => {
                self.ui_state.options.items.diff = Difficulty::from_index(
                    self.ui_state.options.popup.items[self.ui_state.options.buttons_index].index,
                )
            }
            OptionsPopupState::StatusPopup => {
                self.ui_state.options.items.status = Status::from_index(
                    self.ui_state.options.popup.items[self.ui_state.options.buttons_index].index,
                )
            }
            OptionsPopupState::OrderPopup => {
                self.ui_state.options.items.order = Orderings::from_index(
                    self.ui_state.options.popup.items[self.ui_state.options.buttons_index].index,
                )
            }
            OptionsPopupState::None => {}
        }
    }

    pub(crate) fn get_popup_items_length(&self) -> usize {
        match self.ui_state.options.popup.state {
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
    pub(crate) fn start_search(&mut self) {
        let mut request = RequestChallengeList::new();
        request.set_search(self.ui_state.search.input.clone());
        request.set_category(self.ui_state.options.items.cat);
        request.set_difficulty(self.ui_state.options.items.diff);
        request.set_status(self.ui_state.options.items.status);
        request.set_ordering(self.ui_state.options.items.order);

        (self.ui_state.challenges.items, self.ui_state.current_page) = request
            .send_request()
            .context("Failed to send request")
            .unwrap();

        self.ui_state.search.reset_cursor();
    }

    fn start_download(&mut self, path: &str) {
        if let Some(selected_index) = self.ui_state.challenges.state.selected() {
            let challenge = &self.ui_state.challenges.items[selected_index];
            let challenge_data = challenge.download_challenge();
            let f = File::create_new(PathBuf::from(path));

            match f {
                Ok(mut file) => {
                    file.write_all(challenge_data.as_slice())
                        .context("Failed to write file")
                        .unwrap();
                }
                #[allow(unused_variables)]
                Err(e) => {
                    #[cfg(debug_assertions)]
                    log::error!("Failed to create file: {:?}", e);
                }
            }
        }
    }

    pub(crate) fn handle_download_file(&mut self) {
        if let Some(selected_item) = self.ui_state.challenges.state.selected() {
            let workdir = self
                .fs_state
                .workdir
                .to_str()
                .context("Failed to get workdir")
                .unwrap()
                .to_owned();

            let repository = self.ui_state.challenges.items[selected_item]
                .get_metadata()
                .get_repository()
                .to_owned();

            let file_path = format!("{}/{}.zip", workdir, repository);

            self.start_download(&file_path);

            if self.config.as_ref().unwrap().extract_chall_file {
                utils::file_extractor::extract_file(
                    PathBuf::from(&file_path),
                    PathBuf::from(&workdir),
                    &repository,
                )
                .unwrap();
            }

            if !self.config.as_ref().unwrap().keep_chall_file {
                std::fs::remove_file(file_path)
                    .context("Failed to remove file")
                    .unwrap();
            }
        }
    }

    pub(crate) fn handle_create_vm(&mut self) {
        if let Some(selected_item) = self.ui_state.challenges.state.selected() {
            if self.ui_state.challenges.items[selected_item].create_vm(&self.auth) {
                // VM created
                self.vm_state.vm_info =
                    self.ui_state.challenges.items[selected_item].get_vm_info(&self.auth);
            }
        }
    }
}
