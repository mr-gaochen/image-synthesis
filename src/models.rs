use ab_glyph::{FontRef, PxScale};
use image::{imageops::FilterType::Lanczos3, DynamicImage, GenericImage, ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use serde::{Deserialize, Serialize};

use crate::{constant, utils};

// 根据模版生成图片
/* 请求体
{
    "template_id": 234234234,
    "edit_info": [
        {
            "id": "1",
            "type": "text",
            "catagory": "你好",
            "position": {
                "left": 20,
                "top": 20,
                "width": 100,
                "height": 100
            },
            "style": {
                "fontFamily": "宋体",
                "fontSize": 12,
                "color":"#12123"
            }
        },
        {
            "id": "2",
            "catagory": "text",
            "content": "海报",
            "position": {
                "left": 60,
                "top": 80,
                "width": 100,
                "height": 100
            }
        },
        {
            "id": "3",
            "type": "image",
            "content": "https://ai-baobao.oss-cn-chengdu.aliyuncs.com/test/t1.png",
            "position": {
                "left": 100,
                "top": 120,
                "width": 150,
                "height": 257
            }
        }
    ]
}
*/
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageEdit {
    pub id: String,
    pub catagory: String,
    pub backgroud: Option<bool>,
    pub content: String,
    pub position: Position,
    pub style: Option<Style>,
}

impl ImageEdit {
    pub async fn build_text_layer(&self, img: &mut DynamicImage) -> () {
        // 构建文本图层
        let text = self.content.to_owned();
        let font_info = match &self.style {
            Some(style) => style.build_font(),
            None => (
                constant::RGB_DEFAULT,
                constant::FONT_FAMILY_DEFAULT.to_string(),
                constant::FONT_SIZE_DEFAULT,
            ),
        };
        let color = Rgba([
            font_info.0 .0,
            font_info.0 .1,
            font_info.0 .2,
            font_info.0 .3,
        ]);
        // let font = Vec::from(constant::FONT_FILE);
        let scale = PxScale {
            x: font_info.2,
            y: font_info.2,
        };
        let position: (i32, i32) = (self.position.left, self.position.top);
        let font = FontRef::try_from_slice(include_bytes!("font/NotoSansTC-VariableFont_wght.ttf"))
            .unwrap();
        draw_text_mut(
            img,
            color,
            position.0,
            position.1,
            scale,
            &font,
            text.as_str(),
        );
    }

    pub async fn build_image_layer(&self, img: &mut DynamicImage) -> () {
        println!("当前图片信息:{:?}", self);

        let url = self.content.to_owned();
        let mut dynamic_image = utils::remote_image_to_memory(url.as_str())
            .await
            .expect("加载图片失败");

        // 使用 resize 方法进行缩放，保持宽高比
        dynamic_image = dynamic_image.resize_exact(
            50,
            (50 as f32 * dynamic_image.height() as f32 / dynamic_image.width() as f32) as u32, // 计算新的高度
            Lanczos3,
        );
        let image_rgb: ImageBuffer<Rgba<u8>, Vec<u8>> = dynamic_image.to_rgba8();

        println!("重置大小后的尺寸:{:?}", image_rgb.dimensions());

        let position: (u32, u32) = (self.position.left as u32, self.position.top as u32);
        let result = img.copy_from(&image_rgb, position.0, position.1);

        match result {
            Ok(_) => println!("copy 完成"),
            Err(e) => println!("========{:?}", e),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Position {
    pub left: i32,
    pub top: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Style {
    pub font_family: Option<String>,
    pub font_size: Option<f32>,
    pub color: Option<String>,
}

impl Style {
    pub fn build_font(&self) -> ((u8, u8, u8, u8), String, f32) {
        let rgb = match &self.color {
            Some(color) => utils::hex_to_rgba(color.as_str()).unwrap(),
            None => (0, 0, 0, 255),
        };
        let font_family = match self.font_family.to_owned() {
            Some(font) => font,
            None => "NotoSansTC-VariableFont_wght.ttf".to_string(),
        };
        let font_size = match self.font_size {
            Some(size) => size,
            None => constant::FONT_SIZE_DEFAULT,
        };
        (rgb, font_family, font_size)
    }
}
