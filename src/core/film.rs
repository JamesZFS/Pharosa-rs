use super::*;
use crate::macros::*;
use std::fmt::{Debug, Formatter};

#[derive(Clone)]
pub struct Film {
    width: u32,
    height: u32,
    /// row major
    data: Vec<Spectrum>,
}

impl Film {
    pub fn new(width: u32, height: u32) -> Self { Self::new_with_color(width, height, Spectrum::black()) }
    pub fn new_with_color(width: u32, height: u32, color: Spectrum) -> Self {
        Self {
            width,
            height,
            data: vec![color; (width * height) as usize],
        }
    }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn size(&self) -> usize { self.data.len() }
    pub fn at(&self, x: u32, y: u32) -> &Spectrum {
        unsafe { self.data.get_unchecked(self.index(x, y)) } // safety is guaranteed inside self.index
    }
    pub fn at_mut(&mut self, x: u32, y: u32) -> &mut Spectrum {
        let index = self.index(x, y);
        unsafe { self.data.get_unchecked_mut(index) }
    }
    pub unsafe fn at_unchecked(&self, x: u32, y: u32) -> &Spectrum {
        self.data.get_unchecked(self.index_unchecked(x, y))
    }
    pub unsafe fn at_unchecked_mut(&mut self, x: u32, y: u32) -> &mut Spectrum {
        let index = self.index_unchecked(x, y);
        self.data.get_unchecked_mut(index)
    }

    #[inline]
    fn index(&self, x: u32, y: u32) -> usize {
        assert_le!(x, self.width, "Film index out of bounds!");
        assert_le!(y, self.height, "Film index out of bounds!");
        self.index_unchecked(x, y)
    }
    #[inline]
    fn index_unchecked(&self, x: u32, y: u32) -> usize {
        (x + y * self.width) as usize
    }

    pub fn to_raw(&self) -> &Vec<Spectrum> { &self.data }
}

impl Debug for Film {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Film")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}
