use crate::model::{NodeType, Universe};
use crate::physics::{CosmicDust, PhysicsConfig, PhysicsEngine};
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
        config: &PhysicsConfig,
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
                    .title(" 🪐 INTERSTELLAR ORBITAL CANVAS // RELATIVISTIC FLIGHT DECK ")
                    .title_style(Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
            )
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(|ctx| {
                // 1. Draw Warped Spacetime Metric Grid (Einstein Gravitational Funnel)
                if config.show_spacetime_grid {
                    let grid_step = (16.0 / zoom).clamp(8.0, 32.0);
                    let min_gx = (x_bounds[0] / grid_step).floor() * grid_step;
                    let max_gx = (x_bounds[1] / grid_step).ceil() * grid_step;
                    let min_gy = (y_bounds[0] / grid_step).floor() * grid_step;
                    let max_gy = (y_bounds[1] / grid_step).ceil() * grid_step;

                    // Warped grid displacement helper
                    let warp_point = |gx: f64, gy: f64| -> (f64, f64) {
                        let mut dx_total = 0.0;
                        let mut dy_total = 0.0;
                        for n in &universe.nodes {
                            if matches!(n.node_type, NodeType::Star | NodeType::BlackHole | NodeType::Pulsar) {
                                let rx = gx - n.x;
                                let ry = gy - n.y;
                                let dist_sq = rx * rx + ry * ry;
                                let factor = (config.gravity_g * n.mass * 0.008) / (dist_sq + PhysicsEngine::EPSILON_SQ).powf(1.1);
                                dx_total -= rx * factor;
                                dy_total -= ry * factor;
                            }
                        }
                        (gx + dx_total, gy + dy_total)
                    };

                    // Draw Horizontal Spacetime Geodesics
                    let mut gy = min_gy;
                    while gy <= max_gy {
                        let mut gx = min_gx;
                        while gx < max_gx {
                            let p1 = warp_point(gx, gy);
                            let p2 = warp_point(gx + grid_step, gy);
                            ctx.draw(&Line {
                                x1: p1.0,
                                y1: p1.1,
                                x2: p2.0,
                                y2: p2.1,
                                color: theme.grid,
                            });
                            gx += grid_step;
                        }
                        gy += grid_step;
                    }

                    // Draw Vertical Spacetime Geodesics
                    let mut gx = min_gx;
                    while gx <= max_gx {
                        let mut gy = min_gy;
                        while gy < max_gy {
                            let p1 = warp_point(gx, gy);
                            let p2 = warp_point(gx, gy + grid_step);
                            ctx.draw(&Line {
                                x1: p1.0,
                                y1: p1.1,
                                x2: p2.0,
                                y2: p2.1,
                                color: theme.grid,
                            });
                            gy += grid_step;
                        }
                        gx += grid_step;
                    }
                }

                // 2. Ambient cosmic stardust
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

                // 3. Predicted Keplerian Orbital Rings
                if config.show_orbit_paths {
                    for node in &universe.nodes {
                        if matches!(node.node_type, NodeType::Planet | NodeType::Moon) {
                            // Find parent star
                            if let Some(conn) = universe.connections.iter().find(|c| c.from_id == node.id || c.to_id == node.id) {
                                let parent_id = if conn.from_id == node.id { conn.to_id } else { conn.from_id };
                                if let Some(parent) = universe.get_node(parent_id) {
                                    let dx = node.x - parent.x;
                                    let dy = node.y - parent.y;
                                    let radius = (dx * dx + dy * dy).sqrt();
                                    ctx.draw(&Circle {
                                        x: parent.x,
                                        y: parent.y,
                                        radius,
                                        color: theme.dust,
                                    });
                                }
                            }
                        }
                    }
                }

                // 4. Elastic connections / spring tension lines
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

                // 5. Stardust motion trails behind orbiting nodes
                for node in &universe.nodes {
                    if node.trail.len() > 1 {
                        let trail_coords: Vec<(f64, f64)> = node.trail.iter().copied().collect();
                        ctx.draw(&Points {
                            coords: &trail_coords,
                            color: theme.trail,
                        });
                    }
                }

                // 6. Draw Celestial Bodies, Relativistic Jets, and Vectors
                for node in &universe.nodes {
                    let node_color = match node.node_type {
                        NodeType::Star => theme.star,
                        NodeType::Pulsar => theme.pulsar,
                        NodeType::BlackHole => theme.black_hole,
                        NodeType::Planet => theme.planet,
                        NodeType::Moon => theme.moon,
                        NodeType::Asteroid => theme.asteroid,
                    };

                    // Special relativistic rendering for Pulsars (twin laser jets)
                    if node.node_type == NodeType::Pulsar {
                        let jet_len = 22.0;
                        let jx = node.jet_angle.cos() * jet_len;
                        let jy = node.jet_angle.sin() * jet_len;
                        ctx.draw(&Line {
                            x1: node.x - jx,
                            y1: node.y - jy,
                            x2: node.x + jx,
                            y2: node.y + jy,
                            color: theme.pulsar,
                        });
                    }

                    // Special rendering for Black Holes (Accretion disk ring + dark void)
                    if node.node_type == NodeType::BlackHole {
                        ctx.draw(&Circle {
                            x: node.x,
                            y: node.y,
                            radius: node.radius + 3.0,
                            color: theme.black_hole,
                        });
                    }

                    // Pulsing corona for Stars
                    if node.node_type == NodeType::Star {
                        let pulse = (config.sim_time * 3.0).sin() * 0.4;
                        ctx.draw(&Circle {
                            x: node.x,
                            y: node.y,
                            radius: node.radius + 1.2 + pulse,
                            color: theme.star,
                        });
                    }

                    // Node physical body
                    ctx.draw(&Circle {
                        x: node.x,
                        y: node.y,
                        radius: node.radius,
                        color: node_color,
                    });

                    // Velocity Vector Arrow (Flight trajectory)
                    if config.show_velocity_vectors && !node.pinned {
                        let speed = (node.vx * node.vx + node.vy * node.vy).sqrt();
                        if speed > 0.05 {
                            let arrow_len = (speed * 4.0).clamp(2.5, 12.0);
                            let nx = node.vx / speed;
                            let ny = node.vy / speed;
                            ctx.draw(&Line {
                                x1: node.x,
                                y1: node.y,
                                x2: node.x + nx * arrow_len,
                                y2: node.y + ny * arrow_len,
                                color: theme.vector_arrow,
                            });
                        }
                    }

                    // Selection Reticle
                    let is_selected = selected_id == Some(node.id);
                    if is_selected {
                        let box_r = node.radius + 2.2;
                        ctx.draw(&Circle {
                            x: node.x,
                            y: node.y,
                            radius: box_r,
                            color: theme.selection,
                        });
                    }

                    // Node Title Label
                    let badge = match node.node_type {
                        NodeType::Star => "★",
                        NodeType::Pulsar => "⚡",
                        NodeType::BlackHole => "🕳",
                        NodeType::Planet => "●",
                        NodeType::Moon => "◦",
                        NodeType::Asteroid => "·",
                    };
                    let label = format!("{} {}", badge, node.title);

                    let label_style = if is_selected {
                        Style::default()
                            .fg(theme.selection)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.text_primary)
                    };

                    ctx.print(
                        node.x - (label.len() as f64 * 0.4),
                        node.y - (node.radius + 2.2),
                        Span::styled(label, label_style),
                    );
                }
            });

        frame.render_widget(canvas, area);
    }
}
