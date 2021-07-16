use super::EventEncodingType;
use crate::event::{self, Value};

use async_graphql::Object;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Log {
    component_id: String,
    event: event::LogEvent,
}

impl Log {
    pub fn new(component_id: &str, event: event::LogEvent) -> Self {
        Self {
            component_id: component_id.to_string(),
            event,
        }
    }

    pub fn get_message(&self) -> Option<String> {
        Some(self.event.get("message")?.to_string_lossy())
    }

    pub fn get_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.event.get("timestamp")?.as_timestamp()
    }
}

#[Object]
/// Log event with fields for querying log data
impl Log {
    /// Name of the component associated with the log event
    async fn component_id(&self) -> &str {
        &self.component_id
    }

    /// Log message
    async fn message(&self) -> Option<String> {
        self.get_message()
    }

    /// Log timestamp
    async fn timestamp(&self) -> Option<&DateTime<Utc>> {
        self.get_timestamp()
    }

    /// Log event as an encoded string format
    async fn string(&self, encoding: EventEncodingType) -> String {
        match encoding {
            EventEncodingType::Json => serde_json::to_string(&self.event)
                .expect("JSON serialization of log event failed. Please report."),
            EventEncodingType::Yaml => serde_yaml::to_string(&self.event)
                .expect("YAML serialization of log event failed. Please report."),
        }
    }

    /// Get JSON field data on the log event, by field name
    async fn json(&self, field: String) -> Option<&Value> {
        self.event.get(field)
    }
}
