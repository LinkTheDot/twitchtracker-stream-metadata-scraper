use clap::Parser;

#[derive(Parser)]
#[command(name = "StreamTimeScraperConfig")]
pub struct ClapArgs {
  #[arg(short = 'n', long)]
  pub user_login: String,

  #[arg(short = 'c', long)]
  pub cookie: String,

  #[arg(short = 'o', long, default_value = "stream_data.csv")]
  pub output_path: String,
}

impl ClapArgs {
  pub fn new() -> Self {
    ClapArgs::parse()
  }
}
