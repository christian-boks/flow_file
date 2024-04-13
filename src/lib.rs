use bytes::Buf;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FlowInfo {
    pub width: u32,
    pub height: u32,
    pub x_scale: f32,
    pub x_offset: f32,
    pub y_scale: f32,
    pub y_offset: f32,
}

pub struct FlowContainer {
    pub version: u8,
    pub flow_info: FlowInfo,
    pub data: Vec<u8>,
}

impl FlowContainer {
    pub fn new(info: FlowInfo, data: Vec<u8>) -> Self {
        FlowContainer {
            version: 0,
            flow_info: info,
            data: data,
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.data.len() + 34);

        let _ = buf.write_all("flow".as_bytes());
        let _ = buf.write_all(&self.version.to_le_bytes());
        let _ = buf.write_all(&self.flow_info.width.to_le_bytes());
        let _ = buf.write_all(&self.flow_info.height.to_le_bytes());
        let _ = buf.write_all(&self.flow_info.x_scale.to_le_bytes());
        let _ = buf.write_all(&self.flow_info.x_offset.to_le_bytes());
        let _ = buf.write_all(&self.flow_info.y_scale.to_le_bytes());
        let _ = buf.write_all(&self.flow_info.y_offset.to_le_bytes());
        let _ = buf.write_all(&self.data);

        buf
    }

    pub fn read(input: &Vec<u8>) -> Self {
        let mut reader = input.reader();

        let mut header = [0; 4];
        let _ = reader.read_exact(&mut header);

        let mut buffer = [0; 1];
        let _ = reader.read_exact(&mut buffer);
        let version = u8::from_le_bytes(buffer);

        let mut buffer = [0; 4];
        let _ = reader.read_exact(&mut buffer);
        let width = u32::from_le_bytes(buffer);
        let _ = reader.read_exact(&mut buffer);
        let height = u32::from_le_bytes(buffer);
        let _ = reader.read_exact(&mut buffer);
        let x_scale = f32::from_le_bytes(buffer);
        let _ = reader.read_exact(&mut buffer);
        let x_offset = f32::from_le_bytes(buffer);
        let _ = reader.read_exact(&mut buffer);
        let y_scale = f32::from_le_bytes(buffer);
        let _ = reader.read_exact(&mut buffer);
        let y_offset = f32::from_le_bytes(buffer);

        //let mut buffer: Vec<u8> = Vec::with_capacity(width as usize * 2 * height as usize);
        let mut buffer: Vec<u8> = Vec::with_capacity(input.len() - 29);

        let _ = reader.read_to_end(&mut buffer);

        FlowContainer {
            version: version,
            flow_info: FlowInfo {
                width: width,
                height: height,
                x_scale,
                x_offset,
                y_scale,
                y_offset,
            },
            data: buffer,
        }
    }
}
