use ratatui::{
    prelude::{Alignment, Color, Constraint, Direction, Frame, Layout, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Padding, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(100)])
        .split(f.size());
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(main_layout[0]);
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(outer_layout[1]);
    let details_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(content_layout[1]);

    f.render_widget(Block::default().padding(Padding::new(1, 1, 1, 1)).borders(Borders::ALL), main_layout[0]);
    f.render_widget(
        Paragraph::new("Files and Folders Explorer")
            .alignment(Alignment::Center)
            .block(Block::new().borders(Borders::ALL)),
        outer_layout[0],
    );

    app.get_items();

    let items: Vec<ListItem> = app
        .list_of_items
        .iter()
        .map(|entry| {
            ListItem::new(entry.clone()).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    let items = List::new(items)
        .block(
            Block::default()
                .padding(Padding::new(1, 1, 1, 1))
                .borders(Borders::ALL)
                .title("Items in folder"),
        )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(items, content_layout[0], &mut app.state);
    f.render_widget(
        Paragraph::new(
            app.current_item_details
                .clone()
                .unwrap_or("Lack of details".to_string()),
        )
        .block(Block::new().padding(Padding::new(1, 1, 1, 1)).title("Metadata").borders(Borders::ALL)),
        details_layout[0],
    );
    f.render_widget(
        Paragraph::new(
            app.current_item_content
                .clone()
                .unwrap_or("Lack of content".to_string()),
        )
        .block(Block::new().padding(Padding::new(1, 1, 1, 1)).title("Content").borders(Borders::ALL)),
        details_layout[1],
    );
}
