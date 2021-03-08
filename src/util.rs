use carbon::GraphiteDataPoint;
use prom_remote_write::data::{Label, Sample, TimeSeries};
use prom_remote_write::METRIC_NAME_LABEL;
use std::convert::TryFrom;
use std::time::{SystemTime, UNIX_EPOCH};
fn sanitize_name(name: &str) -> String {
    str::replace(name, ".", "_")
}
pub fn carbon_point_to_prom(metric: &GraphiteDataPoint) -> TimeSeries {
    let mut labels = Vec::<Label>::new();
    let label = Label {
        name: METRIC_NAME_LABEL.to_string(),
        value: sanitize_name(metric.path),
    };
    labels.push(label);
    // Samples f64 i64 value timestamp
    let default_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let mut samples = Vec::<Sample>::new();
    let sample = Sample {
        value: metric.value,
        timestamp: i64::try_from(default_time).unwrap(),
    };
    samples.push(sample);
    TimeSeries { labels, samples }
}
