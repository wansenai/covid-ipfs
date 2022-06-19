//!
//! 核酸数据传输对象
//! 
use crate::common::nucleic_enum;
use serde::{Deserialize, Serialize};

type NucleicEnum = nucleic_enum::NucleicAcidsEnum;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NucleicAcidsDto {
    // 姓名
    pub name: Option<String>,
    // 性别 0-女 1-男
    pub sex: Option<u8>,
    // 电话
    pub phone: Option<u64>,
    // 住址
    pub address: Option<String>,
    // 时间
    pub date: Option<String>,
    // 结果
    pub result: NucleicEnum,
}

impl NucleicAcidsDto {

    pub fn new(&self) -> Self {
        NucleicAcidsDto {
            name: self.name.clone(),
            sex: self.sex,
            phone: self.phone,
            address: self.address.clone(),
            date: self.date.clone(),
            result: self.result.clone(),
        }
    }
}