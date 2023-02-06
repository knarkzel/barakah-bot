use crate::*;

pub async fn fetch_comments<T: ToString>(video_id: T) -> Result<String> {
    let response = reqwest::Client::new()
        .get("https://youtube.googleapis.com/youtube/v3/commentThreads")
        .query(&[
            ("videoId", video_id.to_string()),
            ("part", "snippet,replies".to_string()),
            ("key", "AIzaSyCfkWRCGfrPnv-LjGeS5mLzVt3Qyu5gF_U".to_string()),
        ])
        .send()
        .await?
        .text()
        .await?;
    Ok(response)
}

pub async fn fetch_videos<T: ToString>(channel_id: T) -> Result<Vec<String>> {
    let response: types::Videos = reqwest::Client::new()
        .get("https://www.googleapis.com/youtube/v3/search")
        .query(&[
            ("channelId", channel_id.to_string()),
            ("part", "snippet".to_string()),
            ("key", "AIzaSyCfkWRCGfrPnv-LjGeS5mLzVt3Qyu5gF_U".to_string()),
            ("order", "date".to_string()),
            ("max_results", "1000".to_string()),
        ])
        .send()
        .await?
        .json()
        .await?;
    let ids = response
        .items
        .into_iter()
        .map(|it| it.id.video_id)
        .collect();
    Ok(ids)
}
