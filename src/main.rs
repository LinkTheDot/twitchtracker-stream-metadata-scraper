use stream_time_scraper::{scraper_config::ScraperConfig, stream_data::StreamData};
use crate::clap::ClapArgs;

mod clap;

#[tokio::main]
async fn main() {
  let clap_args = ClapArgs::new();
  let scraper_config = ScraperConfig::new(clap_args.cookie);

  let stream_data = scraper_config
    .grab_stream_data_for_user(&clap_args.user_login)
    .await
    .unwrap();

  StreamData::list_to_spreadsheet(stream_data, clap_args.output_path).unwrap();
}
