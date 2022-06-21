use ipfs_api_backend_actix::{IpfsClient, TryFromUri};

pub fn coon() -> IpfsClient{

    let client = IpfsClient::from_str("http://127.0.0.1:5001").unwrap();

    client
}
