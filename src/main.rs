use clap::{Arg, Command};
use log::{LevelFilter, info, warn, debug};
use std::net::TcpStream;
use std::process::exit;
use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (tx, rx) = channel();
    ctrlc::set_handler(move || {
        warn!("Received termination signal, shutting down...");
        tx.send(()).expect("Failed to send termination signal");
    })
    .expect("Error setting Ctrl-C handler");

    let matches = Command::new("")
        .version("1.0")
        .arg(
            Arg::new("level")
                .short('l')
                .long("level")
                .default_value("info")
                .help("Log level"),
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .default_value("5")
                .help("Interval between checks in seconds"),
        )
        .arg(
            Arg::new("tcp")
                .short('t')
                .long("tcp")
                .num_args(0..)
                .action(clap::ArgAction::Append),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .num_args(0..)
                .action(clap::ArgAction::Append),
        )
        .get_matches();

    // Logging setup
    let log_level = matches.get_one::<String>("level").unwrap();
    let level_filter = match log_level.to_lowercase().as_str() {
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };
    env_logger::builder().filter_level(level_filter).init();

    let interval_secs: u64 = matches.get_one::<String>("interval").unwrap().parse().unwrap_or(5);
    let mut tcp_remaining = matches.get_many::<String>("tcp").map(|vals| vals.cloned().collect::<Vec<_>>()).unwrap_or_default();
    let mut http_remaining = matches.get_many::<String>("url").map(|vals| vals.cloned().collect::<Vec<_>>()).unwrap_or_default();

    let all_empty = tcp_remaining.is_empty() && http_remaining.is_empty();

    if all_empty {
        info!("No checks to do.");
        exit(0);
    }

    loop {
        if rx.try_recv().is_ok() {
            debug!("Interrupted by user");
            break;
        }
        tcp_remaining.retain(|addr| {
            match TcpStream::connect_timeout(&addr.parse().unwrap(), Duration::from_secs(3)) {
                Ok(_) => {
                    info!("TCP check succeeded for {}", addr);
                    false
                }
                Err(_) => {
                    warn!("TCP check failed for {}", addr);
                    true
                }
            }
        });

        http_remaining.retain(|url| {
            let client = reqwest::blocking::Client::new();
            match client.get(url).timeout(Duration::from_secs(3)).send() {
                Ok(resp) if resp.status().is_success() => {
                    info!("HTTP check succeeded for {}", url);
                    false
                }
                _ => {
                    warn!("HTTP check failed for {}", url);
                    true
                }
            }
        });

        let all_done = tcp_remaining.is_empty() && http_remaining.is_empty();

        if all_done {
            info!("All checks passed.");
            exit(0);
        } else {
            warn!(
                "Some checks failed, retrying in {} seconds...",
                interval_secs
            );
            sleep(Duration::from_secs(interval_secs));
        }
    }
}