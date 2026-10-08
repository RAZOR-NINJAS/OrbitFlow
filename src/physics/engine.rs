use crate::model::{NodeType, Universe};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OrbitalTelemetry {
    pub primary_name: String,
    pub altitude: f64,
    pub speed: f64,
    pub eccentricity: f64,
    pub orbit_type: &'static str,
    pub semi_major_axis: f64,
    pub periapsis: f64,
    pub apoapsis: f64,
    pub period_sec: f64,
    pub specific_energy: f64,
    pub angular_momentum: f64,
}

#[derive(Debug, Clone)]
pub struct PhysicsConfig {
    pub gravity_g: f64,
    pub repulsion_k: f64,
    pub spring_k: f64,
    pub damping: f64,
    pub time_scale: f64,
    pub paused: bool,
    pub show_spacetime_grid: bool,
    pub show_velocity_vectors: bool,
    pub show_orbit_paths: bool,
    pub frame_counter: usize,
    pub sim_time: f64,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity_g: 45.0,
            repulsion_k: 1400.0,
            spring_k: 0.06,
            damping: 0.9992, // Near-frictionless space: preserves orbital radii
            time_scale: 1.0,
            paused: false,
            show_spacetime_grid: true,
            show_velocity_vectors: true,
            show_orbit_paths: true,
            frame_counter: 0,
            sim_time: 0.0,
        }
    }
}

pub struct PhysicsEngine;

impl PhysicsEngine {
    pub const EPSILON_SQ: f64 = 36.0;
    pub const MAX_VELOCITY: f64 = 8.0;

    pub fn step(universe: &mut Universe, dt: f64, config: &mut PhysicsConfig) {
        if config.paused {
            return;
        }

        config.frame_counter = config.frame_counter.wrapping_add(1);
        config.sim_time += dt * config.time_scale;
        let effective_dt = (dt * config.time_scale).clamp(0.001, 0.04);
        let dt_sim = effective_dt * 2.5;

        // 1. Reset forces
        for node in &mut universe.nodes {
            node.clear_forces();
            // Rotate pulsar relativistic jet
            if node.node_type == NodeType::Pulsar {
                node.jet_angle = (node.jet_angle + 0.12) % std::f64::consts::TAU;
            }
        }

        let n = universe.nodes.len();

        // 2. Pairwise N-Body Gravity & Coulomb Repulsion
        for i in 0..n {
            for j in (i + 1)..n {
                let dx = universe.nodes[j].x - universe.nodes[i].x;
                let dy = universe.nodes[j].y - universe.nodes[i].y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt().max(0.1);

                let nx = dx / dist;
                let ny = dy / dist;

                let m1 = universe.nodes[i].mass;
                let m2 = universe.nodes[j].mass;
                let f_grav = (config.gravity_g * m1 * m2) / (dist_sq + Self::EPSILON_SQ);

                // Anti-clumping repulsion buffer: gives nodes ~28-36 units of breathing room
                let is_bh = universe.nodes[i].node_type == NodeType::BlackHole
                    || universe.nodes[j].node_type == NodeType::BlackHole;

                let min_space = (universe.nodes[i].radius + universe.nodes[j].radius) * 3.5 + 28.0;
                let f_rep = if dist < min_space && !is_bh {
                    let overlap = min_space - dist;
                    (config.repulsion_k * overlap * (1.0 + overlap * 0.1)) / (dist.powi(2) + 1.0)
                } else {
                    0.0
                };

                let net_force = f_grav - f_rep;

                universe.nodes[i].fx += net_force * nx;
                universe.nodes[i].fy += net_force * ny;

                universe.nodes[j].fx -= net_force * nx;
                universe.nodes[j].fy -= net_force * ny;
            }
        }

        // 3. Elastic Spring Forces
        for conn in &mut universe.connections {
            let from_pos = universe.nodes.iter().find(|n| n.id == conn.from_id).map(|n| (n.x, n.y));
            let to_pos = universe.nodes.iter().find(|n| n.id == conn.to_id).map(|n| (n.x, n.y));

            if let (Some((x1, y1)), Some((x2, y2))) = (from_pos, to_pos) {
                let dx = x2 - x1;
                let dy = y2 - y1;
                let dist = (dx * dx + dy * dy).sqrt().max(0.001);
                let delta = dist - conn.rest_length;
                conn.current_tension = delta / conn.rest_length.max(1.0);

                let nx = dx / dist;
                let ny = dy / dist;
                let f_spring = conn.strength * delta * config.spring_k * 10.0;

                if let Some(n1) = universe.nodes.iter_mut().find(|n| n.id == conn.from_id) {
                    n1.fx += f_spring * nx;
                    n1.fy += f_spring * ny;
                }
                if let Some(n2) = universe.nodes.iter_mut().find(|n| n.id == conn.to_id) {
                    n2.fx -= f_spring * nx;
                    n2.fy -= f_spring * ny;
                }
            }
        }

        // 4. Numerical Integration
        let record_trail = config.frame_counter % 2 == 0;

        for node in &mut universe.nodes {
            if node.pinned {
                node.vx = 0.0;
                node.vy = 0.0;
                continue;
            }

            let inv_m = 1.0 / node.mass.max(0.1);
            let ax = node.fx * inv_m;
            let ay = node.fy * inv_m;

            node.vx = (node.vx + ax * dt_sim) * config.damping;
            node.vy = (node.vy + ay * dt_sim) * config.damping;

            let speed = (node.vx * node.vx + node.vy * node.vy).sqrt();
            if speed > Self::MAX_VELOCITY {
                let factor = Self::MAX_VELOCITY / speed;
                node.vx *= factor;
                node.vy *= factor;
            }

            node.x += node.vx * dt_sim * 6.0;
            node.y += node.vy * dt_sim * 6.0;

            if record_trail {
                node.record_trail();
            }
        }

        // 5. Black Hole Event Horizon Absorption (consume stray asteroids that cross inside)
        let mut absorbed_ids = Vec::new();
        let bh_list: Vec<(f64, f64, f64)> = universe
            .nodes
            .iter()
            .filter(|n| n.node_type == NodeType::BlackHole)
            .map(|n| (n.x, n.y, n.radius * 1.5))
            .collect();

        for bh in &bh_list {
            for node in &universe.nodes {
                if node.node_type == NodeType::Asteroid {
                    let dx = node.x - bh.0;
                    let dy = node.y - bh.1;
                    if (dx * dx + dy * dy).sqrt() < bh.2 {
                        absorbed_ids.push(node.id);
                    }
                }
            }
        }

        for id in absorbed_ids {
            universe.remove_node(id);
        }
    }

    /// Calculate real Einsteinian gravitational potential depth at (x, y)
    pub fn gravitational_potential(universe: &Universe, x: f64, y: f64, g: f64) -> f64 {
        let mut pot = 0.0;
        for n in &universe.nodes {
            let dx = x - n.x;
            let dy = y - n.y;
            let dist_sq = dx * dx + dy * dy;
            pot += (g * n.mass) / (dist_sq + Self::EPSILON_SQ).sqrt();
        }
        pot
    }

    /// Compute live Keplerian astrodynamics telemetry for a given node
    pub fn compute_telemetry(universe: &Universe, node_id: usize, g: f64) -> Option<OrbitalTelemetry> {
        let node = universe.get_node(node_id)?;

        // Find primary gravitational anchor (heaviest star or black hole)
        let primary = universe
            .nodes
            .iter()
            .filter(|n| n.id != node_id && matches!(n.node_type, NodeType::Star | NodeType::BlackHole | NodeType::Pulsar))
            .max_by(|a, b| a.mass.partial_cmp(&b.mass).unwrap_or(std::cmp::Ordering::Equal))?;

        let rx = node.x - primary.x;
        let ry = node.y - primary.y;
        let r = (rx * rx + ry * ry).sqrt().max(0.1);

        let vx = node.vx - primary.vx;
        let vy = node.vy - primary.vy;
        let v_sq = vx * vx + vy * vy;
        let speed = v_sq.sqrt();

        let mu = (g * primary.mass).max(1.0);
        let specific_energy = (v_sq / 2.0) - (mu / r);
        let angular_momentum = (rx * vy - ry * vx).abs();

        let semi_major_axis = if specific_energy.abs() > 0.001 {
            -mu / (2.0 * specific_energy)
        } else {
            r
        };

        let ecc_arg = 1.0 + (2.0 * specific_energy * angular_momentum * angular_momentum) / (mu * mu);
        let eccentricity = ecc_arg.max(0.0).sqrt();

        let (orbit_type, periapsis, apoapsis, period_sec) = if eccentricity < 0.15 {
            ("CIRCULAR", r * 0.95, r * 1.05, 2.0 * std::f64::consts::PI * (r.powi(3) / mu).sqrt())
        } else if eccentricity < 1.0 {
            let a = semi_major_axis.abs();
            let p = a * (1.0 - eccentricity);
            let ap = a * (1.0 + eccentricity);
            let period = 2.0 * std::f64::consts::PI * (a.powi(3) / mu).sqrt();
            ("ELLIPTICAL", p, ap, period)
        } else {
            ("HYPERBOLIC (ESCAPE)", r * (1.0 - (eccentricity - 1.0).min(0.5)), f64::INFINITY, f64::INFINITY)
        };

        Some(OrbitalTelemetry {
            primary_name: primary.title.clone(),
            altitude: r,
            speed,
            eccentricity,
            orbit_type,
            semi_major_axis,
            periapsis,
            apoapsis,
            period_sec,
            specific_energy,
            angular_momentum,
        })
    }

    /// Apply thruster impulse burn along prograde (forward) or retrograde (backward)
    pub fn apply_thruster_burn(universe: &mut Universe, node_id: usize, delta_v: f64) {
        if let Some(node) = universe.get_node_mut(node_id) {
            if node.pinned {
                return;
            }
            let speed = (node.vx * node.vx + node.vy * node.vy).sqrt();
            if speed > 0.01 {
                let nx = node.vx / speed;
                let ny = node.vy / speed;
                node.vx += nx * delta_v;
                node.vy += ny * delta_v;
            } else {
                node.vx += delta_v;
            }
        }
    }

    /// Apply radial thruster burn (inward/outward) to rotate orbital periapsis
    pub fn apply_radial_burn(universe: &mut Universe, node_id: usize, delta_v: f64) {
        if let Some(node) = universe.get_node_mut(node_id) {
            if node.pinned {
                return;
            }
            let speed = (node.vx * node.vx + node.vy * node.vy).sqrt();
            if speed > 0.01 {
                // Perpendicular vector (-vy, vx)
                let nx = -node.vy / speed;
                let ny = node.vx / speed;
                node.vx += nx * delta_v;
                node.vy += ny * delta_v;
            }
        }
    }

    /// Trigger gravitational shockwave from center that radiates outward
    pub fn trigger_gravitational_wave(universe: &mut Universe, cx: f64, cy: f64, strength: f64) {
        for node in &mut universe.nodes {
            if node.pinned {
                continue;
            }
            let dx = node.x - cx;
            let dy = node.y - cy;
            let dist = (dx * dx + dy * dy).sqrt().max(1.0);
            let nx = dx / dist;
            let ny = dy / dist;
            let impulse = strength / (dist * 0.1 + 1.0);
            node.vx += nx * impulse;
            node.vy += ny * impulse;
        }
    }
}
