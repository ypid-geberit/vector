use crate::event::{Metric, MetricValue};
use async_graphql::Object;
use chrono::{DateTime, Utc};

pub struct ProcessedEventsTotal(Metric);

impl ProcessedEventsTotal {
    pub fn new(m: Metric) -> Self {
        Self(m)
    }

    pub fn get_timestamp(&self) -> Option<DateTime<Utc>> {
        self.0.timestamp()
    }

    pub fn get_processed_events_total(&self) -> f64 {
        match self.0.value() {
            MetricValue::Counter { value } => *value,
            _ => 0.00,
        }
    }
}

#[Object]
impl ProcessedEventsTotal {
    /// Metric timestamp
    pub async fn timestamp(&self) -> Option<DateTime<Utc>> {
        self.get_timestamp()
    }

    /// Total number of events processed
    pub async fn processed_events_total(&self) -> f64 {
        self.get_processed_events_total()
    }
}

impl From<Metric> for ProcessedEventsTotal {
    fn from(m: Metric) -> Self {
        Self(m)
    }
}

pub struct ComponentProcessedEventsTotal {
    name: String,
    metric: Metric,
}

impl ComponentProcessedEventsTotal {
    /// Returns a new `ComponentProcessedEventsTotal` struct, which is a GraphQL type. The
    /// component id is hoisted for clear field resolution in the resulting payload
    pub fn new(metric: Metric) -> Self {
        let name = metric.tag_value("component_id").expect(
            "Returned a metric without a `component_id`, which shouldn't happen. Please report.",
        );

        Self { name, metric }
    }
}

#[Object]
impl ComponentProcessedEventsTotal {
    /// Component name
    async fn name(&self) -> &str {
        &self.name
    }

    /// Events processed total metric
    async fn metric(&self) -> ProcessedEventsTotal {
        ProcessedEventsTotal::new(self.metric.clone())
    }
}

pub struct ComponentProcessedEventsThroughput {
    name: String,
    throughput: i64,
}

impl ComponentProcessedEventsThroughput {
    /// Returns a new `ComponentProcessedEventsThroughput`, set to the provided name/throughput values
    pub fn new(name: String, throughput: i64) -> Self {
        Self { name, throughput }
    }
}

#[Object]
impl ComponentProcessedEventsThroughput {
    /// Component name
    async fn name(&self) -> &str {
        &self.name
    }

    /// Events processed throughput
    async fn throughput(&self) -> i64 {
        self.throughput
    }
}
