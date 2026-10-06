use crate::model::{NodeType, Universe};
use crate::physics::CosmicDust;
use crate::ui::theme::Theme;
use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Circle, Line, Points},
        Block, BorderType, Borders,
    },
    Frame,
};

pub struct CanvasRenderer;

impl CanvasRenderer {
    pub fn render(
        frame: &mut Frame,
        area: Rect,
        universe: &Universe,
        dust: &CosmicDust,
        camera_x: f64,
        camera_y: f64,
        zoom: f64,
        selected_id: Option<usize>,
        is_focused: bool,
        theme: &Theme,
    ) {
        let aspect = if area.height > 0 {
            (area.width as f64) / (area.height as f64 * 2.0)
        } else {
            1.0
        };

        let half_h = 45.0 / zoom;
        let half_w = half_h * aspect;

        let x_bounds = [camera_x - half_w, camera_x + half_w];
        let y_bounds = [camera_y - half_h, camera_y + half_h];

        let border_color = if is_focused {
            theme.border_focus
        } else {
            theme.border
        };

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(if is_focused {
                        BorderType::Thick
                    } else {
                        BorderType::Rounded
                    })
                    .border_style(Style::default().fg(border_color))
                    .title(" 🪐 GALAXY VIEWPORT (Braille Canvas) ")
                    .title_style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            )
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(|ctx| {
                // 1. Draw cosmic background stardust
                let dust_coords: Vec<(f64, f64)> = dust
                    .particles
                    .iter()
                    .filter(|p| {
                        p.x >= x_bounds[0]
                            && p.x <= x_bounds[1]
                            && p.y >= y_bounds[0]
                            && p.y <= y_bounds[1]
                    })
                    .map(|p| (p.x, p.y))
                    .collect();

                ctx.draw(&Points {
                    coords: &dust_coords,
                    color: theme.dust,
                });

                // 2. Draw elastic connections / spring lines
                for conn in &universe.connections {
                    let from_pos = universe.nodes.iter().find(|n| n.id == conn.from_id).map(|n| (n.x, n.y));
                    let to_pos = universe.nodes.iter().find(|n| n.id == conn.to_id).map(|n| (n.x, n.y));

                    if let (Some((x1, y1)), Some((x2, y2))) = (from_pos, to_pos) {
                        let color = if conn.current_tension.abs() > 0.4 {
                            theme.spring_tense
                        } else {
                            theme.spring_relaxed
                        };

                        ctx.draw(&Line {
                            x1,
                            y1,
                            x2,
                            y2,
                            color,
                        });
                    }
                }

                // 3. Draw stardust motion trails behind orbiting nodes
                for node in &universe.nodes {
                    if node.trail.len() > 1 {
                        let trail_coords: Vec<(f64, f64)> = node.trail.iter().copied().collect();
                        ctx.draw(&Points {
                            coords: &trail_coords,
                            color: theme.trail,
                        });
                    }
                }

                // 4. Draw celestial bodies
                for node in &universe.nodes {
                    let node_color = match node.node_type {
                        NodeType::Star => theme.star,
                        NodeType::Planet => theme.planet,
                        NodeType::Moon => theme.moon,
                        NodeType::Asteroid => theme.asteroid,
                    };

                    // Draw node physical body
                    ctx.draw(&Circle {
                        x: node.x,
                        y: node.y,
                        radius: node.radius,
                        color: node_color,
                    });

                    // Draw selection reticle
                    let is_selected = selected_id == Some(node.id);
                    if is_selected {
                        let box_r = node.radius + 1.8;
                        ctx.draw(&Circle {
                            x: node.x,
                            y: node.y,
                            radius: box_r,
                            color: theme.selection,
                        });
                    }

                    // Print label
                    let label = format!(
                        "{} {}",
                        match node.node_type {
                            NodeType::Star => "★",
                            NodeType::Planet => "●",
                            NodeType::Moon => "◦",
                            NodeType::Asteroid => "·",
                        },
                        node.title
                    );

                    let label_style = if is_selected {
                        Style::default()
                            .fg(theme.selection)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.text_primary)
                    };

                    // Offset label slightly below the node
                    ctx.print(
                        node.x - (label.len() as f64 * 0.4),
                        node.y - (node.radius + 2.0),
                        Span::styled(label, label_style),
                    );
                }
            });

        frame.render_widget(canvas, area);
    }
}
