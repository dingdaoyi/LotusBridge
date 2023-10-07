use std::collections::HashMap;
use crate::config::error::Result;
use crate::models::R;
use axum::Json;
use serial2::SerialPort;

pub async fn list_serial_port() -> Result<Json<R<Vec<HashMap<String,String>>>>> {
    let available_ports = SerialPort::available_ports().unwrap();
    let res = available_ports
        .iter()
        .filter_map(|pt| pt.to_str().map(|e| e.to_string()))
        .collect::<Vec<String>>();
   let res= res.into_iter().map(|name| {
        let mut map = HashMap::new();
        map.insert("name".to_string(), name);
        map
    }).collect();
    Ok(Json(R::success_with_data(res)))
}
