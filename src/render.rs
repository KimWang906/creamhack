use ratatui::{
    layout::*,
    style::*,
    symbols,
    text::{Line, Text},
    widgets::*,
    Frame,
};
use tui_tree_widget::Tree;

use crate::{
    custom_widgets::popup::*,
    dreamhack::{challenge::handle::ToDetailedInfo, options::*, ToColorString},
    termui::*,
};

pub(crate) const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}

impl App {
    pub(crate) fn render_header(area: Rect, frame: &mut Frame) {
        Paragraph::new("CreamHack")
            .bold()
            .centered()
            .render(area, frame.buffer_mut());
    }

    pub(crate) fn render_footer(area: Rect, frame: &mut Frame) {
        Paragraph::new("Author: KimWang906")
            .style(Style::default().bold())
            .centered()
            .render(area, frame.buffer_mut());
    }

    pub(crate) fn render_search(&self, area: Rect, frame: &mut Frame) {
        let block = Block::default()
            .title(Line::raw("Search").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::ROUNDED);

        Paragraph::new(self.ui_state.search.input.as_str())
            .block(block)
            .fg(TEXT_FG_COLOR)
            .render(area, frame.buffer_mut());

        if self.ui_state.cursor_state == CursorState::Search {
            frame.set_cursor_position(Position::new(
                area.x + self.ui_state.search.get_character_index() as u16 + 1,
                area.y + 1,
            ));
        }
    }

    pub(crate) fn render_options(&mut self, area: Rect, frame: &mut Frame) {
        let options: [Rect; 4] = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .areas(area);

        for (i, &button) in self.ui_state.options.get_buttons().iter().enumerate() {
            let block = Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::default().fg(if self.ui_state.options.get_selected_index() == i {
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

    pub(crate) fn render_options_popup(&mut self, frame: &mut Frame) {
        let popup_rect = popup_area(frame.area(), 50, 50);
        frame.render_widget(Clear, popup_rect);

        let block = Block::default()
            .title("Edit Option")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::DarkGray));
        frame.render_widget(block, popup_rect);

        match self.ui_state.options.get_popup().get_state() {
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

    pub(crate) fn render_options_value(&mut self, area: Rect, frame: &mut Frame) {
        let values_area: [Rect; 4] = Layout::horizontal([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .areas(area);

        let items = self.ui_state.options.get_items();
        let values = [
            Paragraph::new(items.get_category().to_string())
                .style(Style::new().bold())
                .alignment(Alignment::Center),
            Paragraph::new(items.get_difficulty().to_color_string())
                .style(Style::new().bold())
                .alignment(Alignment::Center),
            Paragraph::new(items.get_status().to_string())
                .style(Style::new().bold())
                .alignment(Alignment::Center),
            Paragraph::new(items.get_order().to_string())
                .style(Style::new().bold())
                .alignment(Alignment::Center),
        ];

        for (i, value) in values.iter().enumerate() {
            frame.render_widget(value, values_area[i]);
        }
    }

    pub(crate) fn render_current_tab(&self, area: Rect, frame: &mut Frame) {
        let tab = match self.ui_state.current_tab {
            crate::termui::Tabs::Search => "Search",
            crate::termui::Tabs::Options => "Options",
            crate::termui::Tabs::WargameList => "Wargame List",
            crate::termui::Tabs::WargameDetails => "Wargame Details",
        };

        let block = Block::default()
            .title(format!("Current Tab: {}", tab))
            .style(Style::default().bg(Color::DarkGray).bold())
            .title_alignment(Alignment::Center);
        frame.render_widget(block, area);
    }

    pub(crate) fn render_list(&mut self, area: Rect, frame: &mut Frame) {
        let block = Block::new()
            .title(Line::raw("Wargames").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(CREAMHACK_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        if let Some(selected_index) = self.ui_state.challenges.state.selected() {
            if selected_index >= self.ui_state.challenges.items.len() {
                self.ui_state.challenges.select_none();
            }
        }

        let items: Vec<ListItem> = self
            .ui_state
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
        frame.render_stateful_widget(list, area, &mut self.ui_state.challenges.state);
    }

    pub(crate) fn render_selected_item(&self, area: Rect, frame: &mut Frame) {
        let [detail_area, enter_flag_area, buttons_area, vm_info_area, unused_area] =
            Layout::vertical([
                Constraint::Percentage(50),
                Constraint::Length(3),
                Constraint::Length(6),
                Constraint::Length(2),
                Constraint::Fill(1),
            ])
            .areas(area);

        let buttons_area =
            Layout::vertical([Constraint::Length(3), Constraint::Length(3)]).split(buttons_area);

        let info = if let Some(i) = self.ui_state.challenges.state.selected() {
            format!("{}", self.ui_state.challenges.items[i].to_detailed_info())
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

        Paragraph::new(self.ui_state.enter_flag.input.as_str())
            .block(enter_flag_block)
            .fg(TEXT_FG_COLOR)
            .bg(NORMAL_ROW_BG)
            .render(enter_flag_area, frame.buffer_mut());

        if self.ui_state.cursor_state == CursorState::EnterFlag {
            frame.set_cursor_position(Position::new(
                enter_flag_area.x + self.ui_state.enter_flag.get_character_index() as u16 + 1,
                enter_flag_area.y + 1,
            ));
        }

        let buttons = ["Download Challenges", "Create VM"];
        for (i, &button) in buttons.iter().enumerate() {
            let block = Block::default()
                .borders(Borders::ALL)
                .style(
                    Style::default().fg(if self.ui_state.wargame_details_index == (i + 1) {
                        Color::Yellow
                    } else {
                        Color::White
                    }),
                );
            let paragraph = Paragraph::new(button)
                .block(block)
                .bg(NORMAL_ROW_BG)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, buttons_area[i]);
        }

        match self.vm_state.vm_info.get_network_info() {
            Some(network_info) => {
                Paragraph::new(Text::from(vec![
                    Line::raw(format!(
                        "System Hacking: nc {}\n",
                        network_info.get_uri_pwn()
                    )),
                    Line::raw(format!("Web Hacking: {}\n", network_info.get_uri_web())),
                ]))
                .centered()
                .bg(NORMAL_ROW_BG)
                .render(vm_info_area, frame.buffer_mut());
            }
            None => {
                Paragraph::new("")
                    .centered()
                    .bg(NORMAL_ROW_BG)
                    .render(vm_info_area, frame.buffer_mut());
            }
        }

        Paragraph::new("unimplemented...")
            .centered()
            .bg(NORMAL_ROW_BG)
            .render(unused_area, frame.buffer_mut())
    }

    pub(crate) fn render_fs_tree_view_popup(&mut self, frame: &mut Frame) {
        let popup_rect = popup_area(frame.area(), 50, 50);
        frame.render_widget(Clear, popup_rect);

        let widget = Tree::new(self.fs_state.tree_items.as_slice())
            .expect("all item identifiers are unique")
            .block(Block::bordered().title("Select a directory"))
            .experimental_scrollbar(Some(
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(None)
                    .track_symbol(None)
                    .end_symbol(None),
            ))
            .highlight_style(
                Style::new()
                    .fg(Color::Black)
                    .bg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(widget, popup_rect, &mut self.fs_state.tree_state);
    }
}

impl PopupOptions for App {
    fn popup_options<T>(&self, area: Rect, frame: &mut Frame)
    where
        T: PopupItem + PartialEq,
    {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                T::variants()
                    .iter()
                    .map(|_| Constraint::Length(3))
                    .collect::<Vec<Constraint>>(),
            )
            .split(area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }));

        for (i, item) in T::variants().iter().enumerate() {
            let style = if T::from_index(
                self.ui_state.options
                    .get_popup()
                    .get_items()
                    .get(self.ui_state.options.get_buttons_index())
                    .expect("Failed to get the selected index")
                    .get_index(),
            ) == *item
            {
                Style::default()
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let item_str = item.to_string();
            let paragraph = Paragraph::new(item_str)
                .style(style)
                .alignment(Alignment::Center);
            frame.render_widget(paragraph, chunks[i]);
        }
    }
}

impl App {
    fn popup_category(&self, area: Rect, frame: &mut Frame) {
        self.popup_options::<Category>(area, frame);
    }

    fn popup_difficulty(&self, area: Rect, frame: &mut Frame) {
        self.popup_options::<Difficulty>(area, frame);
    }

    fn popup_status(&self, area: Rect, frame: &mut Frame) {
        self.popup_options::<Status>(area, frame);
    }

    fn popup_order(&self, area: Rect, frame: &mut Frame) {
        self.popup_options::<Orderings>(area, frame);
    }
}
