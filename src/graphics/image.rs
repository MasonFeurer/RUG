use crate::graphics::Graphics;
use crate::pixel_buf::{PixBufMutView, PixBufView};
use crate::shapes::Rect;
use crate::vectors::Vec2;
use image::GenericImageView;

#[macro_export]
macro_rules! include_image {
    ($path:literal) => {{
        $crate::graphics::image::load_from_memory(include_bytes!($path))
    }};
}
pub fn load_from_memory(bytes: &[u8]) -> Result<Image, image::error::ImageError> {
    image::load_from_memory(bytes).map(|image| {
        let size: Vec2<u32> = image.dimensions().into();
        let bytes = image.to_rgba8().into_raw();
        Image { bytes, size }
    })
}

pub struct Image {
    pub bytes: Vec<u8>,
    pub size: Vec2<u32>,
}
impl Image {
    #[inline(always)]
    pub fn size(&self) -> Vec2<u32> {
        self.size
    }
    #[inline(always)]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    #[inline(always)]
    pub fn mut_bytes(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Constructs a new `Image` with the given size, with all bytes set to 0.
    pub fn empty(size: Vec2<u32>) -> Self {
        Self {
            bytes: vec![0; size.x as usize * size.y as usize * 4],
            size,
        }
    }

    /// Creates a `PixBufMutRef` with a mutable borrow of the buffer for this image.
    pub fn mut_view(&mut self) -> PixBufMutView {
        PixBufMutView {
            size: self.size,
            bytes: &mut self.bytes,
        }
    }
    pub fn view(&self) -> PixBufView {
        PixBufView {
            size: self.size,
            bytes: &self.bytes,
        }
    }

    /// Creates a `Graphics` with a mutable borrow of the buffer for this image.
    /// All drawing functions in `Graphics` will directly effect this image.
    pub fn create_graphics(&mut self) -> Graphics {
        Graphics {
            size: self.size,
            buffer: self.mut_view(),
        }
    }
    /// Creates a `Rect` at the position given, and with the same size as this image.
    pub fn rect_at(&self, pos: Vec2<i32>) -> Rect {
        Rect::new(pos.x, pos.y, self.size.x as i32, self.size.y as i32)
    }
}
