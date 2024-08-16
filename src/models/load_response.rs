pub struct LoadResponse {
    time: f64,
    status: reqwest::StatusCode,
}

impl LoadResponse {
    pub fn new() -> Self {
        LoadResponse {
            time: 0.0,
            status: reqwest::StatusCode::OK,
        }
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_status(&self) -> reqwest::StatusCode {
        self.status
    }

    pub async fn perform_load(&self,url:&str) -> Result<LoadResponse, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        let resp = reqwest::get(url).await?;
        let time = start.elapsed().as_millis() as f64;
        Ok(LoadResponse {
            time: time,
            status: resp.status(),
        })
    }
}