use ipfs_api_backend_actix::{IpfsClient, TryFromUri};

pub fn coon() -> IpfsClient{

    let client = IpfsClient::from_str("http://101.43.52.162:5001").unwrap();

    client
}