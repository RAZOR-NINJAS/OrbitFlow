mod app;
mod model;
mod physics;
mod storage;
mod ui;

use app::{App, AppMode, FocusedPanel};
use model::NodeType;
use ui::{CanvasRenderer, DialogsRenderer, HudRenderer, InspectorRenderer};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
        MouseButton, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};
use std::io::stdout;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize terminal in raw mode with alternate screen & mouse capture
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Initialize application state
    let mut app = App::new();
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    // 3. Main event loop
    loop {
        // Draw frame
        terminal.draw(|frame| {
            let area = frame.area();

            // Main vertical layout: Top content (Canvas + Inspector) & Bottom HUD
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(8), Constraint::Length(3)])
                .split(area);

            // Top horizontal layout: Canvas (70%) and Inspector (30%)
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(main_chunks[0]);

            app.canvas_area = top_chunks[0];

            // Render Canvas
            CanvasRenderer::render(
                frame,
                top_chunks[0],
                &app.universe,
                &app.cosmic_dust,
                app.camera_x,
                app.camera_y,
                app.zoom,
                app.selected_node_id,
                app.focused_panel == FocusedPanel::Canvas,
                &app.theme,
            );

            // Render Inspector / Scratchpad
            InspectorRenderer::render(
                frame,
                top_chunks[1],
                &app.universe,
                app.selected_node_id,
                app.focused_panel == FocusedPanel::Inspector,
                &app.theme,
            );

            // Render HUD
            let mode_str = match &app.mode {
                AppMode::Normal => "NORMAL",
                AppMode::Help => "HELP",
                AppMode::NewNode { .. } => "BIRTH_THOUGHT",
                AppMode::EditNote { .. } => "EDIT_SCRATCHPAD",
                AppMode::Connect { .. } => "GRAVITY_LINK",
                AppMode::DraggingNode { .. } => "FLING_ORBIT",
            };

            HudRenderer::render(
                frame,
                main_chunks[1],
                &app.physics_config,
                mode_str,
                app.universe.nodes.len(),
                app.universe.connections.len(),
                app.zoom,
                &app.theme,
            );

            // Render Overlays / Modals based on active mode
            match &app.mode {
                AppMode::Help => {
                    DialogsRenderer::render_help(frame, area, &app.theme);
                }
                AppMode::NewNode {
                    title,
                    node_type,
                    tags,
                    focused_field,
                } => {
                    DialogsRenderer::render_new_node(
                        frame,
                        area,
                        title,
                        *node_type,
                        tags,
                        *focused_field,
                        &app.theme,
                    );
                }
                AppMode::EditNote { buffer } => {
                    let title = app
                        .selected_node_id
                        .and_then(|id| app.universe.get_node(id))
                        .map(|n| n.title.as_str())
                        .unwrap_or("Untitled");
                    DialogsRenderer::render_edit_note(frame, area, title, buffer, &app.theme);
                }
                AppMode::Connect {
                    candidates,
                    selected_idx,
                } => {
                    let source_title = app
                        .selected_node_id
                        .and_then(|id| app.universe.get_node(id))
                        .map(|n| n.title.clone())
                        .unwrap_or_else(|| "Source".into());
                    DialogsRenderer::render_connect_modal(
                        frame,
                        area,
                        &source_title,
                        candidates,
                        *selected_idx,
                        &app.theme,
                    );
                }
                _ => {}
            }
        })?;

        // Process inputs
        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    handle_key_event(&mut app, key.code, key.modifiers);
                }
                Event::Mouse(mouse) => {
                    handle_mouse_event(&mut app, mouse);
                }
                _ => {}
            }
        }

        // Advance simulation tick
        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f64();
            app.tick(dt.min(0.05));
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // 4. Restore terminal state cleanly
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    println!("✨ OrbitFlow exited gracefully. Keep your thoughts orbiting!");
    Ok(())
}

fn handle_key_event(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    match &app.mode {
        AppMode::Help => {
            if matches!(code, KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?')) {
                app.mode = AppMode::Normal;
            }
        }
        AppMode::NewNode { .. } => {
            if let AppMode::NewNode {
                mut title,
                mut node_type,
                mut tags,
                mut focused_field,
            } = std::mem::replace(&mut app.mode, AppMode::Normal)
            {
                match code {
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Tab => {
                        focused_field = (focused_field + 1) % 3;
                        app.mode = AppMode::NewNode {
                            title,
                            node_type,
                            tags,
                            focused_field,
                        };
                    }
                    KeyCode::Left if focused_field == 1 => {
                        node_type = match node_type {
                            NodeType::Star => NodeType::Asteroid,
                            NodeType::Planet => NodeType::Star,
                            NodeType::Moon => NodeType::Planet,
                            NodeType::Asteroid => NodeType::Moon,
                        };
                        app.mode = AppMode::NewNode {
                            title,
                            node_type,
                            tags,
                            focused_field,
                        };
                    }
                    KeyCode::Right if focused_field == 1 => {
                        node_type = match node_type {
                            NodeType::Star => NodeType::Planet,
                            NodeType::Planet => NodeType::Moon,
                            NodeType::Moon => NodeType::Asteroid,
                            NodeType::Asteroid => NodeType::Star,
                        };
                        app.mode = AppMode::NewNode {
                            title,
                            node_type,
                            tags,
                            focused_field,
                        };
                    }
                    KeyCode::Backspace => {
                        if focused_field == 0 {
                            title.pop();
                        } else if focused_field == 2 {
                            tags.pop();
                        }
                        app.mode = AppMode::NewNode {
                            title,
                            node_type,
                            tags,
                            focused_field,
                        };
                    }
                    KeyCode::Char(c) => {
                        if focused_field == 0 {
                            title.push(c);
                        } else if focused_field == 2 {
                            tags.push(c);
                        }
                        app.mode = AppMode::NewNode {
                            title,
                            node_type,
                            tags,
                            focused_field,
                        };
                    }
                    KeyCode::Enter => {
                        if !title.trim().is_empty() {
                            use rand::Rng;
                            let mut rng = rand::thread_rng();
                            let angle: f64 = rng.gen_range(0.0..std::f64::consts::TAU);
                            let dist: f64 = rng.gen_range(20.0..50.0);
                            let x = app.camera_x + angle.cos() * dist;
                            let y = app.camera_y + angle.sin() * dist;

                            let new_id = app.universe.add_node(title.clone(), node_type, x, y);
                            let tag_list: Vec<String> = tags
                                .split_whitespace()
                                .map(|s| {
                                    if s.starts_with('#') {
                                        s.to_string()
                                    } else {
                                        format!("#{}", s)
                                    }
                                })
                                .collect();

                            if let Some(n) = app.universe.get_node_mut(new_id) {
                                n.tags = tag_list;
                                n.vx = -angle.sin() * 0.8;
                                n.vy = angle.cos() * 0.8;
                            }

                            if let Some(sel_id) = app.selected_node_id {
                                if sel_id != new_id {
                                    app.universe.connect(sel_id, new_id, 30.0);
                                }
                            }

                            app.selected_node_id = Some(new_id);
                            app.set_status(format!("Spawned \"{}\"", title));
                        }
                        app.mode = AppMode::Normal;
                    }
                    _ => {
                        app.mode = AppMode::NewNode {
                            title,
                            node_type,
                            tags,
                            focused_field,
                        };
                    }
                }
            }
        }
        AppMode::EditNote { .. } => {
            if let AppMode::EditNote { mut buffer } =
                std::mem::replace(&mut app.mode, AppMode::Normal)
            {
                match code {
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Enter => {
                        if modifiers.contains(KeyModifiers::CONTROL) {
                            if let Some(id) = app.selected_node_id {
                                if let Some(node) = app.universe.get_node_mut(id) {
                                    node.notes = buffer;
                                    app.set_status("Notes updated!");
                                }
                            }
                            app.mode = AppMode::Normal;
                        } else {
                            buffer.push('\n');
                            app.mode = AppMode::EditNote { buffer };
                        }
                    }
                    KeyCode::Backspace => {
                        buffer.pop();
                        app.mode = AppMode::EditNote { buffer };
                    }
                    KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                        if let Some(id) = app.selected_node_id {
                            if let Some(node) = app.universe.get_node_mut(id) {
                                node.notes = buffer;
                                app.set_status("Notes updated!");
                            }
                        }
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Char(c) => {
                        buffer.push(c);
                        app.mode = AppMode::EditNote { buffer };
                    }
                    _ => {
                        app.mode = AppMode::EditNote { buffer };
                    }
                }
            }
        }
        AppMode::Connect { .. } => {
            if let AppMode::Connect {
                candidates,
                mut selected_idx,
            } = std::mem::replace(&mut app.mode, AppMode::Normal)
            {
                match code {
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if selected_idx > 0 {
                            selected_idx -= 1;
                        }
                        app.mode = AppMode::Connect {
                            candidates,
                            selected_idx,
                        };
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if !candidates.is_empty() && selected_idx + 1 < candidates.len() {
                            selected_idx += 1;
                        }
                        app.mode = AppMode::Connect {
                            candidates,
                            selected_idx,
                        };
                    }
                    KeyCode::Enter => {
                        if let Some((target_id, _, _)) = candidates.get(selected_idx) {
                            if let Some(source_id) = app.selected_node_id {
                                app.universe.connect(source_id, *target_id, 32.0);
                                app.set_status("Gravitational spring linked!");
                            }
                        }
                        app.mode = AppMode::Normal;
                    }
                    _ => {
                        app.mode = AppMode::Connect {
                            candidates,
                            selected_idx,
                        };
                    }
                }
            }
        }
        AppMode::DraggingNode { .. } => {
            if code == KeyCode::Esc {
                app.mode = AppMode::Normal;
            }
        }
        AppMode::Normal => match code {
            KeyCode::Char('q') => {
                app.should_quit = true;
            }
            KeyCode::Char('?') => {
                app.mode = AppMode::Help;
            }
            KeyCode::Char(' ') => {
                app.physics_config.paused = !app.physics_config.paused;
                app.set_status(if app.physics_config.paused {
                    "Simulation paused"
                } else {
                    "Simulation active"
                });
            }
            KeyCode::Tab => {
                app.cycle_selected_node();
            }
            // Pan camera
            KeyCode::Char('h') | KeyCode::Left => {
                app.camera_x -= 6.0 / app.zoom;
            }
            KeyCode::Char('l') | KeyCode::Right => {
                app.camera_x += 6.0 / app.zoom;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                app.camera_y += 6.0 / app.zoom;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                app.camera_y -= 6.0 / app.zoom;
            }
            // Zoom
            KeyCode::Char('+') | KeyCode::Char('=') => {
                app.zoom = (app.zoom * 1.15).min(5.0);
            }
            KeyCode::Char('-') => {
                app.zoom = (app.zoom / 1.15).max(0.3);
            }
            KeyCode::Char('0') => {
                app.camera_x = 0.0;
                app.camera_y = 0.0;
                app.zoom = 1.0;
                app.set_status("Camera reset to cosmic center");
            }
            KeyCode::Char('f') => {
                app.focus_camera_on_selected();
            }
            KeyCode::Char('p') => {
                app.toggle_pin_selected();
            }
            KeyCode::Char('d') | KeyCode::Delete => {
                app.delete_selected_node();
            }
            KeyCode::Char('t') => {
                app.cycle_theme();
            }
            KeyCode::Char('n') => {
                app.mode = AppMode::NewNode {
                    title: String::new(),
                    node_type: NodeType::Planet,
                    tags: String::new(),
                    focused_field: 0,
                };
            }
            KeyCode::Char('e') => {
                if let Some(id) = app.selected_node_id {
                    let current_notes = app
                        .universe
                        .get_node(id)
                        .map(|n| n.notes.clone())
                        .unwrap_or_default();
                    app.mode = AppMode::EditNote {
                        buffer: current_notes,
                    };
                }
            }
            KeyCode::Char('c') => {
                if let Some(source_id) = app.selected_node_id {
                    let candidates: Vec<(usize, String, NodeType)> = app
                        .universe
                        .nodes
                        .iter()
                        .filter(|n| n.id != source_id)
                        .map(|n| (n.id, n.title.clone(), n.node_type))
                        .collect();
                    if !candidates.is_empty() {
                        app.mode = AppMode::Connect {
                            candidates,
                            selected_idx: 0,
                        };
                    } else {
                        app.set_status("Need at least 2 nodes to connect");
                    }
                }
            }
            // Physics sliders
            KeyCode::Char('[') => {
                app.physics_config.gravity_g = (app.physics_config.gravity_g - 10.0).max(0.0);
                app.set_status(format!("Gravity G: {:.0}", app.physics_config.gravity_g));
            }
            KeyCode::Char(']') => {
                app.physics_config.gravity_g += 10.0;
                app.set_status(format!("Gravity G: {:.0}", app.physics_config.gravity_g));
            }
            KeyCode::Char('{') => {
                app.physics_config.damping = (app.physics_config.damping - 0.01).max(0.90);
                app.set_status(format!("Drag (damping): {:.3}", app.physics_config.damping));
            }
            KeyCode::Char('}') => {
                app.physics_config.damping = (app.physics_config.damping + 0.005).min(1.0);
                app.set_status(format!("Drag (damping): {:.3}", app.physics_config.damping));
            }
            KeyCode::Char('<') => {
                app.physics_config.time_scale = (app.physics_config.time_scale - 0.2).max(0.2);
                app.set_status(format!("Speed: {:.1}x", app.physics_config.time_scale));
            }
            KeyCode::Char('>') => {
                app.physics_config.time_scale = (app.physics_config.time_scale + 0.2).min(3.0);
                app.set_status(format!("Speed: {:.1}x", app.physics_config.time_scale));
            }
            // Presets
            KeyCode::Char('1') => {
                app.universe = crate::model::Universe::preset_solar_system();
                app.selected_node_id = app.universe.nodes.first().map(|n| n.id);
                app.set_status("Loaded Solar System preset");
            }
            KeyCode::Char('2') => {
                app.universe = crate::model::Universe::preset_three_body();
                app.selected_node_id = app.universe.nodes.first().map(|n| n.id);
                app.set_status("Loaded Three-Body Problem preset");
            }
            // Storage
            KeyCode::Char('s') => {
                app.save_universe();
            }
            KeyCode::Char('m') => {
                app.export_markdown();
            }
            _ => {}
        },
    }
}

fn handle_mouse_event(app: &mut App, mouse: event::MouseEvent) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some((world_x, world_y)) = app.screen_to_world(mouse.column, mouse.row) {
                if let Some(node_id) = app.universe.find_node_near(world_x, world_y, 8.0 / app.zoom) {
                    app.selected_node_id = Some(node_id);
                    app.mode = AppMode::DraggingNode {
                        node_id,
                        last_world_x: world_x,
                        last_world_y: world_y,
                    };
                    app.set_status("Flinging thought...");
                }
            }
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            let coords = app.screen_to_world(mouse.column, mouse.row);
            if let (
                Some((new_x, new_y)),
                AppMode::DraggingNode {
                    node_id,
                    last_world_x,
                    last_world_y,
                },
            ) = (coords, &mut app.mode)
            {
                let vx = (new_x - *last_world_x) * 1.8;
                let vy = (new_y - *last_world_y) * 1.8;

                if let Some(n) = app.universe.get_node_mut(*node_id) {
                    n.x = new_x;
                    n.y = new_y;
                    n.vx = vx;
                    n.vy = vy;
                }

                *last_world_x = new_x;
                *last_world_y = new_y;
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            if let AppMode::DraggingNode { .. } = app.mode {
                app.mode = AppMode::Normal;
                app.set_status("Thought released into orbit!");
            }
        }
        MouseEventKind::ScrollUp => {
            app.zoom = (app.zoom * 1.15).min(5.0);
        }
        MouseEventKind::ScrollDown => {
            app.zoom = (app.zoom / 1.15).max(0.3);
        }
        _ => {}
    }
}
