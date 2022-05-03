use eframe::egui;
use eframe::IconData;
use image;

#[allow(dead_code)]
pub struct Image {
    size: (usize, usize),
    pixels: Vec<egui::Color32>,
}

#[allow(dead_code)]
impl Image {
    pub fn open_icon_data(path: &str) -> std::option::Option<IconData> {
        let image_buffer = image::open(path).unwrap();
        let img = image_buffer.to_rgba8();
        let size = (img.width() as u32, img.height() as u32);
        let pixels = img.into_vec();
        let icon_data = eframe::IconData {
            rgba: pixels,
            width: size.0,
            height: size.1,
        };
        Some(icon_data)
    }
    pub fn open_image(path: &str) -> Option<Image> {
        let image_buffer = image::open(path).unwrap();
        let img = image_buffer.to_rgb8();
        let size = (img.width() as usize, img.height() as usize);
        let pixels = img.into_vec();
        let pixels = pixels
            .chunks(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();
        Some(Image { size, pixels })
    }

    pub fn decode(bytes: &[u8]) -> Option<Image> {
        //use image::GenericImageView;
        let image = image::load_from_memory(bytes).ok()?;
        let image_buffer = image.to_rgba8();
        let size = (image.width() as usize, image.height() as usize);
        let pixels = image_buffer.into_vec();
        assert_eq!(size.0 * size.1 * 4, pixels.len());
        let pixels = pixels
            .chunks(4)
            .map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
            .collect();

        Some(Image { size, pixels })
    }
}
