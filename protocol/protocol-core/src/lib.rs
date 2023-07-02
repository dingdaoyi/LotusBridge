use std::any::Any;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents the result of a decoder.
#[derive(Debug, Serialize, Deserialize)]
pub struct DecoderResult {
    /// List of decoder data items.
    pub decoder_data_item_list: Vec<DecoderDataItem>,
    /// ACK Message.
    pub ack_message: Option<String>,
    /// Message ID.
    pub message_id: Option<i32>,
    /// Driver service name.
    pub driver_service_name: Option<String>,
    /// Driver name.
    pub driver_name: Option<String>,
    /// IMSI.
    pub imsi: Option<String>,
    /// ACK topic.
    pub ack_topic: Option<String>,
    /// ICCID.
    pub iccid: Option<String>,
    /// Third-party device ID.
    pub trd_device_id: Option<String>,
    /// Source address.
    pub source_address: Option<String>,
}

/// Represents a decoder data item.
#[derive(Debug, Serialize, Deserialize)]
pub struct DecoderDataItem {
    /// Identifier.
    pub identifier: String,
    /// Unit address.
    pub unit_address: String,
    /// Unit type.
    pub unit_type: Option<i32>,
    /// Unit type name.
    pub unit_type_name: Option<String>,
    /// System type.
    pub system_type: Option<i32>,
    /// Unit description.
    pub unit_description: Option<String>,
    /// Value.
    pub value: Option<Value>,
    /// System address.
    pub system_address: Option<i32>,
    /// Child device code.
    pub child_device_code: Option<String>,
    /// Acquisition time.
    pub acquisition_time: Option<NaiveDateTime>,
}

/// Represents a value that can be stored in `DecoderDataItem`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    /// Integer value.
    Integer(i32),
    /// Float value.
    Float(f64),
    /// String value.
    String(String),
    /// Boolean value.
    Boolean(bool),
}

/// Protocol trait for data processing.
pub trait Protocol:Any + Send + Sync {
    /// Process the data and return the decoder result.
    ///
    /// # Arguments
    ///
    /// * `data` - The input data as a byte slice.
    ///
    /// # Returns
    ///
    /// The decoder result wrapped in a `Result` indicating success or failure.
    fn process_data(&self, data: &[u8]) -> Result<DecoderResult, String>;
}
