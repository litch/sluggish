#[macro_use]
extern crate serde_json;
use serde::Deserialize;
use cln_plugin::{Builder, Error, Plugin};
use std::time::Duration;

use tokio;
use tokio::{time};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    if let Some(plugin) = Builder::new((), tokio::io::stdin(), tokio::io::stdout())
        
        .hook("invoice_payment", invoice_handler)
        .start()
        .await?
    {
        plugin.join().await
    } else {
        Ok(())
    }
}

#[derive(Deserialize, Debug)]
struct Invoice {
    payment: Payment,
    
}
#[derive(Deserialize, Debug)]
struct Payment {
    label: String,

}

async fn invoice_handler(
    _p: Plugin<()>,
    v: serde_json::Value,
) -> Result<serde_json::Value, Error> {
    let i: Invoice = serde_json::from_value(v).unwrap();
    log::info!("Got an invoice paid hook call with invoice {:?}", i);
    let label = i.payment.label.as_str();
    
    if label.contains("sluggish") || label.contains("keysend") {
        log::info!("Sleeping since this matched a sluggish or keysend");
        time::sleep(Duration::from_secs(
            30,
        )).await;
    }
    

    Ok(json!({"result": "continue"}))
}