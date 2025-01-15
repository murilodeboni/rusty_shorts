use image::{DynamicImage, GenericImage, GenericImageView, Luma, Rgba};
use image::imageops::{overlay, FilterType};
use qrcode::QrCode;
use qrcode::render::svg;
use std::fs::File;
use std::io::BufWriter;

pub fn generate_qr_svg(data: &str) -> String {
    let code = QrCode::new(data).unwrap();
    
    code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}

pub fn generate_qr_with_logo_library(data: &str, logo_path: Option<&str>, output_path: &str) {
   let code = QrCode::new(data).expect("Failed to generate QR code");
    
    let module_dims = 10;

    let qr_image = code
        .render::<Luma<u8>>()
        .module_dimensions(module_dims, module_dims)
        .quiet_zone(true) 
        .build();

    let width = qr_image.width();
    let height = qr_image.height();
    let mut qr_dynamic = DynamicImage::new_rgba8(width, height);
    
    for (x, y, pixel) in qr_image.enumerate_pixels() {
        let val = pixel[0];
        let rgba = if val == 0 {
            Rgba([0, 0, 0, 255])       // black
        } else {
            Rgba([255, 255, 255, 255]) // white
        };
        qr_dynamic.put_pixel(x, y, rgba);
    }
    
    if let Some(path) = logo_path {
        put_image_on_qr_code(&mut qr_dynamic, path, height)
    }
    
    let output_file = File::create(output_path).expect("Failed to create output file");
    let mut writer = BufWriter::new(output_file);
    qr_dynamic
        .write_to(&mut writer, image::ImageFormat::Png)
        .expect("Failed to write output image");
}

fn put_image_on_qr_code(qr_dynamic: &mut DynamicImage, logo_path: &str, qr_height_width: u32) {
    let logo = image::open(logo_path).expect("Failed to load logo");
    let (logo_width, logo_height) = logo.dimensions();
    
    // 30% or more risks obstructing essential QR data.
    let logo_target_width = (qr_height_width as f32 * 0.20) as u32;
    let aspect = logo_height as f32 / logo_width as f32;
    let logo_target_height = (logo_target_width as f32 * aspect) as u32;
    
    // High-quality downscaling
    let logo_resized = logo.resize(
        logo_target_width,
        logo_target_height,
        FilterType::Lanczos3
    );

    let x_offset = ((qr_height_width - logo_target_width) / 2) as i64;
    let y_offset = ((qr_height_width - logo_target_height) / 2) as i64;
    overlay(qr_dynamic, &logo_resized, x_offset, y_offset);

}