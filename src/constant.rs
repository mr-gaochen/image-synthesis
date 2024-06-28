// 默认颜色
pub const RGB_DEFAULT: (u8, u8, u8, u8) = (0, 0, 0, 255);

// 默认字体
pub const FONT_FAMILY_DEFAULT: &str = "NotoSansTC-VariableFont_wght.ttf";

pub const FONT_FILE: &[u8] = include_bytes!("font/NotoSansTC-VariableFont_wght.ttf") as &[u8];

// 默认字体大小
pub const FONT_SIZE_DEFAULT: f32 = 14.0;
