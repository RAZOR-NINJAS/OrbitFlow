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

        // Planet 1: Inner Shell (r = 55 AU)
        let physics_id = u.add_node("Physics Loop", NodeType::Planet, 47.6, -27.5);
        if let Some(n) = u.get_node_mut(physics_id) {
            n.notes = "🚀 N-Body Gravity & Spring Mechanics\n- Symplectic Verlet integration\n- Coulomb repulsion to prevent overlap\n- Velocity damping (cosmic drag)".into();
            n.tags = vec!["#physics".into(), "#simulation".into()];
            n.vx = 0.85;
            n.vy = 1.47;
        }
        u.connect(sun_id, physics_id, 55.0);

        // Moon 1A for Physics (r = 24 AU from Planet 1)
        let moon_grav = u.add_node("N-Body Gravity", NodeType::Moon, 64.6, -10.5);
        if let Some(n) = u.get_node_mut(moon_grav) {
            n.notes = "F = G * (m1 * m2) / (r^2 + eps^2)".into();
            n.tags = vec!["#math".into()];
            n.vx = 0.4;
            n.vy = 1.9;
        }
        u.connect(physics_id, moon_grav, 24.0);

        // Planet 2: Mid-Inner Shell (r = 95 AU)
        let canvas_id = u.add_node("Braille Canvas", NodeType::Planet, -47.5, 82.3);
        if let Some(n) = u.get_node_mut(canvas_id) {
            n.notes = "🎨 Sub-pixel terminal rendering via Ratatui Canvas\n- 2x4 dots per cell\n- Faint orbital rings\n- Stardust trails behind moving thoughts".into();
            n.tags = vec!["#graphics".into(), "#ratatui".into()];
            n.vx = -1.17;
            n.vy = -0.67;
        }
        u.connect(sun_id, canvas_id, 95.0);

        // Moon 2A for Canvas (r = 26 AU from Planet 2)
        let moon_trail = u.add_node("Stardust Trails", NodeType::Moon, -69.5, 98.3);
        if let Some(n) = u.get_node_mut(moon_trail) {
            n.notes = "Decaying history trail rendered with glowing braille motes.".into();
            n.tags = vec!["#vibe".into()];
            n.vx = -1.5;
            n.vy = -0.3;
        }
        u.connect(canvas_id, moon_trail, 26.0);

        // Planet 3: Mid-Outer Shell (r = 140 AU)
        let scratch_id = u.add_node("Scratchpad UI", NodeType::Planet, -70.0, -121.2);
        if let Some(n) = u.get_node_mut(scratch_id) {
            n.notes = "📝 Side inspector for deep focus.\n- Markdown notes & tags\n- Keybinding cheatsheet\n- Real-time physics parameters".into();
            n.tags = vec!["#ui".into(), "#notes".into()];
            n.vx = 0.99;
            n.vy = -0.57;
        }
        u.connect(sun_id, scratch_id, 140.0);

        // Moon 3A for Scratchpad (r = 28 AU from Planet 3)
        let moon_md = u.add_node("Markdown Export", NodeType::Moon, -46.0, -139.2);
        if let Some(n) = u.get_node_mut(moon_md) {
            n.notes = "Exports the entire orbital galaxy to hierarchical markdown.".into();
            n.tags = vec!["#export".into()];
            n.vx = 1.35;
            n.vy = -0.2;
        }
        u.connect(scratch_id, moon_md, 28.0);

        // Planet 4: Outer Shell (r = 185 AU)
        let control_id = u.add_node("Flight Controls", NodeType::Planet, 178.7, 47.8);
        if let Some(n) = u.get_node_mut(control_id) {
            n.notes = "🎮 Intuitive interactions:\n- WASD flight burns\n- Camera lock\n- Mouse fling".into();
            n.tags = vec!["#controls".into(), "#ux".into()];
            n.vx = -0.26;
            n.vy = 0.96;
        }
        u.connect(sun_id, control_id, 185.0);

        u
    }

    /// Preset 2: Chaotic Three-Body Problem
    pub fn preset_three_body() -> Self {
        let mut u = Self::new();
        let star_a = u.add_node("Alpha Centauri", NodeType::Star, -55.0, -30.0);
        let star_b = u.add_node("Beta Centauri", NodeType::Star, 55.0, -30.0);
        let star_c = u.add_node("Proxima Core", NodeType::Star, 0.0, 65.0);

        if let Some(n) = u.get_node_mut(star_a) {
            n.notes = "Massive binary companion A".into();
            n.pinned = false;
            n.vx = 0.5;
            n.vy = 0.9;
            n.mass = 50.0;
        }
        if let Some(n) = u.get_node_mut(star_b) {
            n.notes = "Massive binary companion B".into();
            n.pinned = false;
            n.vx = -0.5;
            n.vy = -0.9;
            n.mass = 50.0;
        }
        if let Some(n) = u.get_node_mut(star_c) {
            n.notes = "Chaotic perturber".into();
            n.pinned = false;
            n.vx = 0.3;
            n.vy = -0.2;
            n.mass = 45.0;
        }

        // Wandering asteroids caught in the chaos with wide orbits
        for i in 0..6 {
            let angle = (i as f64) * 1.047;
            let r = 42.0;
            let ast = u.add_node(
                format!("Asteroid #{}", i + 1),
                NodeType::Asteroid,
                angle.cos() * r,
                angle.sin() * r,
            );
            if let Some(n) = u.get_node_mut(ast) {
                n.vx = -angle.sin() * 1.5;
                n.vy = angle.cos() * 1.5;
            }
        }

        u.connect(star_a, star_b, 85.0);
        u.connect(star_b, star_c, 85.0);
        u.connect(star_c, star_a, 85.0);

        u
    }

    /// Preset 3: Singularity & Accretion Laboratory
    pub fn preset_singularity_laboratory() -> Self {
        let mut u = Self::new();

        // Central Supermassive Black Hole
        let bh_id = u.add_node("Gargantua Singularity", NodeType::BlackHole, 0.0, 0.0);
        if let Some(n) = u.get_node_mut(bh_id) {
            n.notes = "🕳 Supermassive gravitational singularity.\n- Massive spacetime curvature well\n- Schwarzschild radius event horizon\n- Consumes stray debris crossing inside".into();
            n.tags = vec!["#singularity".into(), "#relativity".into(), "#event-horizon".into()];
            n.pinned = true;
        }

        // Companion Pulsar at r = 85 AU
        let pulsar_id = u.add_node("Pulsar PSR-01", NodeType::Pulsar, -74.0, -42.0);
        if let Some(n) = u.get_node_mut(pulsar_id) {
            n.notes = "⚡ High-frequency neutron pulsar emitting twin relativistic beams across deep space.".into();
            n.tags = vec!["#pulsar".into(), "#jets".into()];
            n.vx = 0.7;
            n.vy = -1.2;
        }
        u.connect(bh_id, pulsar_id, 85.0);

        // Habitable Planet orbiting safely outside ISCO at r = 135 AU
        let world_id = u.add_node("Endurance Hub", NodeType::Planet, 120.0, 62.0);
        if let Some(n) = u.get_node_mut(world_id) {
            n.notes = "🪐 Orbital research station positioned in stable relativistic resonance.".into();
            n.tags = vec!["#research".into(), "#habitable".into()];
            n.vx = -0.55;
            n.vy = 1.05;
        }
        u.connect(bh_id, world_id, 135.0);

        // Task moon orbiting the research station at r = 28 AU
        let probe_id = u.add_node("Deep Survey Probe", NodeType::Moon, 142.0, 78.0);
        if let Some(n) = u.get_node_mut(probe_id) {
            n.notes = "Telemetry sensor gathering tidal gravitational wave flux data.".into();
            n.tags = vec!["#telemetry".into()];
            n.vx = -0.4;
            n.vy = 1.25;
        }
        u.connect(world_id, probe_id, 28.0);

        // Swarm of accretion disk asteroids with wide clearance (r = 50-70 AU)
        for i in 0..8 {
            let angle = (i as f64) * (std::f64::consts::TAU / 8.0);
            let r = 50.0 + (i as f64) * 2.5;
            let ast = u.add_node(
                format!("Disk Debris #{}", i + 1),
                NodeType::Asteroid,
                angle.cos() * r,
                angle.sin() * r,
            );
            if let Some(n) = u.get_node_mut(ast) {
                let speed = 1.6;
                n.vx = -angle.sin() * speed;
                n.vy = angle.cos() * speed;
            }
        }

        u
    }
}
