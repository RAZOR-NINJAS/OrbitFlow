use crate::model::{NodeType, Universe};
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

pub struct InspectorRenderer;

impl InspectorRenderer {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        universe: &Universe,
        selected_id: Option<usize>,
        is_focused: bool,
        theme: &Theme,
    ) {
        let border_color = if is_focused {
            theme.border_focus
        } else {
            theme.border
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(if is_focused {
                BorderType::Thick
            } else {
                BorderType::Rounded
            })
            .border_style(Style::default().fg(border_color))
            .title(" 📝 SCRATCHPAD & INSPECTOR ")
            .title_style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let Some(id) = selected_id else {
            let empty_text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  🌌 No Celestial Body Selected",
                    Style::default().fg(theme.text_muted).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  • Press [Tab] to cycle through nodes",
                    Style::default().fg(theme.text_muted),
                )),
                Line::from(Span::styled(
                    "  • Click any planet on the canvas",
                    Style::default().fg(theme.text_muted),
                )),
                Line::from(Span::styled(
                    "  • Press [n] to birth a new celestial idea",
                    Style::default().fg(theme.text_muted),
                )),
                Line::from(Span::styled(
                    "  • Press [1-3] to switch galaxy presets",
                    Style::default().fg(theme.text_muted),
                )),
            ];
            frame.render_widget(Paragraph::new(empty_text), inner_area);
            return;
        };

        let Some(node) = universe.get_node(id) else {
            return;
        };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Header & Type
                Constraint::Length(3), // Physics Stats
                Constraint::Length(2), // Tags
                Constraint::Min(6),    // Notes / Scratchpad
                Constraint::Length(4), // Connected Orbiters
                Constraint::Length(3), // Action bar
            ])
            .split(inner_area);

        // 1. Header & Type
        let type_color = match node.node_type {
            NodeType::Star => theme.star,
            NodeType::Planet => theme.planet,
            NodeType::Moon => theme.moon,
            NodeType::Asteroid => theme.asteroid,
        };

        let header_lines = vec![
            Line::from(vec![
                Span::styled(
                    format!("[{}] ", node.node_type.display_badge()),
                    Style::default().fg(type_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("#{}", node.id),
                    Style::default().fg(theme.text_muted),
                ),
            ]),
            Line::from(vec![Span::styled(
                format!("\"{}\"", node.title),
                Style::default()
                    .fg(theme.text_primary)
                    .add_modifier(Modifier::BOLD),
            )]),
        ];
        frame.render_widget(Paragraph::new(header_lines), chunks[0]);

        // 2. Physics properties
        let speed = (node.vx * node.vx + node.vy * node.vy).sqrt();
        let stats_lines = vec![
            Line::from(vec![
                Span::styled("Mass: ", Style::default().fg(theme.text_muted)),
                Span::styled(format!("{:.1}  ", node.mass), Style::default().fg(theme.text_primary)),
                Span::styled("Speed: ", Style::default().fg(theme.text_muted)),
                Span::styled(format!("{:.2}  ", speed), Style::default().fg(theme.text_primary)),
                Span::styled("Pinned: ", Style::default().fg(theme.text_muted)),
                Span::styled(
                    if node.pinned { "YES (Anchor) [p]" } else { "NO [p]" },
                    Style::default().fg(if node.pinned { theme.star } else { theme.text_muted }),
                ),
            ]),
            Line::from(vec![
                Span::styled("Coordinates: ", Style::default().fg(theme.text_muted)),
                Span::styled(
                    format!("({:.1}, {:.1})", node.x, node.y),
                    Style::default().fg(theme.text_primary),
                ),
            ]),
        ];
        frame.render_widget(Paragraph::new(stats_lines), chunks[1]);

        // 3. Tags
        let tags_text = if node.tags.is_empty() {
            vec![Line::from(Span::styled("Tags: (none)", Style::default().fg(theme.text_muted)))]
        } else {
            let tags_str = node.tags.join(" ");
            vec![Line::from(vec![
                Span::styled("Tags: ", Style::default().fg(theme.text_muted)),
                Span::styled(tags_str, Style::default().fg(theme.accent)),
            ])]
        };
        frame.render_widget(Paragraph::new(tags_text), chunks[2]);

        // 4. Notes / Markdown Scratchpad
        let note_content = if node.notes.trim().is_empty() {
            vec![Line::from(Span::styled(
                "  (No scratchpad notes yet. Press 'e' to edit notes)",
                Style::default().fg(theme.text_muted),
            ))]
        } else {
            node.notes
                .lines()
                .map(|line| {
                    if line.starts_with("# ") {
                        Line::from(Span::styled(
                            line,
                            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
                        ))
                    } else if line.starts_with("- ") || line.starts_with("* ") {
                        Line::from(vec![
                            Span::styled("• ", Style::default().fg(theme.selection)),
                            Span::styled(&line[2..], Style::default().fg(theme.text_primary)),
                        ])
                    } else {
                        Line::from(Span::styled(line, Style::default().fg(theme.text_primary)))
                    }
                })
                .collect()
        };

        let notes_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme.border))
            .title(" NOTES ");
        frame.render_widget(
            Paragraph::new(note_content).block(notes_block).wrap(Wrap { trim: false }),
            chunks[3],
        );

        // 5. Connections
        let connected_nodes: Vec<String> = universe
            .connections
            .iter()
            .filter_map(|c| {
                if c.from_id == node.id {
                    universe.get_node(c.to_id).map(|n| format!("→ {}", n.title))
                } else if c.to_id == node.id {
                    universe.get_node(c.from_id).map(|n| format!("← {}", n.title))
                } else {
                    None
                }
            })
            .collect();

        let conn_text = if connected_nodes.is_empty() {
            vec![Line::from(Span::styled("Connections: None (press 'c' to link)", Style::default().fg(theme.text_muted)))]
        } else {
            vec![
                Line::from(Span::styled(
                    format!("Links ({}):", connected_nodes.len()),
                    Style::default().fg(theme.text_muted),
                )),
                Line::from(Span::styled(
                    connected_nodes.join(", "),
                    Style::default().fg(theme.spring_relaxed),
                )),
            ]
        };
        let conn_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme.border));
        frame.render_widget(Paragraph::new(conn_text).block(conn_block), chunks[4]);

        // 6. Action quick hints
        let actions = vec![
            Line::from(vec![
                Span::styled("[e] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Edit Note  ", Style::default().fg(theme.text_primary)),
                Span::styled("[c] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Connect  ", Style::default().fg(theme.text_primary)),
                Span::styled("[p] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Pin  ", Style::default().fg(theme.text_primary)),
                Span::styled("[d] ", Style::default().fg(theme.spring_tense).add_modifier(Modifier::BOLD)),
                Span::styled("Delete", Style::default().fg(theme.text_primary)),
            ]),
        ];
        frame.render_widget(Paragraph::new(actions), chunks[5]);
    }
}
