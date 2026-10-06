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
        let popup_area = Self::centered_rect(75, 85, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(" 🪐 ORBITFLOW // ASTRODYNAMICS FLIGHT MANUAL ")
            .title_alignment(Alignment::Center);

        let help_text = vec![
            Line::from(Span::styled(
                "─── FLIGHT THRUSTERS & ASTRODYNAMICS ─────────────────────────────",
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            )),
            Line::from("  [W]                         : Prograde Thruster Burn (+Δv, raises apoapsis)"),
            Line::from("  [S]                         : Retrograde Thruster Burn (-Δv, lowers periapsis)"),
            Line::from("  [A / D]                     : Radial In/Out Thruster Burns (rotates ellipse)"),
            Line::from("  [Left Click & Drag]         : Manual orbital sling fling with momentum"),
            Line::from(""),
            Line::from(Span::styled(
                "─── RELATIVISTIC & COSMIC VISUALS ───────────────────────────────",
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            )),
            Line::from("  [g]                         : Toggle Einstein Warped Spacetime Metric Grid"),
            Line::from("  [v]                         : Toggle Velocity Vector Arrows (Flight needles)"),
            Line::from("  [o]                         : Toggle Predicted Keplerian Orbital Rings"),
            Line::from("  [b]                         : Birth a Supermassive Singularity (Black Hole)"),
            Line::from("  [k]                         : Detonate Supernova Gravitational Shockwave"),
            Line::from(""),
            Line::from(Span::styled(
                "─── NAVIGATION & FLIGHT DECK ─────────────────────────────────────",
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            )),
            Line::from("  [h / j / k / l] or [Arrows] : Pan camera across deep space"),
            Line::from("  [+] / [-] or [Mouse Scroll] : Zoom in / out"),
            Line::from("  [f]                         : Track & lock camera onto target body"),
            Line::from("  [0]                         : Reset camera to galactic origin (0, 0)"),
            Line::from("  [Tab]                       : Cycle selected celestial body"),
            Line::from(""),
            Line::from(Span::styled(
                "─── MISSION DIRECTIVES & SCRATCHPAD ─────────────────────────────",
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            )),
            Line::from("  [n]                         : Birth celestial thought (Star, Pulsar, Black Hole...)"),
            Line::from("  [e]                         : Open scratchpad mission log editor"),
            Line::from("  [c]                         : Link thoughts with elastic gravitational spring"),
            Line::from("  [p]                         : Pin / Unpin as spatial anchor"),
            Line::from("  [d] / [Delete]              : De-orbit & erase selected thought"),
            Line::from(""),
            Line::from(Span::styled(
                "─── PRESETS & SYSTEM ─────────────────────────────────────────────",
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            )),
            Line::from("  [1]                         : Preset 1 - Solar System Brainstorm"),
            Line::from("  [2]                         : Preset 2 - Chaotic Three-Body Problem"),
            Line::from("  [3]                         : Preset 3 - Gargantua Singularity Laboratory"),
            Line::from("  [t]                         : Cycle TrueColor Theme (Cyberpunk / Mocha / Amber)"),
            Line::from("  [s] / [m]                   : Save to JSON / Export to Markdown"),
            Line::from("  [Space]                     : Freeze / Resume cosmic time"),
            Line::from("  [q] / [Esc]                 : Close manual / Exit OrbitFlow"),
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
        let popup_area = Self::centered_rect(54, 48, area);
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
            .title(" Celestial Class (← / → to switch) ");

        let type_str = match node_type {
            NodeType::Star => "★ Star (Radiant Anchor Hub, Mass: 80)",
            NodeType::Pulsar => "⚡ Pulsar (Relativistic Magnetic Beams, Mass: 120)",
            NodeType::BlackHole => "🕳 Black Hole (Gravitational Singularity, Mass: 220)",
            NodeType::Planet => "● Planet (Core Conceptual Domain, Mass: 24)",
            NodeType::Moon => "◦ Moon (Sub-task / Action Item, Mass: 8)",
            NodeType::Asteroid => "· Asteroid (Floating Stray Debris, Mass: 3)",
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
            Span::raw("Birth  "),
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
            .title(format!(" 📝 MISSION LOG DISPATCH: \"{}\" ", node_title))
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
            .title(" Directives & Notes (Markdown supported) ");

        let p = Paragraph::new(format!("{}_", text_content))
            .block(text_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(p, chunks[0]);

        let footer = Paragraph::new(Line::from(vec![
            Span::styled("[Ctrl+S / Enter] ", Style::default().fg(theme.selection)),
            Span::raw("Transmit  "),
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
        let popup_area = Self::centered_rect(48, 50, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(format!(" 🔗 ESTABLISH GRAVITY LINK: \"{}\" ", source_title))
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
                    NodeType::Pulsar => "⚡",
                    NodeType::BlackHole => "🕳",
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
                .title(" Target Body (↑ / ↓) "),
        );
        frame.render_widget(list, chunks[0]);

        let footer = Paragraph::new(Line::from(vec![
            Span::styled("[Enter] ", Style::default().fg(theme.selection)),
            Span::raw("Link  "),
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
