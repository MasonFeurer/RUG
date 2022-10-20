use crate::graphics::colors::Color;
use crate::graphics::Graphics;
use crate::shapes::Rect;
use crate::vectors::Vec2;

#[macro_export]
macro_rules! calc_index {
    ($pos:expr,$w:expr) => {
        ($pos.x + $pos.y * $w as i32) as usize * 4
    };
}

pub struct PixBufMutView<'a> {
    pub bytes: &'a mut [u8],
    pub size: Vec2<u32>,
}
impl<'a> PixBufMutView<'a> {
    #[inline(always)]
    pub fn rect_at(&self, pos: Vec2<i32>) -> Rect {
        Rect::from_pos_size(pos, self.size.map(|e| e as i32))
    }

    /// Sets a pixel at the given position to the color given.
    ///
    /// ## Returns:
    /// - `Err` if the position is not in bounds.
    /// - `Ok` otherwise.
    pub fn set_pixel(&mut self, pos: Vec2<i32>, color: Color) -> Result<(), ()> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x as i32 || pos.y >= self.size.y as i32 {
            return Err(());
        }
        // SAFETY: TODO
        unsafe {
            self.set_pixel_unchecked(pos, color);
        }
        Ok(())
    }

    /// Sets a pixel at the given position to the color given.
    ///
    /// **UNSAFE** - This function is a single call to `set_pixel_by_index`,
    /// so given an invalid position, this function will cause undefined behavior
    #[inline(always)]
    pub unsafe fn set_pixel_unchecked(&mut self, pos: Vec2<i32>, color: Color) {
        self.set_pixel_by_index(calc_index!(pos, self.size.x), color);
    }

    /// Sets a pixel in the buffer, specified by an index, to an RGBA value represented by `int`.
    ///
    /// **UNSAFE** - Given an invalid index, this function will cause undefined behavior.
    #[inline(always)]
    pub unsafe fn set_pixel_by_index(&mut self, index: usize, color: Color) {
        let ptr_4_bytes = &mut self.bytes[index] as *mut u8 as *mut u32;
        *ptr_4_bytes = color.0;
    }

    /// Returns the color of the pixel at the given position.
    /// If the position is not in bounds, returns `None`.
    pub fn get_pixel(&self, pos: Vec2<i32>) -> Option<Color> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x as i32 || pos.y >= self.size.y as i32 {
            return None;
        }
        // SAFETY: TODO
        unsafe { Some(self.get_pixel_unchecked(pos)) }
    }

    /// Returns the color of the pixel at the given position.
    ///
    /// **UNSAFE** - This function is a single call to `get_pixel_by_index`,
    /// so given an invalid position, this function will cause undefined behavior
    #[inline(always)]
    pub unsafe fn get_pixel_unchecked(&self, pos: Vec2<i32>) -> Color {
        self.get_pixel_by_index(calc_index!(pos, self.size.x))
    }

    /// Returns the color of the pixel in the buffer at the given index.
    ///
    /// **UNSAFE** - Given an invalid index, this function will cause undefined behavior.
    #[inline(always)]
    pub unsafe fn get_pixel_by_index(&self, index: usize) -> Color {
        let ptr = &self.bytes[index] as *const u8 as *const u32;
        Color(*ptr)
    }

    /// Creates a `Graphics` with this buffer.
    ///
    /// All drawing functions in `Graphics` will write to the underlying bytes for this buffer.
    pub fn create_graphics(self) -> Graphics<'a> {
        let size = self.size;
        Graphics::new(self, size)
    }

    pub fn non_mut(self) -> PixBufView<'a> {
        PixBufView {
            bytes: self.bytes,
            size: self.size,
        }
    }
}

pub struct PixBufView<'a> {
    pub bytes: &'a [u8],
    pub size: Vec2<u32>,
}
impl<'a> PixBufView<'a> {
    #[inline(always)]
    pub fn rect_at(&self, pos: Vec2<i32>) -> Rect {
        Rect::from_pos_size(pos, self.size.map(|e| e as i32))
    }

    /// Returns the color of the pixel at the given position.
    /// If the position is not in bounds, returns `None`.
    pub fn get_pixel(&self, pos: Vec2<i32>) -> Option<Color> {
        if pos.x < 0 || pos.y < 0 || pos.x >= self.size.x as i32 || pos.y >= self.size.y as i32 {
            return None;
        }
        // SAFETY: TODO
        unsafe { Some(self.get_pixel_unchecked(pos)) }
    }

    /// Returns the color of the pixel at the given position.
    ///
    /// **UNSAFE** - This function is a single call to `get_pixel_by_index`,
    /// so given an invalid position, this function will cause undefined behavior
    #[inline(always)]
    pub unsafe fn get_pixel_unchecked(&self, pos: Vec2<i32>) -> Color {
        self.get_pixel_by_index(calc_index!(pos, self.size.x))
    }

    /// Returns the color of the pixel in the buffer at the given index.
    ///
    /// **UNSAFE** - Given an invalid index, this function will cause undefined behavior.
    #[inline(always)]
    pub unsafe fn get_pixel_by_index(&self, index: usize) -> Color {
        let ptr = &self.bytes[index] as *const u8 as *const u32;
        Color(*ptr)
    }
}
