extern crate reqwest;
extern crate snap;
pub const METRIC_NAME_LABEL: &str = "__name__";
pub mod client {
    use crate::data::WriteRequest;
    use prost::Message;
    use reqwest;
    use reqwest::header;
    use snap;
    static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
    static REMOTE_WRITE_VERSION: &str = "0.1.0";

    fn serialize_write_request(request: &WriteRequest) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(request.encoded_len());

        request.encode(&mut buf).unwrap();
        buf
    }

    pub async fn write_metrics(request: &WriteRequest) -> Result<reqwest::Response,reqwest::Error> {
        let mut encoder = snap::raw::Encoder::new();
        let request = serialize_write_request(&request);
        let request = encoder.compress_vec(&request).unwrap();

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Encoding",
            header::HeaderValue::from_static("snappy"),
        );
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/x-protobuf"),
        );
        headers.insert(
            "X-Prometheus-Remote-Write-Version",
            header::HeaderValue::from_static(REMOTE_WRITE_VERSION),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .user_agent(APP_USER_AGENT)
            .build().unwrap();

        client
            .post("http://localhost:9090/api/v1/write")
            .body(request)
            .send().await
    }
}

pub mod data {
    include!(concat!(env!("OUT_DIR"), "/prometheus.rs"));

    pub use metric_metadata::MetricType;

    impl MetricType {
        pub fn as_str(&self) -> &'static str {
            match self {
                MetricType::Counter => "counter",
                MetricType::Gauge => "gauge",
                MetricType::Gaugehistogram => "gaugehistogram",
                MetricType::Histogram => "histogram",
                MetricType::Info => "info",
                MetricType::Stateset => "stateset",
                MetricType::Summary => "summary",
                MetricType::Unknown => "unknown",
            }
        }
    }
}
