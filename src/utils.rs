use crate::*;

pub async fn fetch_comments<T: ToString>(video_id: T) -> Result<String> {
    let response = reqwest::Client::new()
        .get("https://youtube.googleapis.com/youtube/v3/commentThreads")
        .query(&[
            ("videoId", video_id.to_string()),
            ("part", "snippet".to_string()),
            ("key", "AIzaSyCfkWRCGfrPnv-LjGeS5mLzVt3Qyu5gF_U".to_string()),
        ])
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}
