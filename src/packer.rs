use crate::{Item, PackedItems, Rect, Rotation};
use std::iter::*;

/// Attempts to tightly pack the supplied `items` into `into_rect`.
///
/// Returns a collection of `PackedItems` on success, or all items
/// that were packed before failure.
///
/// Shorthand for:
/// ```
/// let mut packer = Packer::with_items(items);
/// packer.pack(into_rect)
/// ```
///
/// Example usage:
/// ```
/// let rect = Rect::of_size(15, 15);
/// let items = vec![
///     Item::new('A', 2, 9, Rotation::Allowed),
///     Item::new('B', 3, 8, Rotation::Allowed),
///     Item::new('C', 4, 7, Rotation::Allowed),
///     Item::new('D', 5, 6, Rotation::Allowed),
///     Item::new('E', 6, 5, Rotation::Allowed),
///     Item::new('F', 7, 4, Rotation::Allowed),
///     Item::new('G', 8, 3, Rotation::Allowed),
///     Item::new('H', 9, 2, Rotation::Allowed),
/// ];
///
/// let packed = match pack(rect, items) {
///     Ok(all_packed) => all_packed,
///     Err(some_packed) => some_packed,
/// };
///
/// // Every item fits inside rect without overlapping any others.
/// for (r, chr) in &packed {
///     assert_eq!(rect.contains(&r), true);
///     for (r2, chr2) in &packed {
///         assert_eq!(chr != chr2 && r.overlaps(r2), false);
///     }
/// }
/// ```
pub fn pack<T, I>(into_rect: Rect, items: I) -> Result<PackedItems<T>, PackedItems<T>>
where
    T: Clone,
    I: IntoIterator<Item = Item<T>>,
{
    let mut packer = Packer::with_items(items);
    packer.pack(into_rect)
}

/// Attempts to pack the supplied items into the smallest power of 2 container
/// it possibly can, while not exceeding the provided `max_size`.
///
/// On success, returns the size of the container (a power of 2) and the packed items.
pub fn pack_into_po2<T, I>(max_size: usize, items: I) -> Result<(usize, usize, PackedItems<T>), ()>
where
    T: Clone,
    I: IntoIterator<Item = Item<T>>,
{
    let mut packer = Packer::with_items(items);
    packer.pack_into_po2(max_size)
}

/// A packer for items of type `Item<T>`.
pub struct Packer<T> {
    items_to_pack: Vec<Item<T>>,
    nodes: Vec<Node>,
    indices: Vec<usize>,
}

impl<T> Packer<T> {
    /// Create a new, empty packer.
    pub fn new() -> Self {
        Self {
            items_to_pack: Vec::new(),
            nodes: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Create a packer with an initial `capacity` to prevent collection resizing.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items_to_pack: Vec::with_capacity(capacity),
            nodes: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Create a packer initialized with the collection of `items`.
    pub fn with_items<I: IntoIterator<Item = Item<T>>>(items: I) -> Self {
        Self {
            items_to_pack: items.into_iter().collect(),
            nodes: Vec::new(),
            indices: Vec::new(),
        }
    }
}

impl<T> Default for Packer<T> {
    /// Default packer, equivalent to `Packer::new()`.
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Packer<T> {
    pub fn clear(&mut self) -> &mut Self {
        self.items_to_pack.clear();
        self
    }

    #[inline]
    pub fn push(&mut self, item: Item<T>) -> &mut Self {
        self.items_to_pack.push(item);
        self
    }

    #[inline]
    pub fn extend<I: IntoIterator<Item = Item<T>>>(&mut self, items: I) -> &mut Self {
        self.items_to_pack.extend(items);
        self
    }

    //find the node that best fits a new rectangle of size (w, h)
    #[inline]
    fn find_best_node(&self, w: usize, h: usize, node_index: usize) -> (usize, Score) {
        let node = &self.nodes[node_index];
        //check if this node's branch could potentially hold the new rect
        match w <= node.rect.w && h <= node.rect.h {
            false => (usize::MAX, Score::worst()),
            true => {
                //check if the node is a branch or a leaf node
                match node.is_split {
                    //the node hasn't been split, so it is a packing candidate
                    false => (node_index, Score::new(&node.rect, w, h)),

                    //the node has been split, so check its children for packing candidates
                    true => node.split.iter().filter(|&&i| i > 0).fold(
                        (usize::MAX, Score::worst()),
                        |(best_i, best_s), &child| {
                            let (i, s) = self.find_best_node(w, h, child);
                            match s.better_than(&best_s) {
                                true => (i, s),
                                false => (best_i, best_s),
                            }
                        },
                    ),
                }
            }
        }
    }

    //returns true if any leaf node contains the supplied rect
    #[inline]
    fn leaf_contains_rect(&self, rect: &Rect, node_index: usize) -> bool {
        let node = &self.nodes[node_index];
        match node.rect.contains(rect) {
            false => false,
            true => {
                !node.is_split
                    || node
                        .split
                        .iter()
                        .any(|&i| i > 0 && self.leaf_contains_rect(rect, i))
            }
        }
    }

    //split all nodes that overlap with this rectangle
    #[inline]
    fn split_tree(&mut self, rect: &Rect, node_index: usize) {
        //if the rectangle overlaps with this branch of the tree
        if self.nodes[node_index].rect.overlaps(rect) {
            //if the node is already split, recursively split into its child nodes
            if self.nodes[node_index].is_split {
                let split = self.nodes[node_index].split;
                for i in split.iter().cloned().filter(|&i| i > 0) {
                    self.split_tree(rect, i);
                }
            } else {
                //split the rect into 0-4 sub-rects and make a new node out of each
                self.nodes[node_index].is_split = true;
                let rects = self.nodes[node_index].rect.split(rect);
                for i in 0..rects.len() {
                    if let Some(r) = &rects[i] {
                        //only add the child rect if no other leaf node contains it
                        if !self.leaf_contains_rect(r, 0) {
                            self.nodes[node_index].split[i] = self.nodes.len();
                            self.nodes.push(Node {
                                rect: *r,
                                is_split: false,
                                split: [0; 4],
                            });
                        }
                    }
                }
            }
        }
    }

    /// Attempt to pack all the items into `into_rect`. The returned `PackedItems`
    /// will contain positions for all packed items on success, or just the items
    /// the packer was able to successfull pack before failing.
    ///
    /// This function uses some internal intermediary collections, which is why
    /// it is mutable, so it cannot be called but it is valid to call it multiple times with different
    /// `into_rect` values.
    ///
    /// If you want to attempt to pack the same item list into several different
    /// `into_rect`, it is valid to call this function multiple times on the same
    /// `Packer`, and it will re-use its intermediary data structures.
    pub fn pack(&mut self, into_rect: Rect) -> Result<PackedItems<T>, PackedItems<T>> {
        println!("packing: {}, {}", into_rect.w, into_rect.h);

        //start with one node that is the full size of the rect
        //reserve a deccent amount of room in the initial nodes vec
        self.nodes.clear();
        self.nodes.reserve(self.items_to_pack.len() * 2);
        self.nodes.push(Node {
            rect: into_rect,
            is_split: false,
            split: [0; 4],
        });

        //indices of items we need to pack, sorted by their area
        //the largest items should be packed first for best fits
        self.indices.clear();
        self.indices.extend(0..self.items_to_pack.len());
        {
            let items = &self.items_to_pack;
            self.indices.sort_by(|&a, &b| {
                let sort_a = items[a].sort_priority();
                let sort_b = items[b].sort_priority();
                sort_a.cmp(&sort_b)
            });
        }
        self.indices.reverse();

        //list of packed items we'll return (whether we succeed or fail)
        let mut packed = Vec::with_capacity(self.items_to_pack.len());

        //pack all items, longest sides -> shorted sides
        //for &item_index in (&self.indices).into_iter().rev() {
        for ind in 0..self.indices.len() {
            let item = self.items_to_pack[self.indices[ind]].clone();

            //find the best position to pack the item
            //if the item is rotated 90º, pack_w and pack_h will be swapped
            let mut pack_w = item.w;
            let mut pack_h = item.h;
            let (mut node_i, score) = self.find_best_node(item.w, item.h, 0);
            if item.rot == Rotation::Allowed && item.w != item.h {
                let (i, s) = self.find_best_node(item.h, item.w, 0);
                if s.better_than(&score) {
                    node_i = i;
                    pack_w = item.h;
                    pack_h = item.w;
                }
            }

            //if we failed to pack the item, return failure
            //and everything we did manage to pack
            if node_i == usize::MAX {
                return Err(PackedItems(packed));
            }

            //get the final rectangle where the item will be packed
            let (node_x, node_y) = self.nodes[node_i].rect.top_left();
            let rect = Rect::new(node_x, node_y, pack_w, pack_h);

            //split the tree on the new item's rect to create new packing branches
            self.split_tree(&rect, 0);

            //add the item to the successfully packed list
            packed.push((rect, item.data));
        }

        Ok(PackedItems(packed))
    }

    /// Attempts to pack the supplied items into the smallest power of 2 container
    /// it possibly can while not exceeding the provided `max_size`.
    ///
    /// On success, returns the size of the container (a power of 2) and the packed items.
    pub fn pack_into_po2(&mut self, max_size: usize) -> Result<(usize, usize, PackedItems<T>), ()> {
        let min_area = self.items_to_pack.iter().map(|i| i.w * i.h).sum();

        let mut size = 2;
        while size * size * 2 < min_area {
            size *= 2;
        }

        while size <= max_size {
            let result = if size * size >= min_area {
                self.pack(Rect::of_size(size, size))
            } else {
                Err(PackedItems(Vec::new()))
            };

            let (w, h, result) = match result {
                Ok(packed) => (size, size, Ok(packed)),
                Err(failed) => match size * 2 <= max_size {
                    true => match self.pack(Rect::of_size(size * 2, size)) {
                        Ok(packed) => (size * 2, size, Ok(packed)),
                        Err(_) => match self.pack(Rect::of_size(size, size * 2)) {
                            Ok(packed) => (size, size * 2, Ok(packed)),
                            Err(failed) => (0, 0, Err(failed)),
                        },
                    },
                    false => (0, 0, Err(failed)),
                },
            };

            if let Ok(packed) = result {
                return Ok((w, h, packed));
            }
            size *= 2;
        }

        Err(())
    }
}

/// A branch of the packing tree, `split` are indices that point to other nodes.
struct Node {
    rect: Rect,
    is_split: bool,
    split: [usize; 4],
}

/// The packer's way of scoring how well a rect fits into another rect.
#[derive(Copy, Clone)]
struct Score {
    area_fit: usize,
    short_fit: usize,
}

impl Score {
    /// Score how well `rect` fits into a rect of size `w` x `h`.
    #[inline]
    fn new(rect: &Rect, w: usize, h: usize) -> Self {
        let extra_x = rect.w - w;
        let extra_y = rect.h - h;
        Self {
            area_fit: rect.area() - w * h,
            short_fit: extra_x.min(extra_y),
        }
    }

    /// The worst possible packing score.
    #[inline]
    fn worst() -> Self {
        Self {
            area_fit: usize::MAX,
            short_fit: usize::MAX,
        }
    }

    /// Returns `true` if this score is better than `other`.
    #[inline]
    fn better_than(&self, other: &Score) -> bool {
        self.area_fit < other.area_fit
            || (self.area_fit == other.area_fit && self.short_fit < other.short_fit)
    }
}
