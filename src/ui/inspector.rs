use crate::app::InspectorTab;
use crate::model::{NodeType, Universe};
use crate::physics::{PhysicsConfig, PhysicsEngine};
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
        config: &PhysicsConfig,
        selected_id: Option<usize>,
        active_tab: InspectorTab,
        camera_locked: bool,
        is_focused: bool,
        theme: &Theme,
    ) {
        let border_color = if is_focused {
            theme.border_focus
        } else {
            theme.border
        };

        // Tab indicators in the title bar
        let tab_title = Line::from(vec![
            Span::styled(" [1] Telemetry ", if active_tab == InspectorTab::Telemetry {
                Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_muted)
            }),
            Span::styled("│ [2] Directives/Notes ", if active_tab == InspectorTab::Notes {
                Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_muted)
            }),
            Span::styled("│ [3] Fleet ", if active_tab == InspectorTab::Fleet {
                Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.text_muted)
            }),
        ]);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(if is_focused {
                BorderType::Thick
            } else {
                BorderType::Rounded
            })
            .border_style(Style::default().fg(border_color))
            .title(tab_title);

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let Some(id) = selected_id else {
            let empty_text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  📡 NO CELESTIAL TARGET LOCKED",
                    Style::default().fg(theme.text_muted).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled("  • Press [Enter] to open Quick Actions Menu", Style::default().fg(theme.selection))),
                Line::from(Span::styled("  • Press [L] or [3] to select body from Fleet Roster", Style::default().fg(theme.text_primary))),
                Line::from(Span::styled("  • Press [Tab] to cycle targets", Style::default().fg(theme.text_primary))),
                Line::from(Span::styled("  • Click any planet on the canvas", Style::default().fg(theme.text_primary))),
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
                Constraint::Length(3), // Target Header
                Constraint::Min(6),    // Active Tab Content
                Constraint::Length(3), // Quick Flight Toolbar
            ])
            .split(inner_area);

        // 1. Target Header
        let type_color = match node.node_type {
            NodeType::Star => theme.star,
            NodeType::Pulsar => theme.pulsar,
            NodeType::BlackHole => theme.black_hole,
            NodeType::Planet => theme.planet,
            NodeType::Moon => theme.moon,
            NodeType::Asteroid => theme.asteroid,
        };

        let lock_indicator = if camera_locked {
            Span::styled(" 🎯 CAM LOCKED", Style::default().fg(theme.selection).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(" 🔓 FREE CAM", Style::default().fg(theme.text_muted))
        };

        let header_lines = vec![
            Line::from(vec![
                Span::styled(
                    format!("[{}] ", node.node_type.display_badge()),
                    Style::default().fg(type_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(format!("#{} \"{}\" ", node.id, node.title), Style::default().fg(theme.text_primary).add_modifier(Modifier::BOLD)),
                lock_indicator,
            ]),
            Line::from(vec![
                Span::styled(format!("Mass: {:.0} | Vel: {:.2} AU/s | Pos: ({:.1}, {:.1})", node.mass, (node.vx * node.vx + node.vy * node.vy).sqrt(), node.x, node.y), Style::default().fg(theme.text_muted)),
            ]),
        ];
        frame.render_widget(Paragraph::new(header_lines), chunks[0]);

        // 2. Active Tab Content
        match active_tab {
            InspectorTab::Telemetry => {
                let telemetry = PhysicsEngine::compute_telemetry(universe, node.id, config.gravity_g);
                let mut telem_lines = Vec::new();

                if let Some(telem) = telemetry {
                    telem_lines.push(Line::from(vec![
                        Span::styled("Trajectory: ", Style::default().fg(theme.text_muted)),
                        Span::styled(format!("{} (e = {:.3})", telem.orbit_type, telem.eccentricity), Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)),
                    ]));
                    telem_lines.push(Line::from(vec![
                        Span::styled("Primary Anchor: ", Style::default().fg(theme.text_muted)),
                        Span::styled(telem.primary_name, Style::default().fg(theme.accent)),
                    ]));
                    telem_lines.push(Line::from(""));
                    telem_lines.push(Line::from(vec![
                        Span::styled("Altitude:  ", Style::default().fg(theme.text_muted)),
                        Span::styled(format!("{:.1} AU", telem.altitude), Style::default().fg(theme.text_primary)),
                        Span::raw("    "),
                        Span::styled("Orbital Speed: ", Style::default().fg(theme.text_muted)),
                        Span::styled(format!("{:.2} AU/s", telem.speed), Style::default().fg(theme.text_primary)),
                    ]));
                    telem_lines.push(Line::from(vec![
                        Span::styled("Periapsis: ", Style::default().fg(theme.text_muted)),
                        Span::styled(format!("{:.1} AU", telem.periapsis), Style::default().fg(theme.text_primary)),
                        Span::raw("    "),
                        Span::styled("Apoapsis:      ", Style::default().fg(theme.text_muted)),
                        Span::styled(if telem.apoapsis.is_finite() { format!("{:.1} AU", telem.apoapsis) } else { "Escape (∞)".into() }, Style::default().fg(theme.text_primary)),
                    ]));
                    telem_lines.push(Line::from(vec![
                        Span::styled("Period T:  ", Style::default().fg(theme.text_muted)),
                        Span::styled(if telem.period_sec.is_finite() { format!("{:.1}s", telem.period_sec) } else { "Hyperbolic".into() }, Style::default().fg(theme.text_primary)),
                    ]));
                } else {
                    telem_lines.push(Line::from(Span::styled("Primary Celestial Hub (No parent anchor)", Style::default().fg(theme.accent))));
                }

                // Live Gravitational Wave Oscilloscope
                telem_lines.push(Line::from(""));
                let mut waveform = String::new();
                let wave_chars = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
                for i in 0..24 {
                    let phase = config.sim_time * 4.0 + (i as f64 * 0.45) + (node.id as f64);
                    let val = ((phase.sin() + (phase * 1.5).cos()) * 0.5 + 0.5).clamp(0.0, 0.99);
                    let idx = (val * (wave_chars.len() as f64)) as usize;
                    waveform.push_str(wave_chars[idx]);
                }
                let flux_val = PhysicsEngine::gravitational_potential(universe, node.x, node.y, config.gravity_g);
                telem_lines.push(Line::from(vec![
                    Span::styled("Tidal Flux: ", Style::default().fg(theme.text_muted)),
                    Span::styled(waveform, Style::default().fg(theme.accent)),
                    Span::styled(format!(" {:.1} Φ", flux_val), Style::default().fg(theme.selection)),
                ]));

                let telem_block = Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(theme.border))
                    .title(" KEPLERIAN TELEMETRY ");
                frame.render_widget(Paragraph::new(telem_lines).block(telem_block), chunks[1]);
            }
            InspectorTab::Notes => {
                let note_content = if node.notes.trim().is_empty() {
                    vec![
                        Line::from(""),
                        Line::from(Span::styled(
                            "  (No mission directives logged. Press [E] to edit directives)",
                            Style::default().fg(theme.text_muted),
                        )),
                    ]
                } else {
                    node.notes
                        .lines()
                        .map(|line| {
                            if line.starts_with('#') {
                                Line::from(Span::styled(
                                    line,
                                    Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
                                ))
                            } else if line.starts_with('-') || line.starts_with('*') {
                                Line::from(vec![
                                    Span::styled("◆ ", Style::default().fg(theme.selection)),
                                    Span::styled(&line[1..], Style::default().fg(theme.text_primary)),
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
                    .title(" MISSION DIRECTIVES // SCRATCHPAD [E to Edit] ");
                frame.render_widget(
                    Paragraph::new(note_content).block(notes_block).wrap(Wrap { trim: false }),
                    chunks[1],
                );
            }
            InspectorTab::Fleet => {
                let fleet_lines: Vec<Line> = universe
                    .nodes
                    .iter()
                    .map(|n| {
                        let is_this = n.id == node.id;
                        let badge = match n.node_type {
                            NodeType::Star => "★",
                            NodeType::Pulsar => "⚡",
                            NodeType::BlackHole => "🕳",
                            NodeType::Planet => "●",
                            NodeType::Moon => "◦",
                            NodeType::Asteroid => "·",
                        };
                        let style = if is_this {
                            Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(theme.text_primary)
                        };
                        Line::from(vec![
                            Span::styled(if is_this { "▶ " } else { "  " }, style),
                            Span::styled(format!("{} #{} ", badge, n.id), Style::default().fg(theme.accent)),
                            Span::styled(n.title.clone(), style),
                        ])
                    })
                    .collect();

                let fleet_block = Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::default().fg(theme.border))
                    .title(" FLEET ROSTER [Press L for Full Picker] ");
                frame.render_widget(Paragraph::new(fleet_lines).block(fleet_block), chunks[1]);
            }
        }

        // 3. Quick Action Toolbar
        let toolbar = vec![
            Line::from(vec![
                Span::styled("[Enter] ", Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)),
                Span::styled("Actions Menu  ", Style::default().fg(theme.text_primary)),
                Span::styled("[F] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Cam Lock  ", Style::default().fg(theme.text_primary)),
                Span::styled("[W/S] ", Style::default().fg(theme.accent)),
                Span::styled("Burn  ", Style::default().fg(theme.text_primary)),
                Span::styled("[E] ", Style::default().fg(theme.accent)),
                Span::styled("Edit Log", Style::default().fg(theme.text_primary)),
            ]),
        ];
        let toolbar_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme.border));
        frame.render_widget(Paragraph::new(toolbar).block(toolbar_block), chunks[2]);
    }
}
