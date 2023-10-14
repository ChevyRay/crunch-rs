use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crunch::{Item, Rotation};
use std::path::PathBuf;

pub fn pack_images(c: &mut Criterion) {
    // Get image sizes, ignoring the actual images
    let items: Vec<_> = std::fs::read_dir(PathBuf::from("examples/pack_images/img"))
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_file() {
                let img = image::open(path).unwrap().to_rgba8();
                let (w, h) = (img.width() as usize, img.height() as usize);
                Some(Item::new(0_u8, w, h, Rotation::None))
            } else {
                None
            }
        })
        .collect();

    c.bench_function("pack all images", |b| {
        b.iter(|| crunch::pack_into_po2(1024, black_box(items.clone())))
    });
}

criterion_group!(benches, pack_images);
criterion_main!(benches);
