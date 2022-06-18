//!
//! 核酸服务实现
//! 
use crate::common::{ipfs, nucleic_enum::NucleicAcidsEnum};
use crate::domain::dto::nucleic_acids;
use ipfs_api_backend_actix::IpfsApi;
use std::io::Cursor;
use chrono::prelude::*;
use futures::TryStreamExt;


type NucleicAcidsDto = nucleic_acids::NucleicAcidsDto;

pub async fn save_nucleic_ipfs(data: NucleicAcidsDto) -> String {

    let symbol = String::from("-");
    let mut req_data = String::new();

    if let Some(name) = data.name {
        req_data.push_str(&name)
    }
    req_data.push_str(&symbol);

    // data transform
    let date: String = Local::now().format("%Y-%m-%d %H:%M").to_string();
    req_data.push_str(&date);

    req_data.push_str(&symbol);
    if let NucleicAcidsEnum::NEGATIVE = data.result {
        req_data.push_str("阴性");
    }
    if let NucleicAcidsEnum::POSITIVE = data.result {
        req_data.push_str("阳性");
    }

    let client = ipfs::coon();
    let ipfs_data = Cursor::new(req_data);

    match client.add(ipfs_data).await {
        Ok(res) => res.hash,
        Err(e) => String::from("Save nucleic acid error"),
    }
}

pub async fn get_nucleic(hash: String) -> String {
    let client = ipfs::coon();
    
    let res = client
        .block_get(&hash)
        .map_ok(|chunk| chunk.to_vec())
        .try_concat()
        .await;

        match res {
            Ok(res) => {
                let resp_data = String::from_utf8_lossy(&res);
                resp_data.to_string()
            }
            Err(e) => String::from("No nucleic acid results found～"),
        }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[actix::test]
    async fn test_ipfs_set_data() {
        let client = ipfs::coon();
        
        let data = Cursor::new("zhaoweiaa");
        match client.add(data).await {
            Ok(res) => println!("{}", res.hash),
            Err(e) => eprintln!("error adding file: {}", e)
        }
 
    }

    #[actix::test]
    async fn test_cat_string() {
        let client = ipfs::coon();

        let hash = "QmPpJ7aV4Yp57EqxM28uStst9RGhmFjpwKT48aDiY2JH7P";

        let res = client
            .block_get(hash)
            .map_ok(|chunk| chunk.to_vec())
            .try_concat()
            .await;

        match res {
            Ok(res) => {
                let result = String::from_utf8_lossy(&res);
                println!("{}", result);
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    #[test]
    fn test_now_date() {
        let local: DateTime<Local> = Local::now();

        let date2 = local.format("%Y-%m-%d %H:%M").to_string();

        println!("{}", date2);
    }
}