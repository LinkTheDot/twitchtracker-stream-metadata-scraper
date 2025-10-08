use crate::stream_data::StreamData;
use anyhow::anyhow;
use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::{Europe::London, US::Eastern};
use scraper::{ElementRef, Html, Selector};

pub struct ScraperConfig {
  reqwest_client: reqwest::Client,
  user_cookie: String,
}

impl ScraperConfig {
  const TWITCH_TRACKER_URL: &str = "https://twitchtracker.com/{user_login}/streams";

  pub fn new(user_cookie: String) -> Self {
    let reqwest_client = reqwest::Client::new();

    Self {
      reqwest_client,
      user_cookie,
    }
  }

  pub async fn grab_stream_data_for_user(
    &self,
    user_login: &str,
  ) -> anyhow::Result<Vec<StreamData>> {
    let response_body = self.get_stream_list_html_document(user_login).await?;
    let row_selector = Selector::parse("tbody tr").unwrap();
    let mut streams = vec![];

    for (iteration, row_element) in response_body.select(&row_selector).enumerate() {
      match Self::parse_row_element(row_element) {
        Ok(stream_data) => streams.push(stream_data),
        Err(error) => {
          println!("Failed to get row element `{iteration}`. Reason: `{error}`");
        }
      }
    }

    Ok(streams)
  }

  async fn get_stream_list_html_document(&self, user_login: &str) -> anyhow::Result<Html> {
    let tracker_url = Self::get_tracker_url_for_user(user_login);

    let user_agent =
      "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:143.0) Gecko/20100101 Firefox/143.0";

    let response = self
      .reqwest_client
      .get(&tracker_url)
      .header("User-Agent", user_agent)
      .header("Cookie", &self.user_cookie)
      .header("DNT", "1")
      .header("Priority", "u=0, i")
      .send()
      .await?;

    let status = response.status();

    if !status.is_success() {
      return Err(anyhow!(
        "Response status failed. Error code: {}",
        status.as_u16()
      ));
    }

    let response_body = response.text().await?;

    Ok(Html::parse_document(&response_body))
  }

  fn get_tracker_url_for_user(user_login: &str) -> String {
    Self::TWITCH_TRACKER_URL.replace("{user_login}", user_login)
  }

  fn parse_row_element(row_element: ElementRef) -> anyhow::Result<StreamData> {
    let data_selector = Selector::parse("td span").unwrap();
    let title_selector = Selector::parse("td.status.hidden-xs").unwrap();

    let mut data_spans = row_element.select(&data_selector);

    let Some(datetime_element) = data_spans.next() else {
      return Err(anyhow!("Failed to get datetime from element."));
    };
    let datetime_string = Self::string_from_element(datetime_element);
    let datetime = Self::convert_timestamp_to_timezone(&datetime_string, London)?;

    let Some(duration_in_minutes_element) = data_spans.next() else {
      return Err(anyhow!("Failed to get stream duration from element."));
    };
    let duration_in_minutes = Self::int_from_element(duration_in_minutes_element)?;

    let Some(average_viewers_element) = data_spans.next() else {
      return Err(anyhow!("Failed to get average viewers from element."));
    };
    let average_viewers = Self::int_from_element(average_viewers_element)?;

    let Some(max_viewers_element) = data_spans.next() else {
      return Err(anyhow!("Failed to get max viewers from element."));
    };
    let max_viewers = Self::int_from_element(max_viewers_element)?;

    let Some(followers_element) = data_spans.next() else {
      return Err(anyhow!("Failed to get followers from element."));
    };
    let followers = Self::int_from_element(followers_element)?;

    println!("Getting title.");
    let Some(title_element) = row_element.select(&title_selector).next() else {
      return Err(anyhow!("Failed to get title from element."));
    };
    let title = Self::string_from_element(title_element);
    println!("Got `{title}` for title.");

    Ok(StreamData {
      date: datetime,
      title,

      minutes_duration: duration_in_minutes,
      average_viewers,
      max_viewers,
      followers,
    })
  }

  fn string_from_element(element: ElementRef<'_>) -> String {
    element.text().collect()
  }

  fn int_from_element(element: ElementRef<'_>) -> anyhow::Result<usize> {
    element
      .text()
      .collect::<String>()
      .parse()
      .map_err(Into::into)
  }

  fn convert_timestamp_to_timezone(
    timestamp_string: &str,
    timezone: chrono_tz::Tz,
  ) -> anyhow::Result<chrono::DateTime<chrono_tz::Tz>> {
    let format = "%Y-%m-%d %H:%M";

    let naive_dt = NaiveDateTime::parse_from_str(timestamp_string, format)?;
    let eastern_time = Eastern.from_local_datetime(&naive_dt).unwrap();

    Ok(eastern_time.with_timezone(&timezone))
  }

}
