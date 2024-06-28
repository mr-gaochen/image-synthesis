use image::DynamicImage;

use crate::erros::ApiError;

// 从远程加载图片到内存
pub async fn remote_image_to_memory(url: &str) -> Result<DynamicImage, ApiError> {
    println!("请求远程图片{:?}", url);
    let client = reqwest::Client::new();
    let img_data = client.get(url).send().await.expect("请求失败");
    let body = img_data.bytes().await.expect("无法读取响应体");
    // 使用image库加载图片
    let result = image::load_from_memory(&body);
    match result {
        Ok(img) => Ok(img),
        Err(_) => Err(ApiError::InnerError("加载图片异常".to_string())),
    }
}

// 16进制转rgba
pub fn hex_to_rgba(hex: &str) -> Option<(u8, u8, u8, u8)> {
    // 去除 '#' 字符并限制长度为 6 或 8
    let hex_str = if hex.len() >= 7 && hex.starts_with('#') {
        &hex[1..]
    } else {
        return None;
    };
    let hex_str = if hex_str.len() == 6 || hex_str.len() == 8 {
        hex_str
    } else {
        return None;
    };

    // 解析红色、绿色、蓝色和透明度（如果有）
    let r = u8::from_str_radix(&hex_str[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex_str[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex_str[4..6], 16).ok()?;
    let a = if hex_str.len() == 8 {
        u8::from_str_radix(&hex_str[6..8], 16).ok()?
    } else {
        255 // 如果没有透明度，就默认为完全不透明
    };

    Some((r, g, b, a))
}
