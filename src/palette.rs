//! Color palette based on 
//! DawnBringer's 16 Col Palette v1.0
//!
//! https://pixeljoint.com/forum/forum_posts.asp?TID=12795
//!
//! https://lospec.com/palette-list/shmupy-16

use bevy::color::Color;

#[derive(Debug)]
pub struct ColorPair {
    pub dark: Color,
    pub light: Color,
}

impl ColorPair {
    pub const METAL: Self = Self {
        dark: Color::srgb(0.302, 0.29, 0.306),
        light: Color::srgb(0.533, 0.584, 0.624),
    };

    pub const DARK_WOOD: Self = Self {
        dark: Color::srgb(0.251, 0.149, 0.204),
        light: Color::srgb(0.490, 0.310, 0.212),
    };
}

