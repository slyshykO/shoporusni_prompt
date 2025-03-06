mod shoporusni;

use crate::shoporusni::{cache, config};
use anyhow::Result;
use clap::Parser;
use colored::control as color_control;
use log::info;
use simplelog::ColorChoice;

#[derive(Debug, Parser)]
struct Cli {
    /// -v for warn, -vv for info, -vvv for debug
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Remote API URL
    #[clap(
        short,
        long,
        value_parser,
        default_value = "https://russianwarship.rip/api/v1/statistics/latest"
    )]
    url: url::Url,

    /// Refresh time for cache
    #[clap(short, long, value_parser, default_value = "30minutes")]
    refresh: humantime::Duration,
}

fn main() -> Result<()> {
    color_control::set_override(true);
    let c: Cli = Cli::parse();
    if atty::is(atty::Stream::Stdout) {
        simplelog::TermLogger::init(
            c.verbose.log_level_filter(),
            Default::default(),
            Default::default(),
            ColorChoice::Always,
        )
        .expect("Logging init error");
    }
    info!("Checking if config directory exists, create if its not");
    let cfg_dir = config::config_dir()?;
    info!("Reading cache");
    let cache = cache::read(cfg_dir, c.refresh)?;
    info!("Getting data from API or cache");
    let data = shoporusni::get_data(c.url, cache)?;
    info!("Printing result");
    shoporusni::print_result(&data);
    Ok(())
}
