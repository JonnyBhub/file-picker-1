use std::env;
use std::io::stdout;
use std::path::Path;
use std::time::{Duration, Instant};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton,
        MouseEvent, MouseEventKind,
    },
    execute,
};

mod events;
mod fs; // src/fs/mod.rs exposes pub mod icons;
mod ui; // new: renderer module

pub struct Entry {
    // made public so ui.rs can use it
    pub name: String,
    pub is_dir: bool,
}

pub struct OpenMenu {
    pub items: Vec<String>,
    pub selected: usize,
}

pub struct App {
    // made public so ui.rs can use it
    pub status: String,
    pub entries: Vec<fs::tree::FileNode>,
    pub selected: Option<usize>,
    pub last_click: Option<(usize, Instant)>, // for double-click detection
    pub open_menu: Option<OpenMenu>,
}

fn main() {
    let mut terminal = ratatui::init();

    // Enable mouse capture
    execute!(stdout(), EnableMouseCapture).expect("failed to enable mouse capture");

    let entries = fs::tree::FileNode::read_directory(std::path::Path::new("."));

    let mut app = App {
        status: "Ready. Click or scroll. Press q or Esc to quit.".to_string(),
        entries,
        selected: None,
        last_click: None,
        open_menu: None,
    };
    if !app.entries.is_empty() {
        app.selected = Some(0);
    }

    loop {
        terminal
            .draw(|f| ui::draw(f, &app)) // call into ui module
            .expect("failed to draw frame");

        if let Some(menu) = app.open_menu.as_mut() {
            match event::read().expect("failed to read event") {
                // accept Press or Repeat so we don't skip alternating keys
                Event::Key(k)
                    if k.kind == KeyEventKind::Press || k.kind == KeyEventKind::Repeat =>
                {
                    match k.code {
                        KeyCode::Esc => {
                            app.open_menu = None;
                            app.status = "Open with canceled".to_string();
                        }
                        KeyCode::Up => {
                            if menu.items.is_empty() { /* nothing */
                            } else if menu.selected == 0 {
                                menu.selected = menu.items.len() - 1;
                            } else {
                                menu.selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if menu.items.is_empty() { /* nothing */
                            } else {
                                menu.selected = (menu.selected + 1) % menu.items.len();
                            }
                        }
                        KeyCode::Enter => {
                            // act on the selected opener (existing logic)
                            if let Some(sel_idx) = app.selected {
                                let flat = fs::tree::flatten(&app.entries);
                                if let Some(it) = flat.get(sel_idx) {
                                    let res = if menu.selected == 0 {
                                        events::open_path(&it.path)
                                    } else {
                                        let spec = &menu.items[menu.selected];
                                        events::open_with_spec(spec, &it.path)
                                    };
                                    match res {
                                        Ok(_) => {
                                            app.status = format!("Launched opener for {}", it.name)
                                        }
                                        Err(e) => app.status = format!("Open failed: {}", e),
                                    }
                                }
                            }
                            app.open_menu = None;
                        }
                        _ => {}
                    }
                }
                // ignore other events while menu is active
                _ => {}
            }
            continue;
        }

        match event::read().expect("failed to read event") {
            Event::Key(k) if k.kind == KeyEventKind::Press => match k.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char('o') => {
                    if let Some(i) = app.selected {
                        let flat = fs::tree::flatten(&app.entries);
                        if let Some(it) = flat.get(i) {
                            let items = build_openers_for(&it.path);
                            if !items.is_empty() {
                                app.open_menu = Some(OpenMenu { items, selected: 0 });
                                app.status = format!("Open with: {}", it.name);
                            }
                        }
                    }
                }
                KeyCode::Down => {
                    let flat_len = fs::tree::flatten(&app.entries).len();
                    if flat_len == 0 {
                        continue;
                    }
                    if let Some(i) = app.selected {
                        if i + 1 < flat_len {
                            app.selected = Some(i + 1);
                        }
                    } else {
                        app.selected = Some(0);
                    }
                }
                KeyCode::Up => {
                    let flat_len = fs::tree::flatten(&app.entries).len();
                    if flat_len == 0 {
                        continue;
                    }
                    if let Some(i) = app.selected {
                        if i > 0 {
                            app.selected = Some(i - 1);
                        }
                    }
                }
                KeyCode::Right => {
                    if let Some(i) = app.selected {
                        let flat = fs::tree::flatten(&app.entries);
                        if let Some(it) = flat.get(i) {
                            if it.is_dir {
                                let idx = it.idx_path.clone();
                                if let Some(node) = with_node_mut(&mut app.entries, &idx) {
                                    if !node.is_expanded {
                                        node.expand(); // load children lazily (implemented in your tree.rs)
                                        app.status = format!("Expanded {}", node.name);
                                    }
                                }
                            }
                        }
                    }
                    clamp_selected(&mut app);
                }
                KeyCode::Left => {
                    if let Some(i) = app.selected {
                        let flat = fs::tree::flatten(&app.entries);
                        if let Some(it) = flat.get(i) {
                            if it.is_dir {
                                let idx = it.idx_path.clone();
                                if let Some(node) = with_node_mut(&mut app.entries, &idx) {
                                    if node.is_expanded {
                                        node.collapse();
                                        app.status = format!("Collapsed {}", node.name);
                                    }
                                }
                            }
                        }
                    }
                    clamp_selected(&mut app);
                }
                KeyCode::Enter => {
                    if let Some(i) = app.selected {
                        let flat = fs::tree::flatten(&app.entries);
                        if let Some(it) = flat.get(i) {
                            if it.is_dir {
                                let idx = it.idx_path.clone();
                                if let Some(node) = with_node_mut(&mut app.entries, &idx) {
                                    if node.is_expanded {
                                        node.collapse();
                                        app.status = format!("Collapsed folder: {}", node.name);
                                    } else {
                                        node.expand();
                                        app.status = format!("Expanded folder: {}", node.name);
                                    }
                                }
                            } else {
                                match events::open_path(&it.path) {
                                    Ok(_) => app.status = format!("Opening {}", it.path.display()),
                                    Err(e) => {
                                        app.status =
                                            format!("Failed to open {}: {}", it.path.display(), e)
                                    }
                                }
                            }
                        }
                    }
                    clamp_selected(&mut app);
                }
                _ => {}
            },
            Event::Mouse(m) => {
                match m.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        // Map mouse row to list index (account for borders + status bar)
                        let (_w, h) = crossterm::terminal::size().unwrap_or((0, 0));
                        if h >= 3 {
                            let body_h = h.saturating_sub(1);
                            let inner_start_y = 1u16;
                            let inner_rows = body_h.saturating_sub(2);
                            let y = m.row;
                            if y >= inner_start_y && y < inner_start_y + inner_rows {
                                let clicked_idx = (y - inner_start_y) as usize;
                                let flat = fs::tree::flatten(&app.entries);
                                if clicked_idx < flat.len() {
                                    // Select on single click
                                    app.selected = Some(clicked_idx);
                                    app.status = format!("Selected {}", flat[clicked_idx].name);

                                    // Detect double-click within 350ms on same row
                                    let now = Instant::now();
                                    let dbl_thresh = Duration::from_millis(350);
                                    if let Some((last_idx, t)) = app.last_click {
                                        if last_idx == clicked_idx
                                            && now.duration_since(t) <= dbl_thresh
                                        {
                                            // Double-click: act on the item
                                            let it = &flat[clicked_idx];
                                            if it.is_dir {
                                                let idx = it.idx_path.clone();
                                                if let Some(node) =
                                                    with_node_mut(&mut app.entries, &idx)
                                                {
                                                    if node.is_expanded {
                                                        node.collapse();
                                                        app.status =
                                                            format!("Collapsed {}", node.name);
                                                    } else {
                                                        node.expand();
                                                        app.status =
                                                            format!("Expanded {}", node.name);
                                                    }
                                                }
                                            } else {
                                                match events::open_path(&it.path) {
                                                    Ok(_) => {
                                                        app.status =
                                                            format!("Opening {}", it.path.display())
                                                    }
                                                    Err(e) => {
                                                        app.status = format!(
                                                            "Failed to open {}: {}",
                                                            it.path.display(),
                                                            e
                                                        )
                                                    }
                                                }
                                            }
                                            clamp_selected(&mut app);
                                            app.last_click = None; // reset after double-click
                                            continue;
                                        }
                                    }
                                    // Not a double-click; remember this click
                                    app.last_click = Some((clicked_idx, now));
                                }
                            }
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        let flat_len = fs::tree::flatten(&app.entries).len();
                        if flat_len == 0 {
                            continue;
                        }
                        if let Some(i) = app.selected {
                            if i > 0 {
                                app.selected = Some(i - 1);
                                let flat = fs::tree::flatten(&app.entries);
                                app.status = format!("Selected {}", flat[i - 1].name);
                            }
                        } else {
                            app.selected = Some(0);
                            let flat = fs::tree::flatten(&app.entries);
                            app.status = format!("Selected {}", flat[0].name);
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        let flat_len = fs::tree::flatten(&app.entries).len();
                        if flat_len == 0 {
                            continue;
                        }
                        if let Some(i) = app.selected {
                            if i + 1 < flat_len {
                                app.selected = Some(i + 1);
                                let flat = fs::tree::flatten(&app.entries);
                                app.status = format!("Selected {}", flat[i + 1].name);
                            }
                        } else {
                            app.selected = Some(0);
                            let flat = fs::tree::flatten(&app.entries);
                            app.status = format!("Selected {}", flat[0].name);
                        }
                    }
                    _ => {
                        app.status = describe_mouse(m);
                    }
                }
            }
            Event::Resize(_, _) => { /* redraw next loop */ }
            _ => {}
        }
    }

    // Disable mouse capture
    execute!(stdout(), DisableMouseCapture).expect("failed to disable mouse capture");
    ratatui::restore();
}

fn build_openers_for(path: &Path) -> Vec<String> {
    let mut out = Vec::new();
    out.push("System Default".to_string()); // index 0 = default behavior

    // optional: per-extension env override shown in menu
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        let key = format!("FILE_PICKER_EXT_{}", ext.to_ascii_lowercase());
        if let Ok(spec) = env::var(&key) {
            out.push(spec);
        }
    }

    // optional: user-provided ad-hoc openers (semicolon separated)
    if let Ok(list) = env::var("FILE_PICKER_OPENERS") {
        // support splitting by ';' or newline
        for spec in list.split([';', '\n']) {
            let spec = spec.trim();
            if !spec.is_empty() {
                out.push(spec.to_string());
            }
        }
    }

    // Example helpful defaults (only if not duplicate)
    if !out.iter().any(|s| s.contains("code")) {
        out.push("code -g".to_string());
    }
    if !out.iter().any(|s| s.contains("open -a")) {
        out.push(r#"open -a "Visual Studio Code""#.to_string());
    }

    out
}

fn describe_mouse(m: MouseEvent) -> String {
    match m.kind {
        MouseEventKind::Down(btn) => format!("Mouse Down {:?} at ({}, {})", btn, m.column, m.row),
        MouseEventKind::Up(btn) => format!("Mouse Up {:?} at ({}, {})", btn, m.column, m.row),
        MouseEventKind::Drag(btn) => format!("Mouse Drag {:?} at ({}, {})", btn, m.column, m.row),
        MouseEventKind::Moved => format!("Mouse Move at ({}, {})", m.column, m.row),
        MouseEventKind::ScrollUp => format!("Mouse ScrollUp at ({}, {})", m.column, m.row),
        MouseEventKind::ScrollDown => format!("Mouse ScrollDown at ({}, {})", m.column, m.row),
        _ => "Unknown mouse event".to_string(),
    }
}

fn with_node_mut<'a>(
    nodes: &'a mut [fs::tree::FileNode],
    idx_path: &[usize],
) -> Option<&'a mut fs::tree::FileNode> {
    let (first, rest) = idx_path.split_first()?;
    let node = nodes.get_mut(*first)?;
    if rest.is_empty() {
        Some(node)
    } else {
        with_node_mut(&mut node.children, rest)
    }
}

// Ensure selected is within the visible range after expand/collapse.
fn clamp_selected(app: &mut App) {
    let len = fs::tree::flatten(&app.entries).len();
    match (len, app.selected) {
        (0, _) => app.selected = None,
        (n, Some(i)) if i >= n => app.selected = Some(n - 1),
        (..) => {} // already valid
    }
}
