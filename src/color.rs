use bevy::prelude::{Color, Deref, DerefMut};

#[derive(Deref, DerefMut, Default)]
pub struct HexColor(pub u32);

impl Into<Color> for HexColor {
    fn into(self) -> Color {
        let r = ((self.0 >> 16) & 0xFF) as f32 / 255.0;
        let g = ((self.0 >> 8) & 0xFF) as f32 / 255.0;
        let b = (self.0 & 0xFF) as f32 / 255.0;
        Color::srgb(r, g, b)
    }
}
