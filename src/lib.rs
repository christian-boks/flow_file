use bytes::Buf;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataAttributes {
    pub scale: f32,
    pub offset: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataInfo {
    pub width: u32,
    pub height: u32,
    pub data_attr: Vec<DataAttributes>,
}

pub struct DataContainer {
    pub version: u8,
    pub data_info: DataInfo,
    pub data: Vec<u8>,
}

impl DataContainer {
    pub fn new(info: DataInfo, data: Vec<u8>) -> Self {
        DataContainer {
            version: 0,
            data_info: info,
            data: data,
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let hdr_size = 17 + &self.data_info.data_attr.len() * 8;

        let mut buf = Vec::with_capacity(self.data.len() + hdr_size);

        let _ = buf.write_all("data".as_bytes());
        let _ = buf.write_all(&self.version.to_le_bytes());
        let _ = buf.write_all(&self.data_info.width.to_le_bytes());
        let _ = buf.write_all(&self.data_info.height.to_le_bytes());
        let _ = buf.write_all(&(self.data_info.data_attr.len() as u32).to_le_bytes());

        for a in &self.data_info.data_attr {
            let _ = buf.write_all(&a.scale.to_le_bytes());
            let _ = buf.write_all(&a.offset.to_le_bytes());
        }
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
        let count = u32::from_le_bytes(buffer);

        let mut attrs = vec![];
        for _ in 0..count {
            let scale = f32::from_le_bytes(buffer);
            let _ = reader.read_exact(&mut buffer);
            let offset = f32::from_le_bytes(buffer);
            let _ = reader.read_exact(&mut buffer);

            let data_info = DataAttributes { scale, offset };
            attrs.push(data_info);
        }

        let hdr_size = 17 + count * 8;
        let mut buffer: Vec<u8> = Vec::with_capacity(input.len() - hdr_size as usize);

        let _ = reader.read_to_end(&mut buffer);

        DataContainer {
            version: version,
            data_info: DataInfo {
                width: width,
                height: height,
                data_attr: attrs,
            },
            data: buffer,
        }
    }
}
