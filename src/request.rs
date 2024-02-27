use reqwest;

pub async fn fetch_url(fqdn: &str, ip_address: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let response = client.get(ip_address)
        .header("Host", fqdn)
        .send()
        .await?;

    Ok(response)
}