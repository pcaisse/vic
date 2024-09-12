use crossterm::event::KeyCode;
use state::{EditorState, Mode};

#[test]
fn test_editor_state_switch_to_insert_mode() {
    let mut editor_state = EditorState {
        ..Default::default()
    };
    editor_state.update(KeyCode::Char('i'));
    assert_eq!(editor_state.mode, Mode::Insert)
}

#[test]
fn test_editor_state_quit() {
    let mut editor_state = EditorState {
        ..Default::default()
    };
    for key_code in [KeyCode::Char(':'), KeyCode::Char('q'), KeyCode::Enter] {
        editor_state.update(key_code);
    }
    assert!(editor_state.quit)
}

#[test]
fn test_editor_state_insert_text_repeatedly() {
    let mut editor_state = EditorState {
        ..Default::default()
    };
    for key_code in [
        KeyCode::Char('i'),
        KeyCode::Char('a'),
        KeyCode::Esc,
        KeyCode::Char('i'),
        KeyCode::Char('b'),
    ] {
        editor_state.update(key_code);
    }
    assert_eq!(editor_state.buffer, "ab")
}

#[test]
fn test_editor_state_errors() {
    let mut editor_state = EditorState {
        ..Default::default()
    };
    for key_code in [KeyCode::Char(':'), KeyCode::Char('i'), KeyCode::Enter] {
        editor_state.update(key_code);
    }
    assert!(editor_state.error.is_some())
}
