use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use super::result_code::ResultCode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IResult<T> {
    pub code: u32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> IResult<T>
{   
    pub fn new(code: u32, msg: String, data: Option<T>) -> Self {
        Self {
            code,
            msg,
            data,
        }
    }

    pub fn success() -> Self {
        Self {
            code: ResultCode::get_code(ResultCode::SUCCESS),
            msg: ResultCode::get_message(ResultCode::SUCCESS),
            data: None,
        }
    }

    pub fn success_data(data: T) -> Self {
        Self {
            code: ResultCode::get_code(ResultCode::SUCCESS),
            msg: ResultCode::get_message(ResultCode::SUCCESS),
            data: Some(data),
        }
    }

    pub fn error() -> Self {
        Self {
            code: ResultCode::get_code(ResultCode::FAILED),
            msg: ResultCode::get_message(ResultCode::FAILED),
            data: None,
        }
    }

    pub fn other_error(code: u32, msg: &str) -> Self {
        Self {
            code: code,
            msg: msg.to_string(),
            data: None,
        }
    }
}

impl<T> ToString for IResult<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_success() {
        let success: IResult<String> = IResult::success();

        println!("{:?}", success);
    }
}