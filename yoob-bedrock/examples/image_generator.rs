use yoob::client::{ImageGenerationClient, ProviderClient};
use yoob::image_generation::ImageGenerationModel;
use yoob_bedrock::client::Client;
use yoob_bedrock::image::AMAZON_NOVA_CANVAS;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const DEFAULT_PATH: &str = "./output.png";

#[tokio::main]
async fn main() {
    let client = Client::from_env();
    let image_generation_model = client.image_generation_model(AMAZON_NOVA_CANVAS);
    let response = image_generation_model
        .image_generation_request()
        .prompt("A castle sitting upon a large mountain, overlooking the water.")
        .width(512)
        .height(512)
        .send()
        .await;

    // save image
    let mut file = File::create_new(Path::new(&DEFAULT_PATH)).expect("Failed to create file");
    let _ = file.write(&response.unwrap().image);
}
