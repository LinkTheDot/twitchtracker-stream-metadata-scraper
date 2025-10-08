use stream_time_scraper::{scraper_config::ScraperConfig, stream_data::StreamData};

const GET_DATA_FOR_USER: &str = "fallenshadow";
const USER_COOKIE: &str = "cf_clearance=9r0vob0SDVG1qmMRGXySY4bgd_52xD0e_xg1RH2aLnQ-1758412572-1.2.1.1-pMks4oSYF_WW.m6DqyNI_d5kNLoud33TsHgfUguRniFnFiHSakFm2tzDFwg1ZAtFv2DJazseBzxUYoKE8NqXQwAT_g_DI8Nv5uvP9Nu8wSIVu8vvt3UCPzArQfbfNqYDwa7vEEhhs3F2DEWY8EAn8UaAhkCrorfwuSdpCRdU8L.iqi6OLGtt1guC95GPRBMGL9Ddwl02S_BCBjbyHcRngA.2bjL0CXbTePblo.V.n.s";
const CSV_OUTPUT_PATH: &str = "stream_data.csv";

#[tokio::main]
async fn main() {
  let user_cookie = USER_COOKIE.to_string();
  let scraper_config = ScraperConfig::new(user_cookie);

  let stream_data = scraper_config
    .grab_stream_data_for_user(GET_DATA_FOR_USER)
    .await
    .unwrap();

  StreamData::list_to_spreadsheet(stream_data, CSV_OUTPUT_PATH).unwrap();
}
