use crate::vec::Vector3;

pub type Color = Vector3;

impl Color {
    pub fn white() -> Color {
        Color::new(1, 1, 1)
    }
}
