extern crate image;
use crunch::{Item, PackedItem, PackedItems, Rotation};
use image::{GenericImage, Rgba, RgbaImage};

fn main() {
    println!("loading images...");

    // Load all the files into RGBA images
    let items: [Item<RgbaImage>; 26] = [
        "img/img1.png",
        "img/img2.png",
        "img/img3.png",
        "img/img4.png",
        "img/img5.png",
        "img/img6.png",
        "img/img7.png",
        "img/img8.png",
        "img/img9.png",
        "img/img10.png",
        "img/img11.png",
        "img/img12.png",
        "img/img13.png",
        "img/img13.png",
        "img/img13.png",
        "img/img14.png",
        "img/img14.png",
        "img/img14.png",
        "img/img15.png",
        "img/img15.png",
        "img/img15.png",
        "img/img16.png",
        "img/img16.png",
        "img/img16.png",
        "img/img17.png",
        "img/img17.png",
    ]
    .map(|file| {
        let img = image::open(file).unwrap().to_rgba8();
        let (w, h) = (img.width() as usize, img.height() as usize);
        println!("\tloaded: `{}` ({} x {})", file, w, h);
        Item::new(img, w, h, Rotation::None)
    });

    println!("packing {} images...", items.len());

    // Try packing all the rectangles
    match crunch::pack_into_po2(1024, items) {
        Ok(PackedItems { w, h, items }) => {
            println!("images packed into ({} x {}) rect", w, h);

            // Create a target atlas image to draw the packed images onto
            let mut atlas = RgbaImage::from_pixel(w as u32, h as u32, Rgba([0, 0, 0, 0]));

            // Copy all the packed images onto the target atlas
            for PackedItem { data, rect } in items {
                atlas
                    .copy_from(&data, rect.x as u32, rect.y as u32)
                    .unwrap();
            }

            println!("exporting `packed.png`...");

            // Export the packed atlas
            atlas.save("packed.png").unwrap();
        }
        Err(_) => {
            panic!("failed to pack images");
        }
    }
}
