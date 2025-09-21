use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, List, ListItem, ListState},
    Frame,
};

use crate::{fs, App};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Main body (reserve bottom row for status)
    let body_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height.saturating_sub(1),
    };

    // Build simple list of entries with icons from fs::icons (emoji-based)
    let flat = fs::tree::flatten(&app.entries);
    let items: Vec<ListItem> = flat.iter().map(|e| {
        let icon = fs::icons::get_icon(e.is_dir, e.is_expanded);
        let indent = "  ".repeat(e.indent as usize); // two spaces per indent level
        ListItem::new(format!("{indent}{icon} {}", e.name))
    }).collect();

    let list = List::new(items)
        .block(Block::new().borders(Borders::ALL)
        .style(Style::default().fg(Color::White).bg(Color::Black)))
        .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

    let mut state = ListState::default();
    state.select(app.selected);

    frame.render_stateful_widget(list, body_area, &mut state);

    // Status bar
    let status_area = Rect {
        x: 0,
        y: area.height.saturating_sub(1),
        width: area.width,
        height: 1,
    };
    let status = Paragraph::new(app.status.as_str());
    frame.render_widget(status, status_area);

    if let Some(menu) = &app.open_menu {
        let popup_w = (area.width.saturating_sub(10)).min(60);
        let popup_h = (menu.items.len() as u16 + 2).min(area.height.saturating_sub(4));
        let x = area.x + (area.width.saturating_sub(popup_w)) / 2;
        let y = area.y + (area.height.saturating_sub(popup_h)) / 2;
        let popup_area = Rect { x, y, width: popup_w, height: popup_h };

        let items: Vec<ListItem> = menu.items.iter().map(|s| ListItem::new(s.clone())).collect();
        let mut state = ListState::default();
        state.select(Some(menu.selected));
        // small styling so popup stands out
        let list = List::new(items)
            .block(Block::new().borders(Borders::ALL).title("Open with"))
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black));
        frame.render_stateful_widget(list, popup_area, &mut state);
    }
}

