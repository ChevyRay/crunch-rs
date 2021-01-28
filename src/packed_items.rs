use crate::Rect;

/// Collection of items that were successfully packed by `Packer`.
pub struct PackedItems<T>(pub(crate) Vec<(Rect, T)>);

impl<T> IntoIterator for PackedItems<T> {
    type Item = (Rect, T);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a PackedItems<T> {
    type Item = &'a (Rect, T);
    type IntoIter = std::slice::Iter<'a, (Rect, T)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}
