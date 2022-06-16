use summer_ipfs_client::IpfsApi;

fn coon() -> IpfsApi{
    let api = IpfsApi::new("101.43.52.162", 5001);
    api
}