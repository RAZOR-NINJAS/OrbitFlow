use crate::physics::PhysicsConfig;
use crate::ui::theme::Theme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct HudRenderer;

impl HudRenderer {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        config: &PhysicsConfig,
        mode_str: &str,
        node_count: usize,
        spring_count: usize,
        zoom: f64,
        theme: &Theme,
    ) {
        let sim_status = if config.paused {
            Span::styled(
                "⏸ PAUSED",
                Style::default().fg(theme.spring_tense).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::styled(
                "● RUNNING",
                Style::default().fg(theme.selection).add_modifier(Modifier::BOLD),
            )
        };

        let mode_span = Span::styled(
            format!(" [{}] ", mode_str),
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        );

        let left_part = vec![
            mode_span,
            Span::raw(" "),
            sim_status,
            Span::styled(
                format!(" │ G: {:.0} │ Drag: {:.3} │ Speed: {:.1}x │ Nodes: {} │ Springs: {} │ Zoom: {:.1}x",
                    config.gravity_g, config.damping, config.time_scale, node_count, spring_count, zoom),
                Style::default().fg(theme.text_muted),
            ),
        ];

        let right_hints = vec![
            Span::styled(" [Space] ", Style::default().fg(theme.accent)),
            Span::styled("Pause ", Style::default().fg(theme.text_primary)),
            Span::styled("[Tab] ", Style::default().fg(theme.accent)),
            Span::styled("Cycle ", Style::default().fg(theme.text_primary)),
            Span::styled("[n] ", Style::default().fg(theme.accent)),
            Span::styled("New ", Style::default().fg(theme.text_primary)),
            Span::styled("[f] ", Style::default().fg(theme.accent)),
            Span::styled("Focus ", Style::default().fg(theme.text_primary)),
            Span::styled("[t] ", Style::default().fg(theme.accent)),
            Span::styled("Theme ", Style::default().fg(theme.text_primary)),
            Span::styled("[?] ", Style::default().fg(theme.accent)),
            Span::styled("Help ", Style::default().fg(theme.text_primary)),
            Span::styled("[q] ", Style::default().fg(theme.spring_tense)),
            Span::styled("Quit", Style::default().fg(theme.text_primary)),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border));

        let paragraph = Paragraph::new(vec![
            Line::from(left_part),
            Line::from(right_hints).alignment(Alignment::Right),
        ])
        .block(block);

        frame.render_widget(paragraph, area);
    }
}
