use crate::{Device, ProtocolError, ProtocolState};
use crate::event_bus::PointEvent;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct ProtocolContext {
    pub device_list: Arc<RwLock<Vec<Device>>>,
    pub sender: tokio::sync::mpsc::Sender<PointEvent>,
    pub status: Arc<RwLock<ProtocolState>>,
}



impl ProtocolContext {
    pub fn new(device_list: Arc<RwLock<Vec<Device>>>, sender: tokio::sync::mpsc::Sender<PointEvent>, status: Arc<RwLock<ProtocolState>>) -> Self {
        Self {
            device_list,
            sender,
            status,
        }
    }

    pub fn get_device(&self, device_id: i32) -> Result<Device, ProtocolError> {
        let device_list = self.device_list.read()
            .map_err(|_| ProtocolError::from("获取锁失败"))?;
        let device = device_list.iter().find(|x| x.id == device_id);
        match device {
            None => Err(ProtocolError::from("设备不存在")),
            Some(device) => Ok(device.clone()),
        }
    }

    pub fn device_list(&self) -> Result<Vec<Device>, ProtocolError> {
        let device_list = self.device_list.read()
            .map_err(|_| ProtocolError::from("获取锁失败"))?;
        Ok(device_list.clone())
    }

    // 添加设备
    pub fn add_device(&self, device: Device) -> Result<(), ProtocolError> {
        let mut device_list = self.device_list.write()
            .map_err(|_| ProtocolError::from("获取锁失败"))?;
        device_list.push(device);
        Ok(())
    }

    // 删除设备
    pub fn remove_device(&self, device_id: i32) -> Result<(), ProtocolError> {
        let mut device_list = self.device_list.write()
            .map_err(|_| ProtocolError::from("获取锁失败"))?;
        device_list.retain(|x| x.id != device_id);
        Ok(())
    }

    // 更新设备
    pub fn update_device(&self, device: Device) -> Result<(), ProtocolError> {
        let mut device_list = self.device_list.write()
            .map_err(|_| ProtocolError::from("获取锁失败"))?;
        device_list.retain(|x| x.id != device.id);
        device_list.push(device);
        Ok(())
    }
    pub fn status(&self) -> ProtocolState {
        let status = self.status.read().unwrap();
        status.clone()
    }

    pub fn set_status(&self, status: ProtocolState) {
        let mut value = self.status.write().unwrap();
        *value = status;
    }
}