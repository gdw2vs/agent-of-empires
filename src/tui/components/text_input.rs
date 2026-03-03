//! Shared text input rendering component

use ratatui::prelude::*;
use ratatui::widgets::Paragraph;
use tui_input::Input;

use crate::tui::styles::Theme;

/// Finds the longest common prefix among a set of strings.
pub fn longest_common_prefix(values: &[String]) -> String {
    if values.is_empty() {
        return String::new();
    }

    let mut prefix = values[0].clone();
    for value in &values[1..] {
        while !value.starts_with(&prefix) {
            if prefix.pop().is_none() {
                break;
            }
        }
        if prefix.is_empty() {
            break;
        }
    }
    prefix
}

/// Renders a text input field with a label and cursor.
///
/// When focused, displays an inverse-video cursor over the current character position.
/// When not focused, displays the value (or placeholder if empty).
/// If `ghost_text` is provided, it is rendered after the cursor in dimmed style.
pub fn render_text_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    input: &Input,
    is_focused: bool,
    placeholder: Option<&str>,
    theme: &Theme,
) {
    render_text_field_with_ghost(
        frame,
        area,
        label,
        input,
        is_focused,
        placeholder,
        None,
        theme,
    );
}

/// Like `render_text_field` but with optional ghost (autocomplete) text.
#[allow(clippy::too_many_arguments)]
pub fn render_text_field_with_ghost(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    input: &Input,
    is_focused: bool,
    placeholder: Option<&str>,
    ghost_text: Option<&str>,
    theme: &Theme,
) {
    let label_style = if is_focused {
        Style::default().fg(theme.accent).underlined()
    } else {
        Style::default().fg(theme.text)
    };
    let value_style = if is_focused {
        Style::default().fg(theme.accent)
    } else {
        Style::default().fg(theme.text)
    };

    let value = input.value();

    let mut spans = vec![Span::styled(label, label_style), Span::raw(" ")];

    if value.is_empty() && !is_focused {
        if let Some(placeholder_text) = placeholder {
            spans.push(Span::styled(placeholder_text, value_style));
        }
    } else if is_focused {
        let cursor_pos = input.visual_cursor();
        let cursor_style = Style::default().fg(theme.background).bg(theme.accent);

        // Split value into: before cursor, char at cursor, after cursor
        let before: String = value.chars().take(cursor_pos).collect();
        let cursor_char: String = value
            .chars()
            .nth(cursor_pos)
            .map(|c| c.to_string())
            .unwrap_or_else(|| " ".to_string());
        let after: String = value.chars().skip(cursor_pos + 1).collect();

        if !before.is_empty() {
            spans.push(Span::styled(before, value_style));
        }
        spans.push(Span::styled(cursor_char, cursor_style));
        if !after.is_empty() {
            spans.push(Span::styled(after, value_style));
        }
        if let Some(ghost) = ghost_text {
            spans.push(Span::styled(ghost, Style::default().fg(theme.dimmed)));
        }
    } else {
        spans.push(Span::styled(value, value_style));
    }

    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
