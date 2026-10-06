use crate::model::NodeType;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub struct DialogsRenderer;

impl DialogsRenderer {
    pub fn render_help(frame: &mut Frame, area: Rect, theme: &Theme) {
        let popup_area = Self::centered_rect(70, 75, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(" 🪐 ORBITFLOW // COMMANDS & FLIGHT MANUAL ")
            .title_alignment(Alignment::Center);

        let help_text = vec![
            Line::from(Span::styled(
                "─── NAVIGATION & CAMERA ──────────────────────────────────────────",
                Style::default().fg(theme.accent),
            )),
            Line::from("  [h / j / k / l] or [Arrows] : Pan camera across space"),
            Line::from("  [+] / [-] or [Mouse Scroll] : Zoom in / out"),
            Line::from("  [f]                         : Center camera on selected planet"),
            Line::from("  [0]                         : Reset camera to origin (0, 0)"),
            Line::from(""),
            Line::from(Span::styled(
                "─── MINDMAP & THOUGHT ACTIONS ───────────────────────────────────",
                Style::default().fg(theme.accent),
            )),
            Line::from("  [Tab]                       : Cycle selected celestial node"),
            Line::from("  [n]                         : Spawn a new thought (Star / Planet / Moon)"),
            Line::from("  [e]                         : Open scratchpad note editor"),
            Line::from("  [c]                         : Connect thoughts with gravitational spring"),
            Line::from("  [p]                         : Pin / Unpin node (make it an anchor)"),
            Line::from("  [d] / [x]                   : Delete selected thought"),
            Line::from("  [Left Click & Drag]         : Fling any planet with momentum!"),
            Line::from(""),
            Line::from(Span::styled(
                "─── PHYSICS SANDBOX ──────────────────────────────────────────────",
                Style::default().fg(theme.accent),
            )),
            Line::from("  [Space]                     : Pause / Resume celestial simulation"),
            Line::from("  [ [ / ] ]                   : Decrease / Increase Gravity G"),
            Line::from("  [ { / } ]                   : Decrease / Increase Cosmic Drag"),
            Line::from("  [ < / > ]                   : Slow down / Speed up time"),
            Line::from(""),
            Line::from(Span::styled(
                "─── PRESETS & SYSTEM ─────────────────────────────────────────────",
                Style::default().fg(theme.accent),
            )),
            Line::from("  [1]                         : Load Solar System Brainstorm"),
            Line::from("  [2]                         : Load Three-Body Problem"),
            Line::from("  [t]                         : Cycle TrueColor Theme"),
            Line::from("  [s]                         : Save to orbitflow.json"),
            Line::from("  [m]                         : Export galaxy to orbitflow.md (Markdown)"),
            Line::from("  [q] / [Esc]                 : Close modal / Quit OrbitFlow"),
        ];

        let p = Paragraph::new(help_text)
            .block(block)
            .wrap(Wrap { trim: false });
        frame.render_widget(p, popup_area);
    }

    pub fn render_new_node(
        frame: &mut Frame,
        area: Rect,
        input_title: &str,
        node_type: NodeType,
        tags_input: &str,
        focused_field: usize,
        theme: &Theme,
    ) {
        let popup_area = Self::centered_rect(50, 45, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(" 🌟 BIRTH NEW CELESTIAL THOUGHT ")
            .title_alignment(Alignment::Center);

        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(3), // Type selector
                Constraint::Length(3), // Tags
                Constraint::Length(2), // Help line
            ])
            .split(inner);

        // Title input
        let title_border = if focused_field == 0 {
            theme.border_focus
        } else {
            theme.border
        };
        let title_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(title_border))
            .title(" Title ");
        let title_p = Paragraph::new(format!("{}_", input_title)).block(title_block);
        frame.render_widget(title_p, chunks[0]);

        // Type selector
        let type_border = if focused_field == 1 {
            theme.border_focus
        } else {
            theme.border
        };
        let type_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(type_border))
            .title(" Type (← / → to switch) ");

        let type_str = match node_type {
            NodeType::Star => "★ Star (Anchor Hub, High Mass)",
            NodeType::Planet => "● Planet (Core Concept, Medium Mass)",
            NodeType::Moon => "◦ Moon (Sub-task / Action Item)",
            NodeType::Asteroid => "· Asteroid (Floating Stray Note)",
        };
        let type_p = Paragraph::new(type_str).block(type_block);
        frame.render_widget(type_p, chunks[1]);

        // Tags input
        let tags_border = if focused_field == 2 {
            theme.border_focus
        } else {
            theme.border
        };
        let tags_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(tags_border))
            .title(" Tags (space separated) ");
        let tags_p = Paragraph::new(format!("{}_", tags_input)).block(tags_block);
        frame.render_widget(tags_p, chunks[2]);

        // Footer hint
        let hint = Paragraph::new(Line::from(vec![
            Span::styled("[Tab] ", Style::default().fg(theme.accent)),
            Span::raw("Next Field  "),
            Span::styled("[Enter] ", Style::default().fg(theme.selection)),
            Span::raw("Create  "),
            Span::styled("[Esc] ", Style::default().fg(theme.spring_tense)),
            Span::raw("Cancel"),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(hint, chunks[3]);
    }

    pub fn render_edit_note(
        frame: &mut Frame,
        area: Rect,
        node_title: &str,
        text_content: &str,
        theme: &Theme,
    ) {
        let popup_area = Self::centered_rect(65, 60, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(format!(" 📝 EDIT SCRATCHPAD: \"{}\" ", node_title))
            .title_alignment(Alignment::Center);

        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(2)])
            .split(inner);

        let text_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border_focus))
            .title(" Content (Markdown supported) ");

        let p = Paragraph::new(format!("{}_", text_content))
            .block(text_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(p, chunks[0]);

        let footer = Paragraph::new(Line::from(vec![
            Span::styled("[Ctrl+S / Enter] ", Style::default().fg(theme.selection)),
            Span::raw("Save  "),
            Span::styled("[Esc] ", Style::default().fg(theme.spring_tense)),
            Span::raw("Cancel"),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(footer, chunks[1]);
    }

    pub fn render_connect_modal(
        frame: &mut Frame,
        area: Rect,
        source_title: &str,
        candidates: &[(usize, String, NodeType)],
        selected_idx: usize,
        theme: &Theme,
    ) {
        let popup_area = Self::centered_rect(45, 50, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(format!(" 🔗 LINK \"{}\" TO... ", source_title))
            .title_alignment(Alignment::Center);

        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(2)])
            .split(inner);

        let items: Vec<ListItem> = candidates
            .iter()
            .enumerate()
            .map(|(i, (id, title, ntype))| {
                let is_cur = i == selected_idx;
                let badge = match ntype {
                    NodeType::Star => "★",
                    NodeType::Planet => "●",
                    NodeType::Moon => "◦",
                    NodeType::Asteroid => "·",
                };
                let style = if is_cur {
                    Style::default()
                        .fg(theme.selection)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.text_primary)
                };
                ListItem::new(format!("{} #{} {} {}", if is_cur { "▶" } else { " " }, id, badge, title)).style(style)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border))
                .title(" Select Target Thought (↑ / ↓) "),
        );
        frame.render_widget(list, chunks[0]);

        let footer = Paragraph::new(Line::from(vec![
            Span::styled("[Enter] ", Style::default().fg(theme.selection)),
            Span::raw("Connect  "),
            Span::styled("[Esc] ", Style::default().fg(theme.spring_tense)),
            Span::raw("Cancel"),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(footer, chunks[1]);
    }

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
