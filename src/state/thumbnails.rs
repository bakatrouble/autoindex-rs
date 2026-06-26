use std::hash::{DefaultHasher, Hash, Hasher};
use fchashmap::FcHashMap;
use image::{DynamicImage, EncodableLayout, GenericImageView};
use webpx::{Encoder, Unstoppable};

pub struct Thumbnails {
    map: FcHashMap<u64, Vec<u8>, 1024>,
}

const THUMBNAIL_SIZE: f32 = 200.0;

impl Thumbnails {
    pub fn new() -> Self {
        Self {
            map: FcHashMap::new(),
        }
    }

    pub fn get_thumbnail(&mut self, src_image: &DynamicImage) -> Vec<u8> {
        let mut s = DefaultHasher::new();
        src_image.as_bytes().hash(&mut s);
        let hash = s.finish();

        if self.map.contains_key(&hash) {
            println!("Thumbnail cache hit!");
            return self.map.get(&hash).unwrap().clone();
        }
        println!("Thumbnail cache miss!");

        let size = src_image.dimensions();
        let multiplier = if size.0 > size.1 {
            THUMBNAIL_SIZE / size.1 as f32
        } else {
            THUMBNAIL_SIZE / size.0 as f32
        };
        let dst_size = (
            (size.0 as f32 * multiplier).ceil() as u32,
            (size.1 as f32 * multiplier).ceil() as u32,
        );
        let dst_image = src_image.thumbnail(dst_size.0, dst_size.1).into_rgba8();

        let bytes = Encoder::new_rgba(dst_image.as_bytes(), dst_image.width(), dst_image.height())
            .quality(85.0)
            .encode(Unstoppable).unwrap();
        // let mut bytes = Vec::new();
        // dst_image.write_with_encoder(WebPEncoder::new_lossless(&mut bytes)).unwrap();

        let _ = self.map.insert(hash, bytes.clone());
        bytes
    }
}