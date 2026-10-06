<div align="center">

```
   ____       __    _ __  ______  __               
  / __ \_____/ /_  (_) /_/ ____/ / /___ _      __ 
 / / / / ___/ __ \/ / __/ /_    / / __ \ | /| / / 
/ /_/ / /  / /_/ / / /_/ __/   / / /_/ / |/ |/ /  
\____/_/  /_.___/_/\__/_/     /_/\____/|__/|__/   
```

### 🪐 Zero-Gravity Spatial Mindmap & Scratchpad in Your Terminal
*Turn your ideas into celestial bodies governed by real-time orbital physics.*

[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Ratatui](https://img.shields.io/badge/ratatui-0.29-green.svg?style=flat-square)](https://ratatui.rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)

</div>

---

## ✨ What is OrbitFlow?

**OrbitFlow** is a hacker's spatial note-taking and brainstorming playground that runs entirely inside your terminal.

Instead of rigid bullet lists or static tree diagrams, thoughts in OrbitFlow are **celestial bodies**:
- 🌟 **Stars (Hubs):** Massive anchor thoughts that exert central gravitational attraction.
- ● **Planets (Concepts):** Medium-mass thoughts that orbit stars or drift freely.
- ◦ **Moons (Tasks/Notes):** Sub-tasks bound by elastic gravitational springs to their parent planet.
- · **Asteroids (Stray Thoughts):** Light, unanchored sparks of inspiration.

All bodies interact via **N-Body Gravitational Physics**, **Coulomb Electrostatic Repulsion** (preventing overlapping clutter), and **Hooke's Elastic Springs**—rendered at a silky 60 FPS using Ratatui's high-resolution Braille canvas with stardust motion trails!

---

## 🚀 Key Features

* **Sub-Pixel Braille Canvas:** 2×4 sub-pixel terminal graphics with stardust motion trails, glowing orbital paths, and tension-colored spring lines.
* **Tactile Physics & Mouse Fling:** Click and drag any planet with your mouse to fling it across space with realistic orbital momentum.
* **Side-Panel Markdown Scratchpad:** Inspect thoughts, write multiline markdown notes, tag ideas, and track real-time physical properties (mass, speed, coordinates).
* **Vim-First Navigation:** Pan with `h/j/k/l`, zoom with `+`/`-`, cycle with `Tab`, and snap-focus camera with `f`.
* **Live Physics Sandbox HUD:** Adjust gravitational constant $G$, cosmic drag/damping, and simulation speed on the fly with single keystrokes.
* **Galaxy Presets:**
  * `[1]` **Solar System Brainstorm:** Core initiative with orbiting subsystems and task moons.
  * `[2]` **Three-Body Problem:** Chaotic gravitational sandbox with binary stars and slingshot asteroids.
* **TrueColor Themes:** Switch between **Cyberpunk Neon**, **Catppuccin Mocha**, and **Phosphor Amber** with `t`.
* **Persistence & Export:** Save/load state via JSON (`s`) or export your entire galaxy into a structured Markdown document (`m`).

---

## 🎮 Quickstart

### Prerequisites
Make sure you have [Rust & Cargo](https://rustup.rs/) installed.

```bash
git clone https://github.com/vaibhav/orbitflow.git
cd orbitflow
cargo run --release
```

---

## 🕹️ Flight Manual & Keybindings

### 🌌 Navigation & Camera
| Key | Action |
| --- | --- |
| `h` / `j` / `k` / `l` or `Arrows` | Pan camera across space |
| `+` / `-` or `Mouse Scroll` | Zoom in / Zoom out |
| `f` | Snap & focus camera on selected planet |
| `0` | Reset camera to cosmic origin `(0, 0)` |

### 💡 Mindmap & Thoughts
| Key | Action |
| --- | --- |
| `Tab` | Cycle selected celestial thought |
| `n` | Birth new celestial thought (*Star, Planet, Moon, Asteroid*) |
| `e` | Open scratchpad note editor for selected thought |
| `c` | Link two thoughts with a gravitational spring |
| `p` | Pin / Unpin thought as an immovable spatial anchor |
| `d` / `Delete` | Erase selected thought from the cosmos |
| `Left Click & Drag` | Grab and fling any planet with orbital velocity |

### ⚛️ Physics Sandbox
| Key | Action |
| --- | --- |
| `Space` | Pause / Resume cosmic simulation |
| `[` / `]` | Decrease / Increase Gravity $G$ |
| `{` / `}` | Decrease / Increase Cosmic Drag (damping) |
| `<` / `>` | Slow down / Speed up simulation time scale |

### 🛠️ Presets & System
| Key | Action |
| --- | --- |
| `1` | Load **Solar System Brainstorm** preset |
| `2` | Load **Three-Body Problem** preset |
| `t` | Cycle TrueColor Theme (*Cyberpunk / Catppuccin / Amber*) |
| `s` | Save galaxy state to `orbitflow.json` |
| `m` | Export galaxy to `orbitflow.md` |
| `?` | Open interactive Help & Flight Manual overlay |
| `q` / `Esc` | Quit OrbitFlow |

---

## 🏗️ Architecture

```
src/
├── main.rs              # 60 FPS event loop, raw terminal lifecycle & input router
├── app.rs               # State machine, screen-to-world camera projection
├── model/
│   ├── node.rs          # Node, NodeType (Star, Planet, Moon), stardust trail queue
│   ├── connection.rs    # Spring links and tension calculations
│   └── universe.rs      # Galaxy container, spatial queries, and presets
├── physics/
│   ├── engine.rs        # Symplectic Verlet integration, N-body gravity, Coulomb repulsion
│   └── particle.rs      # Ambient cosmic dust field
├── ui/
│   ├── canvas.rs        # Ratatui Braille sub-pixel canvas renderer
│   ├── inspector.rs     # Markdown scratchpad & node property sidebar
│   ├── hud.rs           # Real-time status bar & physics controls
│   ├── dialogs.rs       # Modal popups (Help manual, New Node, Note editor)
│   └── theme.rs         # 24-bit TrueColor themes
└── storage/
    └── io.rs            # JSON serialization and hierarchical Markdown exporter
```

---

## 📜 License

Licensed under the [MIT License](LICENSE).
