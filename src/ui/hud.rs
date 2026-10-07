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
        _spring_count: usize,
        camera_locked: bool,
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

        let lock_span = if camera_locked {
            Span::styled(" 🎯 CAM LOCKED ", Style::default().fg(theme.selection).add_modifier(Modifier::BOLD))
        } else {
            Span::styled(" 🔓 FREE CAM ", Style::default().fg(theme.text_muted))
        };

        let left_part = vec![
            mode_span,
            Span::raw(" "),
            sim_status,
            lock_span,
            Span::styled(
                format!(" │ Bodies: {} │ Zoom: {:.1}x │ WarpGrid: {}",
                    node_count,
                    zoom,
                    if config.show_spacetime_grid { "ON" } else { "OFF" }),
                Style::default().fg(theme.text_muted),
            ),
        ];

        let right_hints = vec![
            Span::styled(" [Enter] ", Style::default().fg(theme.selection).add_modifier(Modifier::BOLD)),
            Span::styled("ACTIONS MENU  ", Style::default().fg(theme.text_primary).add_modifier(Modifier::BOLD)),
            Span::styled("[F] ", Style::default().fg(theme.accent)),
            Span::styled("Lock Cam  ", Style::default().fg(theme.text_primary)),
            Span::styled("[L] ", Style::default().fg(theme.accent)),
            Span::styled("Fleet  ", Style::default().fg(theme.text_primary)),
            Span::styled("[W/S] ", Style::default().fg(theme.accent)),
            Span::styled("Burn  ", Style::default().fg(theme.text_primary)),
            Span::styled("[Space] ", Style::default().fg(theme.accent)),
            Span::styled("Pause  ", Style::default().fg(theme.text_primary)),
            Span::styled("[?] ", Style::default().fg(theme.accent)),
            Span::styled("Manual  ", Style::default().fg(theme.text_primary)),
            Span::styled("[Q] ", Style::default().fg(theme.spring_tense)),
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
