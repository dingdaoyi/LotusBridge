// SPDX-FileCopyrightText: Copyright (c) 2017-2023 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Asynchronous RTU client example

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_serial::SerialStream;

    use tokio_modbus::prelude::*;

    let tty_path = "/dev/tty.usbserial-1120";
    let slave = Slave(0x03);
    let builder = tokio_serial::new(tty_path, 19200);
    match SerialStream::open(&builder) {
        Ok(port) => {
            let mut ctx = rtu::attach_slave(port, slave);
            println!("Reading a sensor value");
            let rsp = ctx.read_holding_registers(0x0048, 1).await?;
            println!("Sensor value is: {rsp:?}");
        }
        Err(msg) => {
            println!("Failed to open \"{}\". Error: {}", tty_path, msg);
        }
    }

    Ok(())
}
