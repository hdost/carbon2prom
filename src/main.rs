mod util;
use carbon;
use log::{info,warn};
use prom_remote_write::client;
use prom_remote_write::data::{MetricMetadata, TimeSeries, WriteRequest};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
//use bytes::Bytes;

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:2003").await.unwrap();
    let udp = UdpSocket::bind("127.0.0.1:2003").await.unwrap();

    println!("Listening...");
    loop {
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    let mut read = BufReader::new(socket);
    let mut buf = String::new();
    while let Ok(count) = read.read_line(&mut buf).await {
        if count == 0 {
            break;
        }
        match carbon::GraphiteDataPoint::from(&buf) {
            Ok(metric) => {
                let mut series = Vec::<TimeSeries>::new();
                series.push(util::carbon_point_to_prom(&metric));
                let write_request = WriteRequest {
                    timeseries: series,
                    metadata: Vec::<MetricMetadata>::new(),
                };
                let res = client::write_metrics(&write_request).await;
                println!("{:?}",res);
            }
            Err(e) => {
                println!("{} \"{}\"", e, buf);
            }
        }
    }
}
