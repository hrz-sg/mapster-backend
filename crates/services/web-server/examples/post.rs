use lib_core::model::ModelManager;
use reqwest::multipart;

type Result<T> = core::result::Result<T, Error>;
type Error = Box<dyn std::error::Error>;

const BASE_URL: &str = "http://localhost:8080";
const ASSETS_DIR: &str = "crates/services/web-server/examples/assets";

pub fn create_client() -> reqwest::Client {
    reqwest::Client::new()
}

async fn print_response(response: reqwest::Response) -> Result<()> {
    println!("Status: {}", response.status());
    println!("Headers: {:#?}", response.headers());
    println!("Body: {}", response.text().await?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let _mm = ModelManager::new().await?;

    let client = create_client();

    println!("=== Testing Post (multipart upload) ===");

    // -- Create multipart form
    let form = multipart::Form::new()
        .text("title", "Post title")
        .text("description", "Post description")
        .file("files[]", format!("{}/image-1.jpg", ASSETS_DIR)).await?
        .file("files[]", format!("{}/image-2.jpg", ASSETS_DIR)).await?
;

    // -- Send POST-request
    println!("\nSending multipart POST request...");
    let response = client
        .post(&format!("{}/api/posts", BASE_URL))
        .multipart(form)
        .send()
        .await?;

    print_response(response).await?;

    println!("\nPost upload test completed successfully!");
    Ok(())
}
