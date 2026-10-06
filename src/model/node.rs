use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Star,      // Radiant anchor thought
    Pulsar,    // Rapidly pulsating star with relativistic magnetic jets
    BlackHole, // Gravitational singularity with an event horizon
    Planet,    // Major conceptual module
    Moon,      // Subtask / detail bound to a planet
    Asteroid,  // Free-floating debris / stray idea
}

impl NodeType {
    pub fn display_badge(&self) -> &'static str {
        match self {
            NodeType::Star => "★ STAR",
            NodeType::Pulsar => "⚡ PULSAR",
            NodeType::BlackHole => "🕳 SINGULARITY",
            NodeType::Planet => "● PLANET",
            NodeType::Moon => "◦ MOON",
            NodeType::Asteroid => "· DUST",
        }
    }

    pub fn default_mass(&self) -> f64 {
        match self {
            NodeType::Star => 80.0,
            NodeType::Pulsar => 120.0,
            NodeType::BlackHole => 220.0,
            NodeType::Planet => 24.0,
            NodeType::Moon => 8.0,
            NodeType::Asteroid => 3.0,
        }
    }

    pub fn default_radius(&self) -> f64 {
        match self {
            NodeType::Star => 3.2,
            NodeType::Pulsar => 2.4,
            NodeType::BlackHole => 4.0,
            NodeType::Planet => 2.0,
            NodeType::Moon => 1.2,
            NodeType::Asteroid => 0.8,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub title: String,
    pub notes: String,
    pub node_type: NodeType,
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    #[serde(skip)]
    pub fx: f64,
    #[serde(skip)]
    pub fy: f64,
    pub mass: f64,
    pub radius: f64,
    pub pinned: bool,
    pub tags: Vec<String>,
    pub color_idx: usize,
    #[serde(skip)]
    pub trail: VecDeque<(f64, f64)>,
    #[serde(skip)]
    pub jet_angle: f64, // For Pulsar relativistic jets
}

impl Node {
    pub const MAX_TRAIL: usize = 28;

    pub fn new(id: usize, title: impl Into<String>, node_type: NodeType, x: f64, y: f64) -> Self {
        let mass = node_type.default_mass();
        let radius = node_type.default_radius();
        Self {
            id,
            title: title.into(),
            notes: String::new(),
            node_type,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            fx: 0.0,
            fy: 0.0,
            mass,
            radius,
            pinned: matches!(node_type, NodeType::Star | NodeType::BlackHole | NodeType::Pulsar),
            tags: Vec::new(),
            color_idx: 0,
            trail: VecDeque::with_capacity(Self::MAX_TRAIL),
            jet_angle: 0.0,
        }
    }

    #[allow(dead_code)]
    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = notes.into();
        self
    }

    #[allow(dead_code)]
    pub fn with_velocity(mut self, vx: f64, vy: f64) -> Self {
        self.vx = vx;
        self.vy = vy;
        self
    }

    #[allow(dead_code)]
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    #[allow(dead_code)]
    pub fn with_pinned(mut self, pinned: bool) -> Self {
        self.pinned = pinned;
        self
    }

    pub fn clear_forces(&mut self) {
        self.fx = 0.0;
        self.fy = 0.0;
    }

    pub fn record_trail(&mut self) {
        if self.trail.len() >= Self::MAX_TRAIL {
            self.trail.pop_front();
        }
        self.trail.push_back((self.x, self.y));
    }
}
