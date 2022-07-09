use clap::{crate_description, crate_name, crate_version, value_t, App, Arg};
use log::LevelFilter;
use log::{debug, error, info, warn};

use udp_pretend_tcp::logger;

#[tokio::main]
async fn main() {
  log::set_logger(&logger::LOGGER).unwrap();
  log::set_max_level(LevelFilter::Info);

  let matches = App::new(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .arg(
      Arg::with_name("remote-host")
        .short("r")
        .long("remote-host")
        .value_name("REMOTE_HOST")
        .help("the remote host to connect to")
        .required(true),
    )
    .get_matches();

  let remote_host = matches.value_of("remote-host").expect("remote-host is required.");

  info!("remote host read as: {}", remote_host);
}
