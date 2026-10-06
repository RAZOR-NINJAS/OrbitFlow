use crate::model::Universe;

#[derive(Debug, Clone)]
pub struct PhysicsConfig {
    pub gravity_g: f64,
    pub repulsion_k: f64,
    pub spring_k: f64,
    pub damping: f64,
    pub time_scale: f64,
    pub paused: bool,
    pub frame_counter: usize,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            gravity_g: 55.0,
            repulsion_k: 450.0,
            spring_k: 0.12,
            damping: 0.982,
            time_scale: 1.0,
            paused: false,
            frame_counter: 0,
        }
    }
}

pub struct PhysicsEngine;

impl PhysicsEngine {
    const EPSILON_SQ: f64 = 25.0; // Softening parameter to prevent gravitational singularities
    const MAX_VELOCITY: f64 = 8.0;

    pub fn step(universe: &mut Universe, dt: f64, config: &mut PhysicsConfig) {
        if config.paused {
            return;
        }

        config.frame_counter = config.frame_counter.wrapping_add(1);
        let effective_dt = (dt * config.time_scale).clamp(0.001, 0.05);

        // 1. Reset forces
        for node in &mut universe.nodes {
            node.clear_forces();
        }

        let n = universe.nodes.len();

        // 2. Pairwise N-Body Gravity and Coulomb Repulsion
        for i in 0..n {
            for j in (i + 1)..n {
                let dx = universe.nodes[j].x - universe.nodes[i].x;
                let dy = universe.nodes[j].y - universe.nodes[i].y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt().max(0.1);

                let nx = dx / dist;
                let ny = dy / dist;

                // Gravitational pull: F = G * m1 * m2 / (dist^2 + eps^2)
                let m1 = universe.nodes[i].mass;
                let m2 = universe.nodes[j].mass;
                let f_grav = (config.gravity_g * m1 * m2) / (dist_sq + Self::EPSILON_SQ);

                // Coulomb Repulsion to prevent stacking
                let min_space = (universe.nodes[i].radius + universe.nodes[j].radius) * 4.5;
                let f_rep = if dist < min_space {
                    let overlap = min_space - dist;
                    (config.repulsion_k * overlap) / (dist + 1.0)
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

        // 3. Hooke's Elastic Spring Forces for Connected Nodes
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

        // 4. Numerical Integration (Symplectic Euler with Cosmic Drag)
        let record_trail = config.frame_counter % 3 == 0;

        for node in &mut universe.nodes {
            if node.pinned {
                node.vx = 0.0;
                node.vy = 0.0;
                continue;
            }

            let inv_m = 1.0 / node.mass.max(0.1);
            let ax = node.fx * inv_m;
            let ay = node.fy * inv_m;

            // Velocity update with cosmic damping
            node.vx = (node.vx + ax * effective_dt) * config.damping;
            node.vy = (node.vy + ay * effective_dt) * config.damping;

            // Clamp velocity for orbital stability
            let speed = (node.vx * node.vx + node.vy * node.vy).sqrt();
            if speed > Self::MAX_VELOCITY {
                let factor = Self::MAX_VELOCITY / speed;
                node.vx *= factor;
                node.vy *= factor;
            }

            // Position update
            node.x += node.vx * effective_dt * 15.0;
            node.y += node.vy * effective_dt * 15.0;

            if record_trail {
                node.record_trail();
            }
        }
    }
}
