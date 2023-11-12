use crossterm::event::{KeyCode, KeyEvent};

use crate::app::App;

pub fn update(app: &mut App, key_event: KeyEvent) {
  match key_event.code {
    KeyCode::Esc | KeyCode::Char('q') => app.quit(),
    KeyCode::Up | KeyCode::Char('w') => app.previous_item(),
    KeyCode::Down | KeyCode::Char('s') => app.next_item(),
    KeyCode::Enter => app.update_current_path(),
    _ => {},
  };
}