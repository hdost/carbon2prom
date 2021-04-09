#[macro_use]
extern crate lazy_static;

mod settings;
mod util;

use clap::{App, Arg, ArgMatches};
use env_logger::Builder;
use log::LevelFilter;
use log::{error, info, warn};
use prom_remote_write::client;
use prom_remote_write::data::{MetricMetadata, WriteRequest};
use std::{net::SocketAddr, sync::Arc};
use std::io::Cursor;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::sync::mpsc;

fn get_args() -> ArgMatches<'static> {
    let app = App::new("carbon2prom")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Harold A. Dost III <github@hdost.com>")
        .about("Accepts Line Protocol and writes out to Prometheus remote_write destinations.")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .default_value("config.yaml")
                .help("Sets a configuration file.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Turns up verbosity"),
        );

    app.get_matches()
}
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Builder::new().filter(None, LevelFilter::Info).init();

    let args = get_args();

    if let Some(config_file) = &args.value_of("config") {
        let config_file = config_file.clone().to_string();
        let config_fut = tokio::spawn(async move { settings::watch_config(config_file).await });
        // TODO: Move this into the configuration.
        let addr = "[::]:2003";
        let tcp_fut = tokio::spawn(async move { start_listening_tcp(addr).await });
        let udp_fut = tokio::spawn(async move { start_listening_udp(addr).await });
        tokio::join!(config_fut, tcp_fut, udp_fut);

        // TODO: Handle the Tokio Errs
    }
    Ok(())
}

async fn start_listening_tcp(addr:&str) {
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("Listening TCP...");
    loop {
        let (socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process_tcp(socket,addr).await;
        });
    }
}

async fn start_listening_udp(addr:&str) {
    let socket = UdpSocket::bind(addr.parse::<SocketAddr>().unwrap())
        .await
        .unwrap();
    info!("Listening UDP...");

    let r = Arc::new(socket);
    // TODO: Size the channel appropriately
    // Potentially make this a configurable parameter.
    let (tx, mut rx) = mpsc::channel::<(Vec<u8>, SocketAddr)>(1_000);

    tokio::spawn(async move {
        while let Some((bytes, addr)) = rx.recv().await {
            process_udp(bytes, addr).await;
        }
    });
    let mut buf = [0; 1024];
    loop {
        if let Ok((len, addr)) = r.recv_from(&mut buf).await {
            tx.send((buf[..len].to_vec(), addr)).await.unwrap();
        }
    }
}

async fn process_tcp(socket: TcpStream, addr: SocketAddr) {
    let mut read = BufReader::new(socket);
    let mut buf = String::new();
    while let Ok(count) = read.read_line(&mut buf).await {
        if count == 0 {
            break;
        }
        process_metric(&buf,addr).await;
    }
}

async fn process_udp(bytes: Vec<u8>, addr: SocketAddr) {
    let mut read = Cursor::new(bytes);
    let mut buf = String::new();

    while let Ok(count) = read.read_line(&mut buf).await {
        if count == 0 {
            break;
        }
        process_metric(&buf,addr).await;

    }
}

async fn process_metric(buf: &String, addr: SocketAddr){
        match carbon::GraphiteDataPoint::from(buf) {
            Ok(metric) => {

                // Match the paths in the map. Using consistent aggregated mapping
                // Push into the transformation queue.
                //
                let series = vec![util::carbon_point_to_prom(&metric)];
                let write_request = WriteRequest {
                    timeseries: series,
                    metadata: Vec::<MetricMetadata>::new(),
                };
                let res = client::write_metrics(&write_request).await;
                info!("{:?}", res);
            }
            Err(e) => {
                error!("{} \"{}\"", e, buf);
            }
        }

}
