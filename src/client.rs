use image::{DynamicImage, ImageBuffer, Rgba};

use crate::{erros::ApiError, models::ImageEdit, utils};

#[async_trait::async_trait]
pub trait ImageSystehesisTrait: Send + Sync {
    // 执行图片合成
    async fn do_proess(&self) -> Result<(), ApiError>;
}

pub struct ImageParamBuilder {
    pub edit_info: Vec<ImageEdit>,
}

impl ImageParamBuilder {
    pub fn new(param: &Vec<ImageEdit>) -> Self {
        ImageParamBuilder {
            edit_info: param.to_owned(),
        }
    }
    // 获取底图
    fn get_backgroud(&self) -> Option<String> {
        self.edit_info
            .iter()
            .find(|img| img.backgroud == Some(true))
            .map(|edit| edit.content.clone())
    }

    // 构造图层
    async fn builder_layer(&self, img: &mut DynamicImage) {
        for i in 0..self.edit_info.len() {
            let ele = &self.edit_info[i];
            if ele.catagory == "text" {
                ele.build_text_layer(img).await;
            } else if ele.catagory == "image" && ele.backgroud != Some(true) {
                // 图片
                ele.build_image_layer(img).await;
            }
        }
    }
}

#[async_trait::async_trait]
impl ImageSystehesisTrait for ImageParamBuilder {
    async fn do_proess(&self) -> Result<(), ApiError> {
        // 背景图片
        let bg_images = self.get_backgroud();
        match bg_images {
            Some(bg) => {
                let bg_dynamic_image = utils::remote_image_to_memory(bg.as_str()).await?;
                let background: ImageBuffer<Rgba<u8>, Vec<u8>> = bg_dynamic_image.to_rgba8();
                // 创建一个新的ImageBuffer，这里我们使用背景图片的尺寸
                let (width, height) = background.dimensions();
                println!("底图的尺寸{:?}", background.dimensions());
                let bg_image_buffer =
                    ImageBuffer::from_fn(width, height, |x, y| *background.get_pixel(x, y));
                let mut dy_img: DynamicImage = bg_image_buffer.into();
                // 执行 图层拼装
                self.builder_layer(&mut dy_img).await;
                match dy_img.save("output_with_background.png") {
                    Ok(_) => println!("执行完毕"),
                    Err(_) => print!("异常"),
                };
                Ok(())
            }
            None => {
                return Err(ApiError::MissingParamsError("背景图不能少".to_string()));
            }
        }
    }
}

pub struct SysnthesisBuilder<'a> {
    client: &'a (dyn ImageSystehesisTrait + Send + Sync),
}
impl<'a> SysnthesisBuilder<'a> {
    pub fn new(client: &'a (dyn ImageSystehesisTrait + Send + Sync)) -> Self {
        SysnthesisBuilder { client }
    }
    pub async fn exec(self) -> Result<(), ApiError> {
        self.client.do_proess().await
    }
}

pub struct ImageClient {
    client: Box<dyn ImageSystehesisTrait + Send + Sync>,
}
impl ImageClient {
    /// Creates a new `LlmClient` instance with the specified `ClientLlm` variant and API key.
    pub fn new(params: &Vec<ImageEdit>) -> Self {
        let client: Box<dyn ImageSystehesisTrait + Send + Sync> =
            Box::new(ImageParamBuilder::new(params));
        ImageClient { client }
    }

    pub fn builder(&mut self) -> SysnthesisBuilder {
        SysnthesisBuilder::new(self.client.as_ref())
    }
}
