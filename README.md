# Crunch
A packer for cramming lots of rectangles into a larger one, designed primarily with sprite packing in mind. Written in Rust.

## Example

```rust
use crunch::*;
use std::iter::*;

fn main() {

    let container = Rect::of_size(15, 15);
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

    let (result, packed) = pack(container, items);
    assert_eq!(result, Ok(()));

    let mut data : Vec<char> =
        repeat(repeat('.').take(container.w).chain(once('\n')))
        .take(container.h)
        .flatten()
        .collect();
    for (r, chr) in &packed {
        for x in r.x..r.right() {
            for y in r.y..r.bottom() {
                data[y * (container.w + 1) + x] = *chr;
            }
        }
    }

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