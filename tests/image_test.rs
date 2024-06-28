use image_synthesis::models::ImageEdit;

#[cfg(test)]
mod tests {
    use image_synthesis::client::ImageClient;

    use crate::get_data_from_json;

    #[tokio::test]
    async fn test_images_download() {
        let vec_img_edit = get_data_from_json().await;
        let mut client = ImageClient::new(&vec_img_edit);
        let result = client.builder().exec().await;
        match result {
            Ok(_) => todo!(),
            Err(e) => println!("{:?}", e),
        }
    }
}

async fn get_data_from_json() -> Vec<ImageEdit> {
    let json = r#"[{"id":"1","catagory":"text","content":"你好","position":{"left":20,"top":20,"width":100,"height":100},"style":{"fontFamily":"NotoSansTC-VariableFont_wght.ttf","fontSize":12}},{"id":"2","catagory":"text","content":"大大的海报","position":{"left":60,"top":80,"width":100,"height":100}},{"id":"3","catagory":"image","backgroud":true,"content":"网络图片地址","position":{"left":100,"top":120,"width":150,"height":257}},{"id":"4","catagory":"image","backgroud":false,"content":"网络图片地址","position":{"left":100,"top":300,"width":150,"height":257}}]"#;
    let resource: Vec<ImageEdit> = serde_json::from_str(json).unwrap();
    resource
}
