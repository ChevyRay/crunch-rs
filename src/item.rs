use crate::Rect;

/// Rotation setting for packing rectangles.
#[derive(Copy, Clone, PartialEq)]
pub enum Rotation {
    /// The item may not be rotated.
    None,

    /// The item may be rotated 90° to fit better.
    Allowed,
}

/// An item to be packed by `Packer`.
#[derive(Clone)]
pub struct Item<T> {
    /// Data associated with the item (for example, an ID or a
    /// reference to an image).
    pub data: T,

    /// The item's width.
    pub w: usize,

    /// The item's height.
    pub h: usize,

    /// The item may be rotated 90° to fit better.
    ///
    /// Allowing rotation slows down the algorithm, because the item
    /// will try to pack using both `(w, h)` and `(h, w)` as its size
    /// But this can lead to more efficiently packed results.
    ///
    /// If an item is square, it will never be rotated.
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

/// A container of packed items.
pub struct PackedItems<T> {
    /// The width of the container.
    pub w: usize,

    /// The height of the container.
    pub h: usize,

    /// The items packed into the container.
    pub items: Vec<PackedItem<T>>,
}

/// An item that has been packed into a container.
pub struct PackedItem<T> {
    /// The data associated with the item.
    pub data: T,

    /// The position where the item was packed.
    ///
    /// If the item was packed with [`Rotation::Allowed`], you can compare
    /// the rectangle's width with the input width you provided. If they
    /// differ, it means the item was rotated to fit better.
    pub rect: Rect,
}
