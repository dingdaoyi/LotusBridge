use std::sync::Arc;
use std::time::Duration;
use async_trait::async_trait;
use rppal::gpio::{Error, Level};
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use protocol_core::{Device, Point, Protocol, ProtocolError, ProtocolState, ReadPointRequest, Value, WriterPointRequest};
use protocol_core::event_bus::PointEvent;
use protocol_core::protocol_context::ProtocolContext;

pub struct RaspberryGpioProtocol {
    // device_list: Arc<Vec<Device>>,
    // sender: Option<Arc<Mutex<Sender<PointEvent>>>>,
    //
    // status: ProtocolState,
    context: Option<ProtocolContext>,

    task: Option<tokio::task::JoinHandle<()>>,

}

impl RaspberryGpioProtocol {
    pub fn new() -> Self {
        Self {
            context: None,
            task: None,
        }
    }
    pub async fn innit_task(& mut self) -> Result<(), ProtocolError> {
        let context=self.context.clone().unwrap();
        let device_list=context.device_list()?;

        if device_list.len() == 1 {
            let gpio = rppal::gpio::Gpio::new().map_err(|err|GpioError::from(err))?;
            let device = device_list.get(0).unwrap();
            let points: &Vec<Point> = &device.points;
            let input_list: Vec<Point> = points.iter()
                .filter(|point| point.access_mode == protocol_core::AccessMode::ReadWrite)
                .cloned()
                .collect();

            let task = tokio::spawn(async move {
                loop {
                    for point in input_list.iter() {
                        let pin = point.address.parse().unwrap_or(0);
                        //TODO 暂定全部上拉输入,看怎么设置输入模式好点
                       let gpio_result= gpio.get(pin);
                        if gpio_result.is_err(){
                            println!("gpio_result:{:?}", gpio_result);
                            continue;
                        }
                        let mut pin = gpio_result.unwrap().into_input_pullup();
                        let pin_result = pin.poll_interrupt(true, Some(Duration::from_secs(2)));
                        match pin_result {
                            Ok(Some(pin)) => {
                                println!("pin:{:?}", pin);
                                let value = match pin {
                                    Level::Low => {
                                        Value::Boolean(false)
                                    }
                                    Level::High => {
                                        Value::Boolean(true)
                                    }
                                };
                                let _ = context.sender.send(PointEvent {
                                    point_id: point.id,
                                    value,
                                }).await;
                            }
                            Err(error) => {
                                println!("error:{:?}", error);
                            }
                            _ => {}
                        }
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    }
                }
            });
            self.task = Some(task);
        }

        Ok(())
    }
}

#[async_trait]
impl Protocol for RaspberryGpioProtocol {
    fn context(&self) -> Option<ProtocolContext> {
        self.context.clone()
    }

    async fn initialize(&mut self,  context: ProtocolContext) -> Result<(), ProtocolError> {
        self.context = Some(context.clone());
        self.innit_task().await?;
        self.context.clone().unwrap().set_status(ProtocolState::Running);
        Ok(())
    }


    async fn read_point(&self, request: ReadPointRequest) -> Result<Value, ProtocolError> {
        let pin = request.address.parse().unwrap_or(0);
        let  pin = rppal::gpio::Gpio::new()
            .map_err(|error| GpioError::from(error))?
            .get(pin).map_err(|error| GpioError::from(error))?.read();
        match pin {
            rppal::gpio::Level::Low => {
                Ok(Value::Boolean(false))
            }
            rppal::gpio::Level::High => {
                Ok(Value::Boolean(true))
            }
        }
    }

    async fn write_point(&self, request: WriterPointRequest) -> Result<Value, ProtocolError> {
        let pin = request.address.parse().unwrap_or(0);
        let gpio = rppal::gpio::Gpio::new().map_err(|error| GpioError::from(error))?;
        let mut pin = gpio.get(pin).map_err(|error| GpioError::from(error))?.into_output();
        match request.value {
            Value::Boolean(value) => {
                if value {
                    pin.set_high();
                } else {
                    pin.set_low();
                }
                Ok(request.value)
            }
            _ => {
                Err(ProtocolError::new("不支持的数据类型"))
            }
        }
    }

    fn stop(&mut self, _force: bool) -> Result<(), ProtocolError> {
        self.context.clone().unwrap().set_status(ProtocolState::Closed);
        Ok(())
    }
}

pub struct GpioError(ProtocolError);

impl From<GpioError> for ProtocolError {
    fn from(error: GpioError) -> Self {
        error.0
    }
}

impl From<rppal::gpio::Error> for GpioError {
    fn from(error: rppal::gpio::Error) -> Self {
        match error {
            Error::UnknownModel => {
                Self(ProtocolError::new("未知型号"))
            }
            Error::PinUsed(pin) => {
                Self(ProtocolError::new(format!("引脚被占用:{}", pin)))
            }
            Error::PinNotAvailable(pin) => {
                Self(ProtocolError::new(format!("引脚不可用:{}", pin)))
            }
            Error::PermissionDenied(msg) => {
                Self(ProtocolError::new(format!("权限不足:{}", msg)))
            }
            Error::Io(error) => {
                Self(ProtocolError::new(format!("IO错误:{}", error)))
            }
            Error::ThreadPanic => {
                Self(ProtocolError::new("线程崩溃"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use protocol_core::{AccessMode, DataType};
    use super::*;

    #[test]
    #[cfg(target_os = "linux")]
    async fn unknown_model() {
        let protocol = RaspberryGpioProtocol::new();
        let request = ReadPointRequest {
            address: "1".to_string(),
            data_type: DataType::Integer,
            access_mode: AccessMode::ReadWrite,
            multiplier: 0.0,
            device_id: 1,
            point_id: 1,
            precision: 0,
        };
        let result = protocol.read_point(request).await;
        assert_eq!(result.is_err(), true);
        match result {
            Ok(val) => {
                println!("val:{:?}", val);
            }
            Err(val) => {
                println!("val:{:?}", val);
            }
        }
    }

    #[test]
    fn test_start() {
        let protocol = RaspberryGpioProtocol::new();
        println!("启动成功");
    }
}
