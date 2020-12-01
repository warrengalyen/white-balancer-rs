use image;

pub fn image_format_from_string(extension: &str) -> Option<image::ImageFormat> {
    match extension {
        "png" |"PNG" => Some(image::ImageFormat::Png),
        "jpg" | "jpeg" | "JPEG" | "JPG" => Some(image::ImageFormat::Jpeg),
        _ => None
    }
}

#[cfg(test)]
mod gray_test {
    use super::*;

    #[test]
    fn test_from_string() {
        assert_eq!(image_format_from_string("png").unwrap(), image::ImageFormat::Png);
        assert_eq!(image_format_from_string("PNG").unwrap(), image::ImageFormat::Png);
        assert_eq!(image_format_from_string("jpg").unwrap(), image::ImageFormat::Jpeg);
        assert_eq!(image_format_from_string("blabla").is_some(), false);
    }
} 