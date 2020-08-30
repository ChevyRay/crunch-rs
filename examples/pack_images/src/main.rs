extern crate image;
use crunch::{ pack, Item, Rotation, Rect };
use image::*;

fn main() {

    let mut items = Vec::new();
    for _ in 0..2 {
        for i in 2..26 {
            for j in 2..26 {
                let w = i;
                let h = j;
                items.push(Item::new((), w, h, Rotation::Allowed));
            }
        }
    }
    for _ in 0..48 {
        items.push(Item::new((), 4, 4, Rotation::Allowed));
    }

    let container = Rect::of_size(500, 426);

    match pack(container, items) {
        Ok(packed) => {

            let w = container.w as u32;
            let h = container.h as u32;
            let mut img: RgbImage = ImageBuffer::from_fn(w, h, |_, _| image::Rgb([0, 0, 0]));
            for (r, _) in &packed {
                for x in r.x..(r.right() - 1) {
                    for y in r.y..(r.bottom() - 1) {
                        img.put_pixel(x as u32, y as u32, image::Rgb([128, 128, 128]));
                    }
                }
            }
            img.save("packed.png").unwrap();

        }
        Err(_) => panic!("packing failed")
    };
}
