/// Rotation setting for packing rectangles.
#[derive(Copy, Clone, PartialEq)]
pub enum Rotation {
    None,
    Allowed,
}

/// An item to be packed by `Packer`.
#[derive(Clone)]
pub struct Item<T> {
    pub data: T,
    pub w: usize,
    pub h: usize,
    pub rot: Rotation,
}

impl<T> Item<T> {
    /// Creates a new packing item.
    ///
    /// `w` `h`: the size of the item.
    ///
    /// `rot`: controls whether the packer is allowed to rotate the item.
    ///
    /// `data`: custom user-data you can associate with the item
    /// (for example, in an image packer, this might be a reference to the image)
    #[inline]
    pub fn new(data: T, w: usize, h: usize, rot: Rotation) -> Self {
        Self { data, w, h, rot }
    }

    #[inline]
    pub(crate) fn sort_priority(&self) -> usize {
        let area = self.w * self.h;
        let longest_side = self.w.max(self.h);
        area + longest_side
    }
}
