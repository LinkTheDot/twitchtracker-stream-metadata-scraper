use super::StreamData;

#[derive(Debug, serde::Serialize)]
pub struct SpreadsheetFormattedStreamData {
  pub date: String,
  pub title: String,

  pub hours_duration: f64,
  pub average_viewers: f64,
  pub max_viewers: f64,
  pub followers: f64,
}

impl From<&StreamData> for SpreadsheetFormattedStreamData {
  fn from(stream_data: &StreamData) -> Self {
    let date = stream_data.date.format("%Y-%m-%d %H:%M:%S").to_string();

    Self {
      date,
      title: stream_data.title.clone(),

      hours_duration: stream_data.minutes_duration as f64 / 60.0,
      average_viewers: stream_data.average_viewers as f64,
      max_viewers: stream_data.max_viewers as f64,
      followers: stream_data.followers as f64,
    }
  }
}
