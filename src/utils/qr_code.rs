use qrcode::QrCode;
use qrcode::render::svg;

pub fn generate_qr_svg(data: &str) -> String {
    let code = QrCode::new(data).unwrap();
    
    code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
}