mod app;
mod model;
mod physics;
mod storage;
mod ui;

use app::{App, AppMode, FocusedPanel, InspectorTab};
use model::NodeType;
use physics::PhysicsEngine;
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

            // Top horizontal layout: Canvas (68%) and Inspector (32%)
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(68), Constraint::Percentage(32)])
                .split(main_chunks[0]);

            app.canvas_area = top_chunks[0];

            // Render Interstellar Braille Canvas
            CanvasRenderer::render(
                frame,
                top_chunks[0],
                &app.universe,
                &app.cosmic_dust,
                &app.physics_config,
                app.camera_x,
                app.camera_y,
                app.zoom,
                app.selected_node_id,
                app.focused_panel == FocusedPanel::Canvas,
                &app.theme,
            );

            // Render Flight Computer & Telemetry Deck with Active Tab
            InspectorRenderer::render(
                frame,
                top_chunks[1],
                &app.universe,
                &app.physics_config,
                app.selected_node_id,
                app.inspector_tab,
                app.camera_locked,
                app.focused_panel == FocusedPanel::Inspector,
                &app.theme,
            );

            // Render HUD
            let mode_str = match &app.mode {
                AppMode::Normal => "FLIGHT_ACTIVE",
                AppMode::Help => "FLIGHT_MANUAL",
                AppMode::QuickActions { .. } => "ACTIONS_MENU",
                AppMode::FleetList { .. } => "FLEET_ROSTER",
                AppMode::NewNode { .. } => "BIRTH_BODY",
                AppMode::EditNote { .. } => "TRANSMIT_LOG",
                AppMode::Connect { .. } => "GRAVITY_LINK",
                AppMode::DraggingNode { .. } => "ORBITAL_FLING",
            };

            HudRenderer::render(
                frame,
                main_chunks[1],
                &app.physics_config,
                mode_str,
                app.universe.nodes.len(),
                app.universe.connections.len(),
                app.camera_locked,
                app.zoom,
                &app.theme,
            );

            // Render Overlays / Modals based on active mode
            match &app.mode {
                AppMode::QuickActions { selected_idx } => {
                    DialogsRenderer::render_quick_actions(frame, area, *selected_idx, &app.theme);
                }
                AppMode::FleetList { selected_idx } => {
                    DialogsRenderer::render_fleet_list(frame, area, &app.universe.nodes, *selected_idx, &app.theme);
                }
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
                        .unwrap_or("Target");
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

    println!("✨ OrbitFlow flight deck disengaged. Keep your thoughts in orbit!");
    Ok(())
}

fn handle_key_event(app: &mut App, code: KeyCode, modifiers: KeyModifiers) {
    match &app.mode {
        AppMode::QuickActions { .. } => {
            if let AppMode::QuickActions { mut selected_idx } =
                std::mem::replace(&mut app.mode, AppMode::Normal)
            {
                match code {
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        selected_idx = if selected_idx > 0 {
                            selected_idx - 1
                        } else {
                            crate::ui::dialogs::QUICK_ACTIONS.len() - 1
                        };
                        app.mode = AppMode::QuickActions { selected_idx };
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        selected_idx = (selected_idx + 1) % crate::ui::dialogs::QUICK_ACTIONS.len();
                        app.mode = AppMode::QuickActions { selected_idx };
                    }
                    KeyCode::Enter => {
                        app.mode = AppMode::Normal;
                        match selected_idx {
                            0 => app.toggle_camera_lock(),
                            1 => {
                                if let Some(id) = app.selected_node_id {
                                    PhysicsEngine::apply_thruster_burn(&mut app.universe, id, 0.55);
                                    app.set_status("🔥 PROGRADE BURN (+Δv: Raising Apoapsis)");
                                }
                            }
                            2 => {
                                if let Some(id) = app.selected_node_id {
                                    PhysicsEngine::apply_thruster_burn(&mut app.universe, id, -0.55);
                                    app.set_status("💨 RETROGRADE BURN (-Δv: Lowering Periapsis)");
                                }
                            }
                            3 => {
                                let initial_idx = app
                                    .selected_node_id
                                    .and_then(|id| app.universe.nodes.iter().position(|n| n.id == id))
                                    .unwrap_or(0);
                                app.mode = AppMode::FleetList { selected_idx: initial_idx };
                            }
                            4 => {
                                app.mode = AppMode::NewNode {
                                    title: String::new(),
                                    node_type: NodeType::Planet,
                                    tags: String::new(),
                                    focused_field: 0,
                                };
                            }
                            5 => {
                                let bh_id = app.universe.add_node("Singularity Core", NodeType::BlackHole, app.camera_x, app.camera_y);
                                app.selected_node_id = Some(bh_id);
                                app.set_status("🕳 SUPERMASSIVE BLACK HOLE BIRTHED AT CAMERA FOCUS!");
                            }
                            6 => {
                                PhysicsEngine::trigger_gravitational_wave(&mut app.universe, app.camera_x, app.camera_y, 45.0);
                                app.set_status("💥 SUPERNOVA GRAVITATIONAL WAVE RADIATED ACROSS THE SECTOR!");
                            }
                            7 => {
                                if let Some(id) = app.selected_node_id {
                                    let current_notes = app.universe.get_node(id).map(|n| n.notes.clone()).unwrap_or_default();
                                    app.mode = AppMode::EditNote { buffer: current_notes };
                                }
                            }
                            8 => {
                                if let Some(source_id) = app.selected_node_id {
                                    let candidates: Vec<(usize, String, NodeType)> = app
                                        .universe
                                        .nodes
                                        .iter()
                                        .filter(|n| n.id != source_id)
                                        .map(|n| (n.id, n.title.clone(), n.node_type))
                                        .collect();
                                    if !candidates.is_empty() {
                                        app.mode = AppMode::Connect { candidates, selected_idx: 0 };
                                    }
                                }
                            }
                            9 => {
                                app.physics_config.show_spacetime_grid = !app.physics_config.show_spacetime_grid;
                                app.set_status(if app.physics_config.show_spacetime_grid { "Spacetime Grid: ACTIVE" } else { "Spacetime Grid: DISABLED" });
                            }
                            10 => {
                                app.physics_config.show_velocity_vectors = !app.physics_config.show_velocity_vectors;
                                app.set_status(if app.physics_config.show_velocity_vectors { "Velocity Vectors: ACTIVE" } else { "Velocity Vectors: DISABLED" });
                            }
                            11 => {
                                app.universe = crate::model::Universe::preset_solar_system();
                                app.selected_node_id = app.universe.nodes.first().map(|n| n.id);
                                app.set_status("Preset 1: Solar System Brainstorm loaded");
                            }
                            12 => {
                                app.universe = crate::model::Universe::preset_three_body();
                                app.selected_node_id = app.universe.nodes.first().map(|n| n.id);
                                app.set_status("Preset 2: Chaotic Three-Body Problem loaded");
                            }
                            13 => {
                                app.universe = crate::model::Universe::preset_singularity_laboratory();
                                app.selected_node_id = app.universe.nodes.first().map(|n| n.id);
                                app.set_status("Preset 3: Gargantua Singularity Laboratory loaded");
                            }
                            14 => app.cycle_theme(),
                            15 => {
                                app.physics_config.paused = !app.physics_config.paused;
                                app.set_status(if app.physics_config.paused { "Simulation PAUSED" } else { "Simulation RUNNING" });
                            }
                            16 => app.save_universe(),
                            17 => app.export_markdown(),
                            18 => app.mode = AppMode::Help,
                            19 => app.should_quit = true,
                            _ => {}
                        }
                    }
                    _ => {
                        app.mode = AppMode::QuickActions { selected_idx };
                    }
                }
            }
        }
        AppMode::FleetList { .. } => {
            if let AppMode::FleetList { mut selected_idx } =
                std::mem::replace(&mut app.mode, AppMode::Normal)
            {
                match code {
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        let count = app.universe.nodes.len();
                        if count > 0 {
                            selected_idx = if selected_idx > 0 { selected_idx - 1 } else { count - 1 };
                            app.mode = AppMode::FleetList { selected_idx };
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let count = app.universe.nodes.len();
                        if count > 0 {
                            selected_idx = (selected_idx + 1) % count;
                            app.mode = AppMode::FleetList { selected_idx };
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(target) = app.universe.nodes.get(selected_idx) {
                            let id = target.id;
                            let title = target.title.clone();
                            let x = target.x;
                            let y = target.y;
                            app.selected_node_id = Some(id);
                            app.camera_x = x;
                            app.camera_y = y;
                            app.camera_locked = true;
                            app.set_status(format!("🎯 Camera Locked on \"{}\"", title));
                        }
                        app.mode = AppMode::Normal;
                    }
                    _ => {
                        app.mode = AppMode::FleetList { selected_idx };
                    }
                }
            }
        }
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
                            NodeType::Pulsar => NodeType::Star,
                            NodeType::BlackHole => NodeType::Pulsar,
                            NodeType::Planet => NodeType::BlackHole,
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
                            NodeType::Star => NodeType::Pulsar,
                            NodeType::Pulsar => NodeType::BlackHole,
                            NodeType::BlackHole => NodeType::Planet,
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
                            let dist: f64 = rng.gen_range(20.0..45.0);
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
                                n.vx = -angle.sin() * 0.9;
                                n.vy = angle.cos() * 0.9;
                            }

                            if let Some(sel_id) = app.selected_node_id {
                                if sel_id != new_id {
                                    app.universe.connect(sel_id, new_id, 32.0);
                                }
                            }

                            app.selected_node_id = Some(new_id);
                            app.set_status(format!("Birthed celestial body: \"{}\"", title));
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
                                    app.set_status("Mission directives logged!");
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
                                app.set_status("Mission directives logged!");
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
                                app.universe.connect(source_id, *target_id, 35.0);
                                app.set_status("Elastic gravitational spring established!");
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

            // PRIMARY EASY ACCESS: Open Quick Actions Menu
            KeyCode::Enter | KeyCode::Char('/') => {
                app.mode = AppMode::QuickActions { selected_idx: 0 };
            }

            // CAMERA LOCK: Auto-tracks currently selected planet
            KeyCode::Char('f') | KeyCode::Char('F') => {
                app.toggle_camera_lock();
            }

            // FLEET ROSTER: Quick picker for all celestial bodies
            KeyCode::Char('l') | KeyCode::Char('L') => {
                let initial_idx = app
                    .selected_node_id
                    .and_then(|id| app.universe.nodes.iter().position(|n| n.id == id))
                    .unwrap_or(0);
                app.mode = AppMode::FleetList { selected_idx: initial_idx };
            }

            // INSPECTOR TABS: 1: Telemetry, 2: Notes, 3: Fleet
            KeyCode::Char('1') => {
                app.inspector_tab = InspectorTab::Telemetry;
                app.set_status("Inspector Tab: [1] Keperian Telemetry");
            }
            KeyCode::Char('2') => {
                app.inspector_tab = InspectorTab::Notes;
                app.set_status("Inspector Tab: [2] Mission Directives / Notes");
            }
            KeyCode::Char('3') => {
                app.inspector_tab = InspectorTab::Fleet;
                app.set_status("Inspector Tab: [3] Fleet Overview");
            }

            // SIMULATION PAUSE
            KeyCode::Char(' ') => {
                app.physics_config.paused = !app.physics_config.paused;
                app.set_status(if app.physics_config.paused {
                    "Cosmic simulation PAUSED"
                } else {
                    "Cosmic simulation RUNNING"
                });
            }
            KeyCode::Tab => {
                app.cycle_selected_node();
            }

            // FLIGHT THRUSTER BURNS
            KeyCode::Char('w') | KeyCode::Char('W') => {
                if let Some(id) = app.selected_node_id {
                    PhysicsEngine::apply_thruster_burn(&mut app.universe, id, 0.55);
                    app.set_status("🔥 PROGRADE BURN (+Δv: Raising Apoapsis Altitude)");
                }
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                if modifiers.contains(KeyModifiers::CONTROL) {
                    app.save_universe();
                } else if let Some(id) = app.selected_node_id {
                    PhysicsEngine::apply_thruster_burn(&mut app.universe, id, -0.55);
                    app.set_status("💨 RETROGRADE BURN (-Δv: Lowering Periapsis Altitude)");
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                if let Some(id) = app.selected_node_id {
                    PhysicsEngine::apply_radial_burn(&mut app.universe, id, -0.45);
                    app.set_status("🚀 RADIAL INWARD BURN (Rotating Orbital Ellipse)");
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if let Some(id) = app.selected_node_id {
                    PhysicsEngine::apply_radial_burn(&mut app.universe, id, 0.45);
                    app.set_status("🚀 RADIAL OUTWARD BURN (Rotating Orbital Ellipse)");
                }
            }

            // VISUAL TOGGLES
            KeyCode::Char('g') => {
                app.physics_config.show_spacetime_grid = !app.physics_config.show_spacetime_grid;
                app.set_status(if app.physics_config.show_spacetime_grid {
                    "Einstein Warped Spacetime Grid: ACTIVE"
                } else {
                    "Spacetime Grid: DISABLED"
                });
            }
            KeyCode::Char('v') => {
                app.physics_config.show_velocity_vectors = !app.physics_config.show_velocity_vectors;
                app.set_status(if app.physics_config.show_velocity_vectors {
                    "Flight Velocity Vector Arrows: ACTIVE"
                } else {
                    "Velocity Vectors: DISABLED"
                });
            }
            KeyCode::Char('o') => {
                app.physics_config.show_orbit_paths = !app.physics_config.show_orbit_paths;
                app.set_status(if app.physics_config.show_orbit_paths {
                    "Predicted Keplerian Orbital Rings: ACTIVE"
                } else {
                    "Orbital Rings: DISABLED"
                });
            }

            // COSMIC PHENOMENA
            KeyCode::Char('b') => {
                let bh_id = app.universe.add_node("Singularity Core", NodeType::BlackHole, app.camera_x, app.camera_y);
                app.selected_node_id = Some(bh_id);
                app.set_status("🕳 SUPERMASSIVE BLACK HOLE BIRTHED AT CAMERA FOCUS!");
            }
            KeyCode::Char('k') => {
                PhysicsEngine::trigger_gravitational_wave(&mut app.universe, app.camera_x, app.camera_y, 45.0);
                app.set_status("💥 SUPERNOVA GRAVITATIONAL WAVE RADIATED ACROSS THE SECTOR!");
            }

            // CAMERA PANNING (Disables Camera Lock if manually moved)
            KeyCode::Left | KeyCode::Char('h') => {
                app.camera_locked = false;
                app.camera_x -= 6.0 / app.zoom;
            }
            KeyCode::Right => {
                app.camera_locked = false;
                app.camera_x += 6.0 / app.zoom;
            }
            KeyCode::Up => {
                app.camera_locked = false;
                app.camera_y += 6.0 / app.zoom;
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.camera_locked = false;
                app.camera_y -= 6.0 / app.zoom;
            }

            // ZOOM
            KeyCode::Char('+') | KeyCode::Char('=') => {
                app.zoom = (app.zoom * 1.15).min(5.0);
            }
            KeyCode::Char('-') => {
                app.zoom = (app.zoom / 1.15).max(0.3);
            }
            KeyCode::Char('0') => {
                app.camera_locked = false;
                app.camera_x = 0.0;
                app.camera_y = 0.0;
                app.zoom = 1.0;
                app.set_status("Camera centered on galactic origin (0, 0)");
            }

            KeyCode::Char('p') => {
                app.toggle_pin_selected();
            }
            KeyCode::Delete => {
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
                        app.set_status("Need at least 2 bodies to establish gravity link");
                    }
                }
            }

            // FUNCTION KEYS FOR PRESETS
            KeyCode::F(1) => {
                app.universe = crate::model::Universe::preset_solar_system();
                app.selected_node_id = app.universe.nodes.first().map(|n| n.id);
                app.set_status("Preset 1: Solar System Brainstorm loaded");
            }
            KeyCode::F(2) => {
                app.universe = crate::model::Universe::preset_three_body();
                app.selected_node_id = app.universe.nodes.first().map(|n| n.id);
                app.set_status("Preset 2: Chaotic Three-Body Problem loaded");
            }
            KeyCode::F(3) => {
                app.universe = crate::model::Universe::preset_singularity_laboratory();
                app.selected_node_id = app.universe.nodes.first().map(|n| n.id);
                app.set_status("Preset 3: Gargantua Singularity Laboratory loaded");
            }

            // Export Markdown
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
                    app.set_status("🛸 Orbital sling engaged...");
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
                app.set_status("🚀 Slingshot released into orbit with momentum!");
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
