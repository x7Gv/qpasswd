use bishop::*;

pub fn random_art(data: &[u8]) {

    let opts = DrawingOptions {
        top_text: "scrypt".to_string(),
        bottom_text: "derived key".to_string(),
        ..Default::default()
    };

    let field = BishopArt::new().chain(data).result();

    println!("\n{}", field.draw_with_opts(&opts));
}
