use std::cell::RefCell;

use crate::tui_util::{StatefulList, TabsState};
use crate::{header_widget, section_widget};

use elf_utilities::file;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Tabs};
use tui::Frame;

pub struct App<'a> {
    pub tabs: TabsState<'a>,
    pub items: RefCell<StatefulList<&'a str>>,
}

impl<'a> App<'a> {
    /// 最も大枠のレイアウトを描画する
    pub fn draw<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        file_path: String,
        elf_file: &'a file::ELF64,
    ) {
        let size = frame.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
        frame.render_widget(block, size);

        let titles = self
            .tabs
            .titles
            .iter()
            .map(|t| {
                let (first, rest) = t.split_at(1);
                Spans::from(vec![
                    Span::styled(first, Style::default().fg(Color::Yellow)),
                    Span::styled(rest, Style::default().fg(Color::Green)),
                ])
            })
            .collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title(file_path))
            .select(self.tabs.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black),
            );
        frame.render_widget(tabs, chunks[0]);

        match self.tabs.index {
            0 => self.draw_header_tab(frame, &elf_file, chunks[1]),
            1 => self.draw_section_tab(frame, &elf_file, chunks[1]),
            _ => unreachable!(),
        }
    }

    fn draw_header_tab<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        elf_file: &'a file::ELF64,
        area: Rect,
    ) {
        let inner = header_widget::header_information(&elf_file);
        frame.render_widget(inner, area);
    }
    fn draw_section_tab<B: Backend>(
        &mut self,
        frame: &mut Frame<B>,
        elf_file: &'a file::ELF64,
        area: Rect,
    ) {
        let chunks = self.split_list_and_detail(area);

        let scts = section_widget::section_list(&elf_file);
        let selected_sct = self.items.borrow().state.selected().unwrap();

        let selected_sct_name = self.items.borrow().items[selected_sct];
        frame.render_stateful_widget(scts, chunks[0], &mut self.items.borrow_mut().state);

        let selected_sct = elf_file.first_section_by(|sct| sct.name == selected_sct_name);
        let sct_info = section_widget::section_information(elf_file, selected_sct.unwrap());
        frame.render_widget(sct_info, chunks[1]);
    }
    fn split_list_and_detail(
        &self,
        area: Rect,
    ) -> Vec<Rect>{
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(70),

                ]
                    .as_ref(),
            )
            .split(area)
    }


    pub fn new(elf_file: &'a file::ELF64) -> Self {
        Self {
            tabs: TabsState::new(vec!["Header", "Sections", "Segments"]),
            items: RefCell::new(StatefulList::with_items(section_widget::section_names(
                elf_file,
            ))),
        }
    }
}
