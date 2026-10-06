use crate::model::{NodeType, Universe};
use std::fs::File;
use std::io::{Read, Write};

pub struct StorageManager;

impl StorageManager {
    pub const DEFAULT_SAVE_PATH: &'static str = "orbitflow.json";
    pub const DEFAULT_EXPORT_PATH: &'static str = "orbitflow.md";

    pub fn save(universe: &Universe, path: &str) -> Result<(), String> {
        let json_str = serde_json::to_string_pretty(universe).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(json_str.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn load(path: &str) -> Result<Universe, String> {
        let mut file = File::open(path).map_err(|e| e.to_string())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
        let universe: Universe = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
        Ok(universe)
    }

    pub fn export_markdown(universe: &Universe, path: &str) -> Result<(), String> {
        let mut md = String::new();
        md.push_str("# 🪐 OrbitFlow Galaxy Mindmap\n\n");
        md.push_str(&format!(
            "> Exported with {} celestial thoughts and {} gravitational links.\n\n",
            universe.nodes.len(),
            universe.connections.len()
        ));

        // Group stars
        let stars: Vec<_> = universe.nodes.iter().filter(|n| n.node_type == NodeType::Star).collect();
        for star in &stars {
            md.push_str(&format!("## ★ {}\n\n", star.title));
            if !star.tags.is_empty() {
                md.push_str(&format!("*Tags: `{}`*\n\n", star.tags.join("`, `")));
            }
            if !star.notes.trim().is_empty() {
                md.push_str(&format!("{}\n\n", star.notes));
            }

            // Find connected planets
            let connected_ids: Vec<usize> = universe
                .connections
                .iter()
                .filter_map(|c| {
                    if c.from_id == star.id {
                        Some(c.to_id)
                    } else if c.to_id == star.id {
                        Some(c.from_id)
                    } else {
                        None
                    }
                })
                .collect();

            let planets: Vec<_> = universe
                .nodes
                .iter()
                .filter(|n| n.node_type == NodeType::Planet && connected_ids.contains(&n.id))
                .collect();

            for planet in &planets {
                md.push_str(&format!("### ● {}\n\n", planet.title));
                if !planet.tags.is_empty() {
                    md.push_str(&format!("*Tags: `{}`*\n\n", planet.tags.join("`, `")));
                }
                if !planet.notes.trim().is_empty() {
                    md.push_str(&format!("{}\n\n", planet.notes));
                }

                // Moons orbiting this planet
                let planet_links: Vec<usize> = universe
                    .connections
                    .iter()
                    .filter_map(|c| {
                        if c.from_id == planet.id {
                            Some(c.to_id)
                        } else if c.to_id == planet.id {
                            Some(c.from_id)
                        } else {
                            None
                        }
                    })
                    .collect();

                let moons: Vec<_> = universe
                    .nodes
                    .iter()
                    .filter(|n| n.node_type == NodeType::Moon && planet_links.contains(&n.id))
                    .collect();

                for moon in &moons {
                    md.push_str(&format!("- [ ] **◦ {}**", moon.title));
                    if !moon.notes.trim().is_empty() {
                        md.push_str(&format!(": {}", moon.notes.lines().next().unwrap_or("")));
                    }
                    md.push('\n');
                }
                md.push('\n');
            }
        }

        // Unlinked or remaining nodes
        let remaining: Vec<_> = universe
            .nodes
            .iter()
            .filter(|n| n.node_type == NodeType::Asteroid)
            .collect();

        if !remaining.is_empty() {
            md.push_str("## ☄ Stray Thoughts & Asteroids\n\n");
            for ast in remaining {
                md.push_str(&format!("- **· {}**\n", ast.title));
                if !ast.notes.trim().is_empty() {
                    md.push_str(&format!("  {}\n", ast.notes));
                }
            }
        }

        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(md.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}
