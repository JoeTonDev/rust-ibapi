use std::time::Duration;
use std::{thread, time};

use log::{debug, info};

use ibapi::client::BasicClient;
use ibapi::contracts;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut client = BasicClient::connect("odin:4002")?;

    info!("Connected {:?}", client);

    thread::sleep(Duration::from_secs(2));

    let results = contracts::find_contract_descriptions_matching(&mut client, "microsoft")?;
    for result in &results {
        println!("contract: {:?}", result);
    }

    Ok(())
}
