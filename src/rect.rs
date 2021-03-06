/// A simple rectangle structure used for packing.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    /// Create a new `Rect`.
    #[inline]
    pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    /// Create a new `Rect` with the size `w` x `h`.
    ///
    /// This is the same as calling `Rect::new(0, 0, w, h)`.
    #[inline]
    pub fn of_size(w: usize, h: usize) -> Self {
        Self::new(0, 0, w, h)
    }

    /// The area of the rectangle.
    #[inline]
    pub fn area(&self) -> usize {
        self.w * self.h
    }

    /// Returns true if `other` is fully contained inside `self`.
    #[inline]
    pub fn contains(&self, other: &Rect) -> bool {
        other.x >= self.x
            && other.y >= self.y
            && other.right() <= self.right()
            && other.bottom() <= self.bottom()
    }

    /// Returns true if `other` overlaps `self`.
    #[inline]
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.x < other.right()
            && self.y < other.bottom()
            && self.right() > other.x
            && self.bottom() > other.y
    }

    /// The rectangle's top-left coordinates.
    #[inline]
    pub fn top_left(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    /// The right edge of the rectangle.
    #[inline]
    pub fn right(&self) -> usize {
        self.x + self.w
    }

    /// The bottom edge of the rectangle.
    #[inline]
    pub fn bottom(&self) -> usize {
        self.y + self.h
    }

    #[inline]
    pub(crate) fn split(&self, rect: &Rect) -> [Option<Self>; 4] {
        let (self_r, self_b) = (self.right(), self.bottom());
        let (rect_r, rect_b) = (rect.right(), rect.bottom());
        [
            match rect.x > self.x {
                true => Some(Self::new(self.x, self.y, rect.x - self.x, self.h)),
                false => None,
            },
            match rect_r < self_r {
                true => Some(Self::new(rect_r, self.y, self_r - rect_r, self.h)),
                false => None,
            },
            match rect.y > self.y {
                true => Some(Self::new(self.x, self.y, self.w, rect.y - self.y)),
                false => None,
            },
            match rect_b < self_b {
                true => Some(Self::new(self.x, rect_b, self.w, self_b - rect_b)),
                false => None,
            },
        ]
    }
}
