use anyhow::anyhow;
use chrono::DateTime;
use spreadsheet_formatted_stream_data::SpreadsheetFormattedStreamData;
use std::path::Path;

mod spreadsheet_formatted_stream_data;

#[derive(Debug)]
pub struct StreamData {
  pub date: DateTime<chrono_tz::Tz>,
  pub title: String,

  pub minutes_duration: usize,
  pub average_viewers: usize,
  pub max_viewers: usize,
  pub followers: usize,
}

impl StreamData {
  pub fn list_to_spreadsheet<P: AsRef<Path>>(
    stream_data_list: Vec<Self>,
    output_path: P,
  ) -> anyhow::Result<()> {
    let mut csv_writer = csv::Writer::from_path(output_path)?;

    for row in &stream_data_list {
      let row = SpreadsheetFormattedStreamData::from(row);

      if let Err(error) = csv_writer.serialize(&row) {
        println!("Failed to parse row. Reason: {:?}. Row:\n{:?}", error, row);
      }
    }

    if let Err(error) = csv_writer.flush() {
      return Err(anyhow!("Failed to flush csv writer. Reason: `{:?}`", error));
    }

    Ok(())
  }
}
