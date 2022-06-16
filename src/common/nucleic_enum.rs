use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum NucleicAcidsEnum { //
    // 阴性
    NEGATIVE,
    // 阳性
    POSITIVE,
}

impl NucleicAcidsEnum {

    fn get_value(&self) -> String {
        match *self {
            NucleicAcidsEnum::NEGATIVE => String::from("阴性"),
            NucleicAcidsEnum::POSITIVE => String::from("阳性"),
        }
    }
}