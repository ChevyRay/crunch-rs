extern crate image;
use crunch::{Item, Rect, Rotation};
use image::{RgbaImage, ImageBuffer, GenericImageView, GenericImage};

fn main() {

    // The images we'll be loading
    let img_paths = [
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
    ];

    println!("loading images...");

    // Load all the files into RGBA images
    let images: Vec<RgbaImage> = img_paths.iter().map(|file| {
        let img = image::open(file).unwrap().to_rgba8();
        println!("\tloaded: `{}` ({} x {})", file, img.width(), img.height());
        img
    }).collect();

    // Create a packing item for every image using its size
    let items: Vec<Item<&RgbaImage>> = images.iter().enumerate().map(|(i, img)| {
        Item::new(img, img.width() as usize, img.height() as usize, Rotation::None)
    }).collect();

    println!("packing {} images...", items.len());

    // Try packing all the rectangles
    match crunch::pack_into_po2(1024, items) {
        Ok((w, h, packed)) => {

            println!("images packed into ({} x {}) rect", w, h);

            // Create a target atlas image to draw the packed images onto
            let mut atlas: RgbaImage = ImageBuffer::from_fn(w as u32, h as u32, |_, _| image::Rgba([0, 0, 0, 0]));

            // Copy all the packed images onto the target atlas
            for (rect, img) in &packed {
                let view = img.view(0, 0, img.width(), img.height());
                atlas.copy_from(&view, rect.x as u32, rect.y as u32);
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
