#[macro_export]
macro_rules! color {
    ($r:expr,$g:expr,$b:expr,$a:expr) => {
        Color::pack_rgba($r, $g, $b, $a)
    };
    ($r:expr,$g:expr,$b:expr) => {
        Color::pack_rgba($r, $g, $b, 255)
    };
    ($shade:expr,$a:expr) => {
        Color::pack_rgba($shade, $shade, $shade, $a)
    };
    ($shade:expr) => {
        Color::pack_rgba($shade, $shade, $shade, 255)
    };
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Color(pub u32);
impl Color {
    pub const WHITE: Self = Self::pack_const(255, 255, 255, 255);
    pub const BLACK: Self = Self::pack_const(0, 0, 0, 255);

    pub const RED: Self = Self::pack_const(255, 0, 0, 255);
    pub const GREEN: Self = Self::pack_const(0, 255, 0, 255);
    pub const BLUE: Self = Self::pack_const(0, 0, 255, 255);

    pub const YELLOW: Self = Self::pack_const(255, 255, 0, 255);
    pub const CYAN: Self = Self::pack_const(0, 255, 255, 255);
    pub const MAGENTA: Self = Self::pack_const(255, 0, 255, 255);

    pub const fn pack_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::pack([r, g, b, a])
    }
    pub const fn pack_const(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::pack([a, b, g, r])
        // a g b r
        // a b g r
    }
    pub const fn pack(vals: [u8; 4]) -> Self {
        let [x, y, z, w] = vals;
        Self((x as u32) << 24 | (y as u32) << 16 | (z as u32) << 8 | (w as u32))
    }
    pub const fn unpack(self) -> [u8; 4] {
        [
            ((self.0 >> 0) & 0xFF) as u8,
            ((self.0 >> 8) & 0xFF) as u8,
            ((self.0 >> 16) & 0xFF) as u8,
            ((self.0 >> 24) & 0xFF) as u8,
        ]
    }
}
