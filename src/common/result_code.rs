#[derive(Debug)]
pub enum ResultCode {
    SUCCESS,

    FAILED,

    PARAM_NOT_VALID(u32, String),

    SYSTEM_EXECUTION_ERROR(u32, String),

    NO_DATA_FOUND(u32, String),
}

impl ResultCode {

    pub fn get_code(data: ResultCode) -> u32 {
        match data {
            ResultCode::SUCCESS                                  => 200,
            ResultCode::FAILED                                   => 500,
            ResultCode::PARAM_NOT_VALID(code, _)            => code,
            ResultCode::SYSTEM_EXECUTION_ERROR(code, _)     => code,
            ResultCode::NO_DATA_FOUND(code, _)              => code,
        }
    }

    pub fn get_message(data: ResultCode) -> String {
        match data {
            ResultCode::SUCCESS                                        => String::from("成功"),
            ResultCode::FAILED                                         => String::from("失败"),
            ResultCode::PARAM_NOT_VALID(_, message)            => message,
            ResultCode::SYSTEM_EXECUTION_ERROR(_, message)     => message,
            ResultCode::NO_DATA_FOUND(_, message)              => message,
        }
    }
    
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_success() {
        let success = ResultCode::SUCCESS;

        println!("{:?}", success);
    }
}