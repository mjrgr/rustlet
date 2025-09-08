mod checkerror;

use clap::{Arg, ArgMatches, Command};
use log::{LevelFilter, debug, error, info, warn};
use std::net::{TcpStream, ToSocketAddrs};
use std::process::exit;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::sleep;
use std::time::Duration;

use crate::checkerror::CheckError;

fn main() {
    let shutdown_flag = Arc::new(AtomicBool::new(false));
    let shutdown_flag_clone = Arc::clone(&shutdown_flag);

    if let Err(e) = ctrlc::set_handler(move || {
        warn!("Received termination signal (Ctrl+C), initiating graceful shutdown...");
        shutdown_flag_clone.store(true, Ordering::Relaxed);
    }) {
        error!("Failed to set Ctrl-C handler: {}", e);
        exit(1);
    }

    let matches = init_params();

    // Setup logging with better formatting
    let log_level = matches.get_one::<String>("level").unwrap();
    let level_filter = match log_level.as_str() {
        "debug" => LevelFilter::Debug,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        "trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    };

    env_logger::builder()
        .filter_level(level_filter)
        .format_timestamp_secs()
        .init();

    let interval_secs = *matches.get_one::<u64>("interval").unwrap();
    let timeout_secs = *matches.get_one::<u64>("timeout").unwrap();
    let timeout = Duration::from_secs(timeout_secs);

    let mut tcp_remaining = matches
        .get_many::<String>("tcp")
        .map(|vals| vals.cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    let mut http_remaining = matches
        .get_many::<String>("url")
        .map(|vals| vals.cloned().collect::<Vec<_>>())
        .unwrap_or_default();

    if tcp_remaining.is_empty() && http_remaining.is_empty() {
        info!("No checks to perform, exiting successfully");
        exit(0);
    }

    debug!(
        "Starting health checks (interval: {}s, timeout: {}s)",
        interval_secs, timeout_secs
    );
    debug!(
        "TCP endpoints: {}, HTTP endpoints: {}",
        tcp_remaining.len(),
        http_remaining.len()
    );

    loop_over_checks(
        shutdown_flag,
        interval_secs,
        timeout,
        &mut tcp_remaining,
        &mut http_remaining,
    );
}

fn check_tcp_endpoint(addr: &str, timeout: Duration) -> Result<(), CheckError> {
    let clean_addr = addr.strip_prefix("tcp://").unwrap_or(addr);

    let socket_addrs: Vec<_> = clean_addr
        .to_socket_addrs()
        .map_err(|e| CheckError::InvalidAddress(format!("{}: {}", addr, e)))?
        .collect();

    if socket_addrs.is_empty() {
        return Err(CheckError::InvalidAddress(format!(
            "No addresses resolved for: {}",
            addr
        )));
    }

    // Attempt to connect to the first resolved address
    let socket_addr = socket_addrs[0];
    TcpStream::connect_timeout(&socket_addr, timeout)
        .map_err(|e| CheckError::ConnectionFailed(format!("{}: {}", addr, e)))?;

    debug!(
        "TCP connection successful to {} (resolved to {})",
        addr, socket_addr
    );
    Ok(())
}

fn check_http_endpoint(url: &str, timeout: Duration) -> Result<(), CheckError> {
    debug!("Attempting HTTP check for: {}", url);

    let client = reqwest::blocking::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| CheckError::InvalidAddress(format!("Client creation failed: {}", e)))?;

    let response = client.get(url).send().map_err(|e| {
        error!("HTTP request error for {}: {}", url, e);
        CheckError::RequestFailed(format!("{}: {}", url, e))
    })?;

    debug!("HTTP response status for {}: {}", url, response.status());

    if !response.status().is_success() {
        return Err(CheckError::RequestFailed(format!(
            "{}: HTTP {}",
            url,
            response.status()
        )));
    }

    Ok(())
}

fn loop_over_checks(
    shutdown_flag: Arc<AtomicBool>,
    interval_secs: u64,
    timeout: Duration,
    tcp_remaining: &mut Vec<String>,
    http_remaining: &mut Vec<String>,
) {
    loop {
        if shutdown_flag.load(Ordering::Relaxed) {
            debug!("Graceful shutdown initiated, stopping health checks...");
            exit(130); // Code de sortie standard pour SIGINT (Ctrl+C)
        }

        debug!("Starting health check iteration...");

        // Check TCP endpoints
        tcp_remaining.retain(|addr| match check_tcp_endpoint(addr, timeout) {
            Ok(()) => {
                info!("✅ TCP check succeeded: {}", addr);
                false
            }
            Err(e) => {
                error!("❌ TCP check failed: {:?}", e);
                true
            }
        });

        // Check HTTP endpoints
        http_remaining.retain(|url| match check_http_endpoint(url, timeout) {
            Ok(()) => {
                info!("✅ HTTP check succeeded: {}", url);
                false
            }
            Err(e) => {
                error!("❌ HTTP check failed: {:?}", e);
                true
            }
        });

        if tcp_remaining.is_empty() && http_remaining.is_empty() {
            info!("🎉 All health checks passed successfully!");
            exit(0);
        }

        let remaining_total = tcp_remaining.len() + http_remaining.len();
        warn!(
            "⏳ {} checks remaining, retrying in {}s...",
            remaining_total, interval_secs
        );
        sleep(Duration::from_secs(interval_secs));
    }
}

fn init_params() -> ArgMatches {
    Command::new("rustlet")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Lightweight init container tool")
        .arg(
            Arg::new("level")
                .short('l')
                .long("level")
                .default_value("info")
                .value_parser(["debug", "info", "warn", "error", "trace"])
                .help("Log level"),
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .default_value("5")
                .value_parser(clap::value_parser!(u64).range(1..=300))
                .help("Interval between checks in seconds (1-300)"),
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .default_value("10")
                .value_parser(clap::value_parser!(u64).range(1..=60))
                .help("Timeout for each check in seconds (1-60)"),
        )
        .arg(
            Arg::new("tcp")
                .short('t')
                .long("tcp")
                .num_args(0..)
                .action(clap::ArgAction::Append)
                .help("TCP endpoints to check (host:port)"),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .num_args(0..)
                .action(clap::ArgAction::Append)
                .help("HTTP URLs to check"),
        )
        .get_matches()
}
