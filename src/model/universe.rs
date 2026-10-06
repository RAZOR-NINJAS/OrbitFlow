use super::connection::Connection;
use super::node::{Node, NodeType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Universe {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
    pub next_id: usize,
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            connections: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_node(&mut self, title: impl Into<String>, node_type: NodeType, x: f64, y: f64) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        let node = Node::new(id, title, node_type, x, y);
        self.nodes.push(node);
        id
    }

    pub fn remove_node(&mut self, id: usize) {
        self.nodes.retain(|n| n.id != id);
        self.connections.retain(|c| c.from_id != id && c.to_id != id);
    }

    pub fn connect(&mut self, from_id: usize, to_id: usize, rest_length: f64) {
        if from_id == to_id {
            return;
        }
        // Avoid duplicate connection
        if !self.connections.iter().any(|c| {
            (c.from_id == from_id && c.to_id == to_id) || (c.from_id == to_id && c.to_id == from_id)
        }) {
            self.connections.push(Connection::new(from_id, to_id, rest_length));
        }
    }

    #[allow(dead_code)]
    pub fn disconnect(&mut self, from_id: usize, to_id: usize) {
        self.connections.retain(|c| {
            !((c.from_id == from_id && c.to_id == to_id) || (c.from_id == to_id && c.to_id == from_id))
        });
    }

    pub fn get_node(&self, id: usize) -> Option<&Node> {
        self.nodes.iter().find(|n| n.id == id)
    }

    pub fn get_node_mut(&mut self, id: usize) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    pub fn find_node_near(&self, world_x: f64, world_y: f64, tolerance: f64) -> Option<usize> {
        self.nodes
            .iter()
            .filter_map(|n| {
                let dx = n.x - world_x;
                let dy = n.y - world_y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist <= tolerance.max(n.radius * 2.0) {
                    Some((n.id, dist))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(id, _)| id)
    }

    /// Default template: A vibrant cosmic mindmap for OrbitFlow
    pub fn preset_solar_system() -> Self {
        let mut u = Self::new();

        // Central Sun
        let sun_id = u.add_node("OrbitFlow Core", NodeType::Star, 0.0, 0.0);
        if let Some(n) = u.get_node_mut(sun_id) {
            n.notes = "🌟 The heart of the galaxy.\nCoordinates real-time physics with terminal rendering.\n- Pinned in space\n- Exerts central gravitational pull".into();
            n.tags = vec!["#arch".into(), "#core".into(), "#rust".into()];
            n.pinned = true;
        }

        // Planet 1: Physics Engine
        let physics_id = u.add_node("Physics Loop", NodeType::Planet, -35.0, -15.0);
        if let Some(n) = u.get_node_mut(physics_id) {
            n.notes = "🚀 N-Body Gravity & Spring Mechanics\n- Symplectic Verlet integration\n- Coulomb repulsion to prevent overlap\n- Velocity damping (cosmic drag)".into();
            n.tags = vec!["#physics".into(), "#simulation".into()];
            n.vx = 0.5;
            n.vy = -1.2;
        }
        u.connect(sun_id, physics_id, 38.0);

        // Moon 1A for Physics
        let moon_grav = u.add_node("N-Body Gravity", NodeType::Moon, -46.0, -24.0);
        if let Some(n) = u.get_node_mut(moon_grav) {
            n.notes = "F = G * (m1 * m2) / (r^2 + eps^2)".into();
            n.tags = vec!["#math".into()];
            n.vx = 0.3;
            n.vy = -1.0;
        }
        u.connect(physics_id, moon_grav, 14.0);

        // Planet 2: Braille Canvas
        let canvas_id = u.add_node("Braille Canvas", NodeType::Planet, 32.0, -18.0);
        if let Some(n) = u.get_node_mut(canvas_id) {
            n.notes = "🎨 Sub-pixel terminal rendering via Ratatui Canvas\n- 2x4 dots per cell\n- Faint orbital rings\n- Stardust trails behind moving thoughts".into();
            n.tags = vec!["#graphics".into(), "#ratatui".into()];
            n.vx = -0.4;
            n.vy = 1.3;
        }
        u.connect(sun_id, canvas_id, 36.0);

        // Moon 2A for Canvas
        let moon_trail = u.add_node("Stardust Trails", NodeType::Moon, 44.0, -25.0);
        if let Some(n) = u.get_node_mut(moon_trail) {
            n.notes = "Decaying history trail rendered with glowing braille motes.".into();
            n.tags = vec!["#vibe".into()];
            n.vx = -0.2;
            n.vy = 1.0;
        }
        u.connect(canvas_id, moon_trail, 13.0);

        // Planet 3: Scratchpad & Notes
        let scratch_id = u.add_node("Scratchpad UI", NodeType::Planet, 22.0, 32.0);
        if let Some(n) = u.get_node_mut(scratch_id) {
            n.notes = "📝 Side inspector for deep focus.\n- Markdown notes & tags\n- Keybinding cheatsheet\n- Real-time physics parameters".into();
            n.tags = vec!["#ui".into(), "#notes".into()];
            n.vx = -1.1;
            n.vy = -0.6;
        }
        u.connect(sun_id, scratch_id, 38.0);

        // Moon 3A for Scratchpad
        let moon_md = u.add_node("Markdown Export", NodeType::Moon, 32.0, 42.0);
        if let Some(n) = u.get_node_mut(moon_md) {
            n.notes = "Exports the entire orbital galaxy to hierarchical markdown.".into();
            n.tags = vec!["#export".into()];
            n.vx = -0.8;
            n.vy = -0.4;
        }
        u.connect(scratch_id, moon_md, 14.0);

        // Planet 4: Spatial Controls
        let control_id = u.add_node("Input & Camera", NodeType::Planet, -28.0, 26.0);
        if let Some(n) = u.get_node_mut(control_id) {
            n.notes = "🎮 Intuitive interactions:\n- Mouse left drag to fling thoughts\n- Vim keys (hjkl) to pan camera\n- Zoom with +/- or scroll wheel".into();
            n.tags = vec!["#controls".into(), "#ux".into()];
            n.vx = 1.0;
            n.vy = 0.7;
        }
        u.connect(sun_id, control_id, 38.0);

        // Cross-domain connection between Physics and Canvas
        u.connect(physics_id, canvas_id, 55.0);

        u
    }

    /// Preset 2: Chaotic Three-Body Problem
    pub fn preset_three_body() -> Self {
        let mut u = Self::new();
        let star_a = u.add_node("Alpha Centauri", NodeType::Star, -25.0, -10.0);
        let star_b = u.add_node("Beta Centauri", NodeType::Star, 25.0, -10.0);
        let star_c = u.add_node("Proxima Core", NodeType::Star, 0.0, 30.0);

        if let Some(n) = u.get_node_mut(star_a) {
            n.notes = "Massive binary companion A".into();
            n.pinned = false;
            n.vx = 0.4;
            n.vy = 0.8;
            n.mass = 50.0;
        }
        if let Some(n) = u.get_node_mut(star_b) {
            n.notes = "Massive binary companion B".into();
            n.pinned = false;
            n.vx = -0.4;
            n.vy = -0.8;
            n.mass = 50.0;
        }
        if let Some(n) = u.get_node_mut(star_c) {
            n.notes = "Chaotic perturber".into();
            n.pinned = false;
            n.vx = 0.2;
            n.vy = -0.2;
            n.mass = 45.0;
        }

        // Wandering asteroids caught in the chaos
        for i in 0..6 {
            let angle = (i as f64) * 1.047; // 60 degrees
            let r = 18.0;
            let ast = u.add_node(
                format!("Asteroid #{}", i + 1),
                NodeType::Asteroid,
                angle.cos() * r,
                angle.sin() * r,
            );
            if let Some(n) = u.get_node_mut(ast) {
                n.vx = -angle.sin() * 1.2;
                n.vy = angle.cos() * 1.2;
            }
        }

        u.connect(star_a, star_b, 45.0);
        u.connect(star_b, star_c, 45.0);
        u.connect(star_c, star_a, 45.0);

        u
    }
}
