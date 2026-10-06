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
            .title(" 🛰 FLIGHT COMPUTER & ASTRODYNAMICS TELEMETRY ")
            .title_style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let Some(id) = selected_id else {
            let empty_text = vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  📡 SENSORS SCANNING DEEP SPACE...",
                    Style::default().fg(theme.text_muted).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    "  • [Tab] Select orbiting celestial body",
                    Style::default().fg(theme.text_muted),
                )),
                Line::from(Span::styled(
                    "  • [Click & Drag] Impart manual orbital velocity",
                    Style::default().fg(theme.text_muted),
                )),
                Line::from(Span::styled(
                    "  • [W / S] Fire Prograde / Retrograde thrusters",
                    Style::default().fg(theme.text_muted),
                )),
                Line::from(Span::styled(
                    "  • [b] Birth a Supermassive Singularity / Black Hole",
                    Style::default().fg(theme.text_muted),
                )),
                Line::from(Span::styled(
                    "  • [g] Toggle Einstein Warped Spacetime Grid",
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
                Constraint::Length(3), // Header & Type
                Constraint::Length(5), // Astrodynamics Telemetry
                Constraint::Length(3), // Live Tidal Waveform Oscilloscope
                Constraint::Min(6),    // Mission Dispatch / Scratchpad Log
                Constraint::Length(3), // Thruster Controls
            ])
            .split(inner_area);

        // 1. Header & Type
        let type_color = match node.node_type {
            NodeType::Star => theme.star,
            NodeType::Pulsar => theme.pulsar,
            NodeType::BlackHole => theme.black_hole,
            NodeType::Planet => theme.planet,
            NodeType::Moon => theme.moon,
            NodeType::Asteroid => theme.asteroid,
        };

        let header_lines = vec![
            Line::from(vec![
                Span::styled(
                    format!("TARGET #{} [{}] ", node.id, node.node_type.display_badge()),
                    Style::default().fg(type_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    if node.pinned { "⚓ ANCHORED" } else { "🛸 DRIFTING" },
                    Style::default().fg(if node.pinned { theme.star } else { theme.text_muted }),
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

        // 2. Real Astrodynamics Telemetry
        let telemetry = PhysicsEngine::compute_telemetry(universe, node.id, config.gravity_g);
        let telemetry_lines = if let Some(telem) = telemetry {
            vec![
                Line::from(vec![
                    Span::styled("Orbit: ", Style::default().fg(theme.text_muted)),
                    Span::styled(
                        format!("{} (e={:.2})  ", telem.orbit_type, telem.eccentricity),
                        Style::default().fg(theme.selection).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("Anchor: ", Style::default().fg(theme.text_muted)),
                    Span::styled(telem.primary_name, Style::default().fg(theme.accent)),
                ]),
                Line::from(vec![
                    Span::styled("Altitude: ", Style::default().fg(theme.text_muted)),
                    Span::styled(format!("{:.1} AU  ", telem.altitude), Style::default().fg(theme.text_primary)),
                    Span::styled("Speed: ", Style::default().fg(theme.text_muted)),
                    Span::styled(format!("{:.2} AU/s  ", telem.speed), Style::default().fg(theme.text_primary)),
                    Span::styled("Period: ", Style::default().fg(theme.text_muted)),
                    Span::styled(
                        if telem.period_sec.is_finite() { format!("{:.1}s", telem.period_sec) } else { "ESC".into() },
                        Style::default().fg(theme.text_primary),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("Periapsis: ", Style::default().fg(theme.text_muted)),
                    Span::styled(format!("{:.1} AU  ", telem.periapsis), Style::default().fg(theme.text_primary)),
                    Span::styled("Apoapsis: ", Style::default().fg(theme.text_muted)),
                    Span::styled(
                        if telem.apoapsis.is_finite() { format!("{:.1} AU", telem.apoapsis) } else { "∞".into() },
                        Style::default().fg(theme.text_primary),
                    ),
                    Span::styled("Mass: ", Style::default().fg(theme.text_muted)),
                    Span::styled(format!("{:.0}", node.mass), Style::default().fg(theme.text_primary)),
                ]),
            ]
        } else {
            vec![
                Line::from(Span::styled("Anchor: PRIMARY COSMIC HUB", Style::default().fg(theme.accent))),
                Line::from(format!("Coordinates: ({:.1}, {:.1}) | Mass: {:.0}", node.x, node.y, node.mass)),
            ]
        };

        let telem_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme.border))
            .title(" KEPLERIAN ASTRODYNAMICS ");
        frame.render_widget(Paragraph::new(telemetry_lines).block(telem_block), chunks[1]);

        // 3. Live Gravitational Waveform Oscilloscope
        let mut waveform = String::new();
        let wave_chars = [" ", "▂", "▃", "▄", "▅", "▆", "▇", "█"];
        for i in 0..26 {
            let phase = config.sim_time * 4.0 + (i as f64 * 0.45) + (node.id as f64);
            let val = ((phase.sin() + (phase * 1.5).cos()) * 0.5 + 0.5).clamp(0.0, 0.99);
            let idx = (val * (wave_chars.len() as f64)) as usize;
            waveform.push_str(wave_chars[idx]);
        }
        let flux_val = PhysicsEngine::gravitational_potential(universe, node.x, node.y, config.gravity_g);
        let wave_lines = vec![
            Line::from(vec![
                Span::styled("WAVE: ", Style::default().fg(theme.text_muted)),
                Span::styled(waveform, Style::default().fg(theme.accent)),
                Span::styled(format!(" {:.1} Φ", flux_val), Style::default().fg(theme.selection)),
            ]),
        ];
        let wave_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme.border))
            .title(" RELATIVISTIC GRAVITATIONAL FLUX ");
        frame.render_widget(Paragraph::new(wave_lines).block(wave_block), chunks[2]);

        // 4. Mission Dispatch / Scratchpad Log
        let note_content = if node.notes.trim().is_empty() {
            vec![Line::from(Span::styled(
                "  (No mission directives logged. Press 'e' to transmit log)",
                Style::default().fg(theme.text_muted),
            ))]
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
            .title(" MISSION DIRECTIVES // SCRATCHPAD LOG ");
        frame.render_widget(
            Paragraph::new(note_content).block(notes_block).wrap(Wrap { trim: false }),
            chunks[3],
        );

        // 5. Orbital Thruster Controls
        let thruster_line = vec![
            Line::from(vec![
                Span::styled("[W] ", Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)),
                Span::styled("+Prograde  ", Style::default().fg(theme.text_primary)),
                Span::styled("[S] ", Style::default().fg(theme.spring_tense).add_modifier(Modifier::BOLD)),
                Span::styled("-Retrograde  ", Style::default().fg(theme.text_primary)),
                Span::styled("[A/D] ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
                Span::styled("Radial Steer  ", Style::default().fg(theme.text_primary)),
                Span::styled("[e] ", Style::default().fg(theme.accent)),
                Span::styled("Log", Style::default().fg(theme.text_primary)),
            ]),
        ];
        let thruster_block = Block::default()
            .borders(Borders::TOP)
            .border_style(Style::default().fg(theme.border));
        frame.render_widget(Paragraph::new(thruster_line).block(thruster_block), chunks[4]);
    }
}
