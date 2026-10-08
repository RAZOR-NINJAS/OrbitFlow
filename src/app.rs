use crate::model::{NodeType, Universe};
use crate::physics::{CosmicDust, PhysicsConfig, PhysicsEngine};
use crate::storage::StorageManager;
use crate::ui::theme::Theme;
use ratatui::layout::Rect;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FocusedPanel {
    Canvas,
    Inspector,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InspectorTab {
    Telemetry,
    Notes,
    Fleet,
}

#[derive(Debug, Clone)]
pub enum AppMode {
    Normal,
    Help,
    QuickActions {
        selected_idx: usize,
    },
    FleetList {
        selected_idx: usize,
    },
    NewNode {
        title: String,
        node_type: NodeType,
        tags: String,
        focused_field: usize,
    },
    EditNote {
        buffer: String,
    },
    Connect {
        candidates: Vec<(usize, String, NodeType)>,
        selected_idx: usize,
    },
    DraggingNode {
        node_id: usize,
        last_world_x: f64,
        last_world_y: f64,
    },
}

pub struct App {
    pub universe: Universe,
    pub physics_config: PhysicsConfig,
    pub cosmic_dust: CosmicDust,
    pub theme: Theme,
    pub theme_idx: usize,
    pub camera_x: f64,
    pub camera_y: f64,
    pub zoom: f64,
    pub camera_locked: bool,
    pub selected_node_id: Option<usize>,
    pub focused_panel: FocusedPanel,
    pub inspector_tab: InspectorTab,
    pub mode: AppMode,
    pub canvas_area: Rect,
    pub status_message: Option<(String, Instant)>,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let universe = Universe::preset_solar_system();
        let selected_node_id = universe.nodes.first().map(|n| n.id);

        Self {
            universe,
            physics_config: PhysicsConfig::default(),
            cosmic_dust: CosmicDust::new(50),
            theme: Theme::cyberpunk(),
            theme_idx: 0,
            camera_x: 0.0,
            camera_y: 0.0,
            zoom: 0.40,
            camera_locked: false,
            selected_node_id,
            focused_panel: FocusedPanel::Canvas,
            inspector_tab: InspectorTab::Telemetry,
            mode: AppMode::Normal,
            canvas_area: Rect::default(),
            status_message: Some((
                "🪐 Welcome! Press [Enter] for Quick Actions menu, [F] to lock camera, [?] for help".into(),
                Instant::now(),
            )),
            should_quit: false,
        }
    }

    pub fn tick(&mut self, dt: f64) {
        PhysicsEngine::step(&mut self.universe, dt, &mut self.physics_config);
        self.cosmic_dust.step(dt);

        // Camera lock tracking
        if self.camera_locked {
            if let Some(id) = self.selected_node_id {
                if let Some(node) = self.universe.get_node(id) {
                    self.camera_x = node.x;
                    self.camera_y = node.y;
                }
            }
        }

        // Expire status message after 4 seconds
        if let Some((_, time)) = self.status_message {
            if time.elapsed().as_secs() > 4 {
                self.status_message = None;
            }
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>) {
        self.status_message = Some((msg.into(), Instant::now()));
    }

    pub fn toggle_camera_lock(&mut self) {
        self.camera_locked = !self.camera_locked;
        if self.camera_locked {
            if let Some(id) = self.selected_node_id {
                if let Some(node) = self.universe.get_node(id) {
                    self.camera_x = node.x;
                    self.camera_y = node.y;
                    self.set_status(format!("🎯 Camera LOCKED on \"{}\"", node.title));
                    return;
                }
            }
            self.set_status("🎯 Camera Lock: ACTIVE");
        } else {
            self.set_status("🔓 Camera Lock: RELEASED (Free Flight)");
        }
    }

    pub fn cycle_theme(&mut self) {
        self.theme_idx = (self.theme_idx + 1) % 3;
        self.theme = match self.theme_idx {
            0 => Theme::cyberpunk(),
            1 => Theme::catppuccin_mocha(),
            _ => Theme::retro_amber(),
        };
        self.set_status(format!("Theme: {}", self.theme.name));
    }

    pub fn cycle_selected_node(&mut self) {
        if self.universe.nodes.is_empty() {
            self.selected_node_id = None;
            return;
        }

        let next_id = match self.selected_node_id {
            Some(cur_id) => {
                if let Some(pos) = self.universe.nodes.iter().position(|n| n.id == cur_id) {
                    let next_pos = (pos + 1) % self.universe.nodes.len();
                    self.universe.nodes[next_pos].id
                } else {
                    self.universe.nodes[0].id
                }
            }
            None => self.universe.nodes[0].id,
        };

        self.selected_node_id = Some(next_id);
        let target_info = self.universe.get_node(next_id).map(|n| (n.title.clone(), n.x, n.y));
        if let Some((title, x, y)) = target_info {
            self.set_status(format!("Target: \"{}\"", title));
            if self.camera_locked {
                self.camera_x = x;
                self.camera_y = y;
            }
        }
    }

    #[allow(dead_code)]
    pub fn focus_camera_on_selected(&mut self) {
        if let Some(id) = self.selected_node_id {
            if let Some(node) = self.universe.get_node(id) {
                self.camera_x = node.x;
                self.camera_y = node.y;
                self.set_status(format!("Centered on \"{}\"", node.title));
            }
        }
    }

    pub fn toggle_pin_selected(&mut self) {
        if let Some(id) = self.selected_node_id {
            if let Some(node) = self.universe.get_node_mut(id) {
                node.pinned = !node.pinned;
                let pinned = node.pinned;
                self.set_status(if pinned { "Node pinned as Anchor" } else { "Node unpinned" });
            }
        }
    }

    pub fn delete_selected_node(&mut self) {
        if let Some(id) = self.selected_node_id {
            self.universe.remove_node(id);
            self.selected_node_id = self.universe.nodes.first().map(|n| n.id);
            self.set_status("Node erased from cosmos");
        }
    }

    pub fn save_universe(&mut self) {
        match StorageManager::save(&self.universe, StorageManager::DEFAULT_SAVE_PATH) {
            Ok(_) => self.set_status(format!("Saved to {}", StorageManager::DEFAULT_SAVE_PATH)),
            Err(e) => self.set_status(format!("Save error: {}", e)),
        }
    }

    pub fn export_markdown(&mut self) {
        match StorageManager::export_markdown(&self.universe, StorageManager::DEFAULT_EXPORT_PATH) {
            Ok(_) => self.set_status(format!("Exported to {}", StorageManager::DEFAULT_EXPORT_PATH)),
            Err(e) => self.set_status(format!("Export error: {}", e)),
        }
    }

    pub fn screen_to_world(&self, col: u16, row: u16) -> Option<(f64, f64)> {
        let area = self.canvas_area;
        if area.width <= 2 || area.height <= 2 {
            return None;
        }

        if col < area.x || col >= area.x + area.width || row < area.y || row >= area.y + area.height {
            return None;
        }

        let aspect = (area.width as f64) / (area.height as f64 * 2.0);
        let half_h = 45.0 / self.zoom;
        let half_w = half_h * aspect;

        let min_x = self.camera_x - half_w;
        let max_x = self.camera_x + half_w;
        let min_y = self.camera_y - half_h;
        let max_y = self.camera_y + half_h;

        let u = (col.saturating_sub(area.x) as f64) / (area.width as f64);
        let v = 1.0 - (row.saturating_sub(area.y) as f64) / (area.height as f64);

        let world_x = min_x + u * (max_x - min_x);
        let world_y = min_y + v * (max_y - min_y);

        Some((world_x, world_y))
    }
}
