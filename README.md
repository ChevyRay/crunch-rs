# Crunch
A rectangle packer, written in Rust, for cramming lots of rectangles into a larger one. It is designed
primarily with sprite-packing in mind (eg. to create sprite-atlases or CSS image sheets).

It is very fast, and very single-minded. It also seems to be able to pack *very* densely,
often surprising me at how much % of space it can fill.

Item rotation is supported, and can be enabled on a per-item basis.

## Example
```rust
use crunch::{pack, Rect, Item, Rotation};
use std::iter::*;

fn main() {

    // The 15x15 container we'll be packing our items into
    let container = Rect::of_size(15, 15);

    // Our items to pack. The user-data here are chars, but could be any copyable type
    let items = vec![
        Item::new('A', 2, 9, Rotation::Allowed),
        Item::new('B', 3, 8, Rotation::Allowed),
        Item::new('C', 4, 7, Rotation::Allowed),
        Item::new('D', 5, 6, Rotation::Allowed),
        Item::new('E', 6, 5, Rotation::Allowed),
        Item::new('F', 7, 4, Rotation::Allowed),
        Item::new('G', 8, 3, Rotation::Allowed),
        Item::new('H', 9, 2, Rotation::Allowed),
    ];

    // Now we can try to pack all the items into this container
    let result = pack(container, items);
    let packed = match result {
        Ok(all_packed) => all_packed,
        Err(some_packed) => some_packed,
    };

    // To display the results, let's create a 15x15 grid of '.' characters
    let mut data : Vec<char> =
        repeat(repeat('.').take(container.w).chain(once('\n')))
        .take(container.h)
        .flatten()
        .collect();
    
    // We can iterate through each (rect, data) pair that was packed
    for (r, chr) in &packed {
        for x in r.x..r.right() {
            for y in r.y..r.bottom() {
                data[y * (container.w + 1) + x] = *chr;
            }
        }
    }

    // The items packed very efficiently, using 90% of the 15x15 container's space.
    // You'll notice that some ('H', for example) were able to rotate to fit.
    let text: String = data.iter().collect();
    println!("{}", text);

    //EEEEEEDDDDDDBBB
    //EEEEEEDDDDDDBBB
    //EEEEEEDDDDDDBBB
    //EEEEEEDDDDDDBBB
    //EEEEEEDDDDDDBBB
    //FFFFCCCCHHAABBB
    //FFFFCCCCHHAABBB
    //FFFFCCCCHHAABBB
    //FFFFCCCCHHAA...
    //FFFFCCCCHHAA...
    //FFFFCCCCHHAA...
    //FFFFCCCCHHAA...
    //GGGGGGGGHHAA...
    //GGGGGGGGHHAA...
    //GGGGGGGG.......
```

## How does it work?
It's a bit tricky to explain, but the algorithm works with a *tree* of nodes.
At the root is the initial container rectangle. When the first item is packed,
it *splits* the container into 0-4 potential sub-containers, or *leaf nodes*.
Every time we insert a new item, every leaf node is given a *score* for how
efficiently (space used) it could pack the item, and then we then insert the
item into the leaf that had the best score. That leaf is then split.

The trick to the algorithm is in how the node tree splits itself. Basically, the
item is packed into the top-left corner of the chosen leaf node it was assigned
to be packed in. Then, we *collide* that rectangle down the tree, starting with
the root. Every leaf node that overlaps the newly packed rectangle is then split,
not just the node that the rectangle was packed in.

The way the splitting works is that, given the node's overlap with the packed item,
we create as many corner-fitting rectangles we can out of it. For example:

```
+-----+              ..+---+     .......
| |   |              . |   |     .     .
+--   |              . |   |     +-----+
|     | splits into: . |   | and |     |
|     |              . |   |     |     |
|     |              . |   |     |     |
+-----+              ..+---+     +-----+
```

These split nodes are allowed to overlap, so we're not *sub-dividing* the root as
we go, we're creating new potential packing *paths*. Because they are allowed to
overlap, a node splitting might create a leaf that is *entirely contained* within
another node higher up the tree. When this happens, we don't create that leaf node,
since the node higher up on the tree has already fully claimed that space.

As we pack more and more rectangles, these score-checks and collision-checks get
more and more expensive, as the amount of leaf nodes expands quickly. You will often
have 2x or more leaf nodes than total items you are packing.

The tree structure is the thing that allows us to do this so efficiently, and also
pre-sorting the items before packing so we pack the largest items first. When
colliding, we can check collisions with the splits as we go, and each layer of
the tree that we do this will prevent us from ever having to overlap-check huge
amounts of leaf nodes. My earlier versions of this just scanned the whole flat
list of leaf nodes and... it was much slower.

## Contributions
I'm happy to take pull requests if you manage to find ways to make it faster
or more memory-friendly. If you have a non-obvious speed improvement change,
I'd appreciate if you supplied a benchmark with it so I can see the effect.