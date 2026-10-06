use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    #[allow(dead_code)]
    pub bg: Color,
    pub star: Color,
    pub planet: Color,
    pub moon: Color,
    pub asteroid: Color,
    pub selection: Color,
    pub spring_relaxed: Color,
    pub spring_tense: Color,
    pub dust: Color,
    pub trail: Color,
    pub border: Color,
    pub border_focus: Color,
    pub text_primary: Color,
    pub text_muted: Color,
    pub accent: Color,
}

impl Theme {
    pub fn cyberpunk() -> Self {
        Self {
            name: "Cyberpunk Neon",
            bg: Color::Rgb(10, 10, 18),
            star: Color::Rgb(250, 204, 21),       // Radiant Gold
            planet: Color::Rgb(56, 189, 248),     // Neon Cyan
            moon: Color::Rgb(192, 132, 252),      // Neon Violet
            asteroid: Color::Rgb(148, 163, 184),  // Slate Dust
            selection: Color::Rgb(52, 211, 153),  // Emerald Reticle
            spring_relaxed: Color::Rgb(96, 165, 250), // Sky Blue
            spring_tense: Color::Rgb(248, 113, 113),  // Crimson Strain
            dust: Color::Rgb(71, 85, 105),        // Faint Stardust
            trail: Color::Rgb(99, 102, 241),      // Indigo Trail
            border: Color::Rgb(51, 65, 85),
            border_focus: Color::Rgb(129, 140, 248),
            text_primary: Color::Rgb(241, 245, 249),
            text_muted: Color::Rgb(148, 163, 184),
            accent: Color::Rgb(244, 114, 182),    // Pink Flare
        }
    }

    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha",
            bg: Color::Rgb(30, 30, 46),
            star: Color::Rgb(249, 226, 175),      // Yellow
            planet: Color::Rgb(137, 180, 250),    // Blue
            moon: Color::Rgb(203, 166, 247),      // Mauve
            asteroid: Color::Rgb(166, 173, 200),  // Subtext
            selection: Color::Rgb(166, 227, 161), // Green
            spring_relaxed: Color::Rgb(116, 199, 236), // Sapphire
            spring_tense: Color::Rgb(243, 139, 168),   // Red
            dust: Color::Rgb(88, 91, 112),        // Surface2
            trail: Color::Rgb(180, 190, 254),     // Lavender
            border: Color::Rgb(69, 71, 90),
            border_focus: Color::Rgb(203, 166, 247),
            text_primary: Color::Rgb(205, 214, 244),
            text_muted: Color::Rgb(166, 173, 200),
            accent: Color::Rgb(245, 194, 231),    // Pink
        }
    }

    pub fn retro_amber() -> Self {
        Self {
            name: "Phosphor Amber",
            bg: Color::Black,
            star: Color::Rgb(255, 191, 0),
            planet: Color::Rgb(255, 165, 0),
            moon: Color::Rgb(255, 215, 0),
            asteroid: Color::Rgb(180, 120, 0),
            selection: Color::Rgb(255, 255, 200),
            spring_relaxed: Color::Rgb(200, 140, 0),
            spring_tense: Color::Rgb(255, 80, 0),
            dust: Color::Rgb(90, 60, 0),
            trail: Color::Rgb(160, 100, 0),
            border: Color::Rgb(120, 80, 0),
            border_focus: Color::Rgb(255, 191, 0),
            text_primary: Color::Rgb(255, 200, 50),
            text_muted: Color::Rgb(160, 120, 20),
            accent: Color::Rgb(255, 220, 100),
        }
    }
}
