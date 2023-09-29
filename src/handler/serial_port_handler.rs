use crate::config::error::Result;
use crate::models::R;
use axum::Json;
use serial2::SerialPort;

pub async fn list_serial_port() -> Result<Json<R<Vec<String>>>> {
    let res = SerialPort::available_ports().unwrap();
    let res = res
        .iter()
        .filter_map(|pt| pt.to_str().map(|e| e.to_string()))
        .collect::<Vec<String>>();
    Ok(Json(R::success_with_data(res)))
}
