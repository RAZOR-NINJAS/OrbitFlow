use rand::Rng;

pub struct CosmicDust {
    pub particles: Vec<DustParticle>,
}

pub struct DustParticle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    #[allow(dead_code)]
    pub brightness: f64,
}

impl CosmicDust {
    pub const BOUND_RADIUS: f64 = 100.0;

    pub fn new(count: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(count);

        for _ in 0..count {
            let x = rng.gen_range(-Self::BOUND_RADIUS..Self::BOUND_RADIUS);
            let y = rng.gen_range(-Self::BOUND_RADIUS..Self::BOUND_RADIUS);
            let vx = rng.gen_range(-0.04..0.04);
            let vy = rng.gen_range(-0.04..0.04);
            let brightness = rng.gen_range(0.2..1.0);
            particles.push(DustParticle {
                x,
                y,
                vx,
                vy,
                brightness,
            });
        }

        Self { particles }
    }

    pub fn step(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.x += p.vx * dt * 10.0;
            p.y += p.vy * dt * 10.0;

            if p.x > Self::BOUND_RADIUS {
                p.x = -Self::BOUND_RADIUS;
            } else if p.x < -Self::BOUND_RADIUS {
                p.x = Self::BOUND_RADIUS;
            }

            if p.y > Self::BOUND_RADIUS {
                p.y = -Self::BOUND_RADIUS;
            } else if p.y < -Self::BOUND_RADIUS {
                p.y = Self::BOUND_RADIUS;
            }
        }
    }
}
