use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{text::Text, Frame};

pub fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello World!");
    frame.render_widget(text, frame.area());
}

pub fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            if let KeyCode::Char('q') = key.code {
                return Ok(true);
            }
        }
        _ => {}
    }
    Ok(false)
}
