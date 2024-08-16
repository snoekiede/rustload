pub mod models;
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>
{
    let url = "https://telegraaf.nl/";
    let response = models::load_response::LoadResponse::new();
    let result = response.perform_load(url).await;
    match result {
        Ok(load_response) => {
            println!("URL: {}", url);
            println!("Time: {}", load_response.get_time());
            println!("Status: {}", load_response.get_status());
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }


    Ok(())
}

