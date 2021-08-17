fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("http://ifconfig.me/ip")?;
    if resp.status().is_success() {
        let ip = resp.text()?;
        println!("Your outbound IP: {}", ip);
    } else if resp.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something strange happened. Status: {:?}", resp.status());
    }
    Ok(())
}
