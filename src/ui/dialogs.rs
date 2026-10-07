use crate::model::{Node, NodeType};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub static QUICK_ACTIONS: &[(&str, &str)] = &[
    ("🎯 Toggle Camera Lock", "Lock camera to orbit along with selected body (Key: F)"),
    ("🚀 Prograde Burn (+Δv)", "Fire thrusters forward to raise orbital altitude (Key: W)"),
    ("💨 Retrograde Burn (-Δv)", "Fire reverse thrusters to lower orbit (Key: S)"),
    ("🛸 Open Celestial Fleet Roster", "Browse and instantly jump to any body (Key: L)"),
    ("➕ Birth Celestial Thought", "Create a new Star, Planet, Moon, or Asteroid (Key: N)"),
    ("🕳 Birth Supermassive Black Hole", "Spawn a gravitational singularity (Key: B)"),
    ("💥 Detonate Supernova Wave", "Trigger radiating gravitational shockwave (Key: K)"),
    ("📝 Edit Mission Directives / Notes", "Open Markdown scratchpad editor (Key: E)"),
    ("🔗 Link Gravity Spring", "Connect thoughts with elastic tension (Key: C)"),
    ("🌐 Toggle Spacetime Grid", "Toggle Einstein warped spacetime mesh (Key: G)"),
    ("🚀 Toggle Velocity Vectors", "Toggle flight trajectory needles (Key: V)"),
    ("🌌 Preset 1: Solar System", "Load core brainstorm galaxy (Key: 1)"),
    ("🌌 Preset 2: Three-Body Problem", "Load chaotic orbital sandbox (Key: 2)"),
    ("🌌 Preset 3: Gargantua Singularity", "Load Black Hole & Pulsar system (Key: 3)"),
    ("🎨 Cycle TrueColor Theme", "Switch Cyberpunk / Mocha / Amber (Key: T)"),
    ("⏸ Pause / Resume Time", "Freeze or resume orbital motion (Key: Space)"),
    ("💾 Save Galaxy (JSON)", "Save progress to orbitflow.json (Key: Ctrl+S)"),
    ("📄 Export to Markdown", "Export full notes to orbitflow.md (Key: M)"),
    ("❓ Open Flight Manual", "View complete reference guide (Key: ?)"),
    ("❌ Exit OrbitFlow", "Quit cleanly to shell (Key: Q)"),
];

pub struct DialogsRenderer;

impl DialogsRenderer {
    pub fn render_quick_actions(
        frame: &mut Frame,
        area: Rect,
        selected_idx: usize,
        theme: &Theme,
    ) {
        let popup_area = Self::centered_rect(58, 70, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(" ⚡ QUICK ACTIONS PALETTE (↑/↓ Navigate, Enter Select, Esc Close) ")
            .title_alignment(Alignment::Center);

        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(2)])
            .split(inner);

        let items: Vec<ListItem> = QUICK_ACTIONS
            .iter()
            .enumerate()
            .map(|(i, (title, desc))| {
                let is_sel = i == selected_idx;
                let title_style = if is_sel {
                    Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.text_primary)
                };
                let desc_style = Style::default().fg(theme.text_muted);

                let line = Line::from(vec![
                    Span::styled(if is_sel { "▶ " } else { "  " }, title_style),
                    Span::styled(*title, title_style),
                    Span::raw("  "),
                    Span::styled(format!("— {}", desc), desc_style),
                ]);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::NONE),
        );
        frame.render_widget(list, chunks[0]);

        let footer = Paragraph::new(Line::from(vec![
            Span::styled("[Enter] ", Style::default().fg(theme.selection)),
            Span::raw("Execute Action  "),
            Span::styled("[Esc] ", Style::default().fg(theme.spring_tense)),
            Span::raw("Cancel"),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(footer, chunks[1]);
    }

    pub fn render_fleet_list(
        frame: &mut Frame,
        area: Rect,
        nodes: &[Node],
        selected_idx: usize,
        theme: &Theme,
    ) {
        let popup_area = Self::centered_rect(60, 65, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(" 🛸 CELESTIAL FLEET ROSTER (Select Body & Lock Camera) ")
            .title_alignment(Alignment::Center);

        let inner = block.inner(popup_area);
        frame.render_widget(block, popup_area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(2)])
            .split(inner);

        let items: Vec<ListItem> = nodes
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let is_sel = i == selected_idx;
                let badge = match node.node_type {
                    NodeType::Star => "★ STAR",
                    NodeType::Pulsar => "⚡ PULSAR",
                    NodeType::BlackHole => "🕳 HOLE",
                    NodeType::Planet => "● PLANET",
                    NodeType::Moon => "◦ MOON",
                    NodeType::Asteroid => "· DUST",
                };
                let speed = (node.vx * node.vx + node.vy * node.vy).sqrt();

                let style = if is_sel {
                    Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.text_primary)
                };

                let line = Line::from(vec![
                    Span::styled(if is_sel { "▶ " } else { "  " }, style),
                    Span::styled(format!("[{}] ", badge), Style::default().fg(theme.accent)),
                    Span::styled(format!("#{} \"{}\" ", node.id, node.title), style),
                    Span::styled(
                        format!("| Mass: {:.0} | Vel: {:.2} AU/s", node.mass, speed),
                        Style::default().fg(theme.text_muted),
                    ),
                ]);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items);
        frame.render_widget(list, chunks[0]);

        let footer = Paragraph::new(Line::from(vec![
            Span::styled("[Enter] ", Style::default().fg(theme.selection)),
            Span::raw("Track & Focus Target  "),
            Span::styled("[Esc] ", Style::default().fg(theme.spring_tense)),
            Span::raw("Close"),
        ]))
        .alignment(Alignment::Center);
        frame.render_widget(footer, chunks[1]);
    }

    pub fn render_help(frame: &mut Frame, area: Rect, theme: &Theme) {
        let popup_area = Self::centered_rect(75, 85, area);
        frame.render_widget(Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .border_style(Style::default().fg(theme.accent))
            .title(" 🪐 ORBITFLOW // FLIGHT MANUAL ")
            .title_alignment(Alignment::Center);

        let help_text = vec![
            Line::from(Span::styled(
                "─── ESSENTIAL CONTROLS (EASY MODE) ───────────────────────────────",
                Style::default().fg(theme.selection).add_modifier(Modifier::BOLD),
            )),
            Line::from("  [Enter] / [/]               : Open Quick Actions Palette (do anything in 1 click!)"),
            Line::from("  [L]                         : Open Celestial Fleet Roster to pick any body"),
            Line::from("  [F]                         : Toggle Camera Lock (auto-tracks selected body)"),
            Line::from("  [Space]                     : Pause / Resume cosmic time"),
            Line::from("  [1] / [2]                   : Switch Inspector Tabs (Telemetry / Notes)"),
            Line::from(""),
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
            Line::from("  [0]                         : Reset camera to galactic origin (0, 0)"),
            Line::from("  [Tab]                       : Cycle selected celestial body"),
            Line::from(""),
            Line::from(Span::styled(
                "─── PRESETS & SYSTEM ─────────────────────────────────────────────",
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            )),
            Line::from("  [F1 / 1]                    : Preset 1 - Solar System Brainstorm"),
            Line::from("  [F2 / 2]                    : Preset 2 - Chaotic Three-Body Problem"),
            Line::from("  [F3 / 3]                    : Preset 3 - Gargantua Singularity Laboratory"),
            Line::from("  [t]                         : Cycle TrueColor Theme (Cyberpunk / Mocha / Amber)"),
            Line::from("  [Ctrl+S] / [m]              : Save to JSON / Export to Markdown"),
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
