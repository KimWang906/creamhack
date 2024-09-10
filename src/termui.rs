// use std::time::{Duration, Instant};

use crate::{
    custom_widgets::{button::*, state_list::*},
    dreamhack::{challenge::*, options::*, FromIndex, ToColorString, Variants},
};
use anyhow::Context;
use color_eyre::Result;
use crossterm::event;
use handle::*;
use palette::tailwind::*;
use ratatui::{
    crossterm::event::*, layout::*, style::*, symbols, text::Line, widgets::*, DefaultTerminal,
    Frame,
};

const CREAMHACK_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;

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

#[derive(PartialEq, Eq)]
pub struct App {
    should_exit: bool,
    is_popup_active: bool,
    // cursor_flag: bool,
    cursor_state: CursorState,
    challenges: StateList<Challenge>,
    options: Options,
    current_page: PageInfo,
    current_tab: Tabs,
    search: Input,
    enter_flag: Input,
    wargame_details_index: usize,
    create_vm_button: CreateVm,
}

impl Default for App {
    fn default() -> Self {
        Self {
            should_exit: false,
            is_popup_active: false,
            // cursor_flag: false,
            cursor_state: CursorState::Search,
            challenges: StateList {
                items: Vec::new(),
                state: ListState::default(),
            },
            options: Options {
                buttons: Vec::new(),
                buttons_index: 0,
                items: OptionsData {
                    cat: Category::All,
                    diff: Difficulty::All,
                    status: Status::All,
                    order: Orderings::Newist,
                },
                popup: OptionsPopup {
                    items: Vec::new(),
                    state: OptionsPopupState::None,
                },
            },
            current_page: PageInfo::default(),
            current_tab: Tabs::Search,
            search: Input::default(),
            enter_flag: Input::default(),
            wargame_details_index: 0,
            create_vm_button: CreateVm::default(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CursorState {
    // None,
    Search,
    EnterFlag,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Options {
    buttons: Vec<Button>,
    buttons_index: usize,
    items: OptionsData,
    popup: OptionsPopup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OptionsPopupState {
    None,
    CategoryPopup,
    DifficultyPopup,
    StatusPopup,
    OrderPopup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OptionsPopup {
    items: Vec<OptionInfo>,
    state: OptionsPopupState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OptionInfo {
    index: usize,
    size: usize,
}

impl Options {
    fn get_selected_index(&self) -> usize {
        self.buttons
            .iter()
            .position(|btn| btn.get_state() == ButtonState::Selected)
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
struct OptionsData {
    cat: Category,
    diff: Difficulty,
    status: Status,
    order: Orderings,
}

#[derive(Default, PartialEq, Eq)]
struct CreateVm {
    state: CreateVmState,
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord)]
enum CreateVmState {
    #[default]
    None,
    Selected,
}

#[derive(Default, PartialEq, Eq)]
struct Input {
    input: String,
    character_index: usize,
}

#[derive(Default, PartialEq, Eq)]
enum Tabs {
    #[default]
    Search,
    Options,
    WargameList,
    WargameDetails,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
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

        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            // if last_cursor_toggle.elapsed() >= Duration::from_millis(500) {
            //     self.toggle_cursor_blink();
            //     last_cursor_toggle = Instant::now();
            // }

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

        // tab switching
        if key.code == KeyCode::Tab {
            self.next_tab()
        }

        match self.current_tab {
            Tabs::Search => self.handle_search_input(key),
            Tabs::Options => self.handle_options_input(key),
            Tabs::WargameList => self.handle_wargame_list_input(key),
            Tabs::WargameDetails => self.handle_wargame_details_input(key),
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
        if self.is_popup_active {
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
                    self.is_popup_active = false;
                    self.options.popup.state = OptionsPopupState::None;
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.is_popup_active = false;
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
                    self.is_popup_active = true;
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
                if self.wargame_details_index < 1 {
                    self.wargame_details_index += 1;
                }
            }
            _ => {}
        }

        match self.wargame_details_index {
            0 => {
                self.create_vm_button.state = CreateVmState::None;
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
                self.create_vm_button.state = CreateVmState::Selected;
                if key.code == KeyCode::Enter {
                    todo!()
                }
            }
            _ => {}
        }
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

        if self.is_popup_active {
            self.render_popup(frame);
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

// impl App {
//     fn toggle_cursor_blink(&mut self) {
//         self.cursor_flag = !self.cursor_flag
//     }
// }

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

impl App {
    fn render_header(area: Rect, frame: &mut Frame) {
        Paragraph::new("CreamHack")
            .bold()
            .centered()
            .render(area, frame.buffer_mut());
    }

    fn render_footer(area: Rect, frame: &mut Frame) {
        Paragraph::new("Author: KimWang906")
            .style(Style::default().bold())
            .centered()
            .render(area, frame.buffer_mut());
    }

    fn render_search(&self, area: Rect, frame: &mut Frame) {
        let block = Block::default()
            .title(Line::raw("Search").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED);

        Paragraph::new(self.search.input.as_str())
            .block(block)
            .fg(TEXT_FG_COLOR)
            .render(area, frame.buffer_mut());

        if self.cursor_state == CursorState::Search {
            frame.set_cursor_position(Position::new(
                area.x + self.search.character_index as u16 + 1,
                area.y + 1,
            ));
        }
    }

    fn render_options(&mut self, area: Rect, frame: &mut Frame) {
        let options: [Rect; 4] = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .areas(area);

        for (i, &button) in self.options.buttons.iter().enumerate() {
            let block = Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::default().fg(if self.options.get_selected_index() == i {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                );
            let paragraph = Paragraph::new(button.label)
                .block(block)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, options[i]);
        }
    }

    fn render_popup(&mut self, frame: &mut Frame) {
        let popup_rect = popup_area(frame.area(), 50, 50);
        frame.render_widget(Clear, popup_rect);

        let block = Block::default()
            .title("Edit Option")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::DarkGray));
        frame.render_widget(block, popup_rect);

        match self.options.popup.state {
            OptionsPopupState::CategoryPopup => {
                self.popup_category(popup_rect, frame);
            }
            OptionsPopupState::DifficultyPopup => {
                self.popup_difficulty(popup_rect, frame);
            }
            OptionsPopupState::StatusPopup => {
                self.popup_status(popup_rect, frame);
            }
            OptionsPopupState::OrderPopup => {
                self.popup_order(popup_rect, frame);
            }
            _ => {}
        }
    }

    fn render_options_value(&mut self, area: Rect, frame: &mut Frame) {
        let values_area: [Rect; 4] = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .areas(area);

        let values = [
            Paragraph::new(self.options.items.cat.to_string())
                .style(Style::new().bold())
                .alignment(Alignment::Center),
            Paragraph::new(self.options.items.diff.to_color_string())
                .style(Style::new().bold())
                .alignment(Alignment::Center),
            Paragraph::new(self.options.items.status.to_string())
                .style(Style::new().bold())
                .alignment(Alignment::Center),
            Paragraph::new(self.options.items.order.to_string())
                .style(Style::new().bold())
                .alignment(Alignment::Center),
        ];

        for (i, value) in values.iter().enumerate() {
            frame.render_widget(value, values_area[i]);
        }
    }

    fn render_current_tab(&self, area: Rect, frame: &mut Frame) {
        let tab = match self.current_tab {
            Tabs::Search => "Search",
            Tabs::Options => "Options",
            Tabs::WargameList => "Wargame List",
            Tabs::WargameDetails => "Wargame Details",
        };

        let block = Block::default()
            .title(format!("Current Tab: {}", tab))
            .style(Style::default().bg(Color::DarkGray).bold())
            .title_alignment(Alignment::Center);
        frame.render_widget(block, area);
    }

    fn render_list(&mut self, area: Rect, frame: &mut Frame) {
        let block = Block::new()
            .title(Line::raw("Wargames").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(CREAMHACK_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        if let Some(selected_index) = self.challenges.state.selected() {
            if selected_index >= self.challenges.items.len() {
                self.challenges.select_none();
            }
        }

        let items: Vec<ListItem> = self
            .challenges
            .items
            .iter()
            .enumerate()
            .map(|(i, chall_item)| {
                let color = alternate_colors(i);
                ListItem::from(chall_item).bg(color)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // frame을 바로 전달
        frame.render_stateful_widget(list, area, &mut self.challenges.state);
    }

    fn render_selected_item(&self, area: Rect, frame: &mut Frame) {
        let [detail_area, enter_flag_area, create_vm_button_area, unused_area] =
            Layout::vertical([
                Constraint::Percentage(50),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .areas(area);

        let info = if let Some(i) = self.challenges.state.selected() {
            format!("{}", self.challenges.items[i].to_detailed_info())
        } else {
            "Nothing selected...".to_string()
        };

        let details_block = Block::new()
            .title(Line::raw("Details").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(CREAMHACK_HEADER_STYLE)
            .bg(NORMAL_ROW_BG)
            .padding(Padding::horizontal(1));

        Paragraph::new(info)
            .block(details_block)
            .fg(TEXT_FG_COLOR)
            .wrap(Wrap { trim: false })
            .render(detail_area, frame.buffer_mut());

        let enter_flag_block = Block::default()
            .title(Line::raw("Enter Flag").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED);

        Paragraph::new(self.enter_flag.input.as_str())
            .block(enter_flag_block)
            .fg(TEXT_FG_COLOR)
            .bg(NORMAL_ROW_BG)
            .render(enter_flag_area, frame.buffer_mut());

        if self.cursor_state == CursorState::EnterFlag {
            frame.set_cursor_position(Position::new(
                enter_flag_area.x + self.enter_flag.character_index as u16 + 1,
                enter_flag_area.y + 1,
            ));
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(
                if self.create_vm_button.state == CreateVmState::Selected {
                    Color::Yellow
                } else {
                    Color::White
                },
            ));
        let paragraph = Paragraph::new("Create VM")
            .block(block)
            .bg(NORMAL_ROW_BG)
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, create_vm_button_area);

        Paragraph::new("unimplemented...")
            .centered()
            .bg(NORMAL_ROW_BG)
            .render(unused_area, frame.buffer_mut())
    }
}

impl App {
    fn popup_category(&self, area: Rect, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                Category::variants()
                    .iter()
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<Constraint>>(),
            )
            .split(area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }));

        for (i, cat) in Category::variants().iter().enumerate() {
            let style = if Category::from_index(
                self.options
                    .popup
                    .items
                    .get(self.options.buttons_index)
                    .context("Failed to get the selected index")
                    .unwrap()
                    .index,
            ) == *cat
            {
                Style::default()
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let cat_str = cat.to_string();
            let paragraph = Paragraph::new(cat_str)
                .style(style)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, chunks[i]);
        }
    }

    fn popup_difficulty(&self, area: Rect, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                Difficulty::variants()
                    .iter()
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<Constraint>>(),
            )
            .split(area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }));

        for (i, diff) in Difficulty::variants().iter().enumerate() {
            let style = if Difficulty::from_index(
                self.options
                    .popup
                    .items
                    .get(self.options.buttons_index)
                    .context("Failed to get the difficulty selected index")
                    .unwrap()
                    .index,
            ) == *diff
            {
                Style::default()
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let cat_str = diff.to_color_string();
            let paragraph = Paragraph::new(cat_str)
                .style(style)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, chunks[i]);
        }
    }

    fn popup_status(&self, area: Rect, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                Status::variants()
                    .iter()
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<Constraint>>(),
            )
            .split(area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }));

        for (i, status) in Status::variants().iter().enumerate() {
            let style = if Status::from_index(
                self.options
                    .popup
                    .items
                    .get(self.options.buttons_index)
                    .context("Failed to get the status selected index")
                    .unwrap()
                    .index,
            ) == *status
            {
                Style::default()
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let cat_str = status.to_string();
            let paragraph = Paragraph::new(cat_str)
                .style(style)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, chunks[i]);
        }
    }

    fn popup_order(&self, area: Rect, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                Orderings::variants()
                    .iter()
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<Constraint>>(),
            )
            .split(area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }));

        for (i, order) in Orderings::variants().iter().enumerate() {
            let style = if Orderings::from_index(
                self.options
                    .popup
                    .items
                    .get(self.options.buttons_index)
                    .context("Failed to get the order selected index")
                    .unwrap()
                    .index,
            ) == *order
            {
                Style::default()
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let cat_str = order.to_string();
            let paragraph = Paragraph::new(cat_str)
                .style(style)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, chunks[i]);
        }
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl<'a> From<&'a Challenge> for ListItem<'a> {
    fn from(value: &'a Challenge) -> Self {
        let line = value.to_simple_info();
        ListItem::new(line)
    }
}

impl Input {
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
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

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
        self.input.clear();
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
}
