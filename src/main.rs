use bundler::utils::core::bundle::Bundle;
use bundler::utils::core::envelope::Envelope;
use eyre::Result;
use reqwest::Client;
use serde_json::Value;
use hex;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {

    let private_key = String::from("YOUR_PRIVATE_KEY");
    // private key of EVM wallet funded with tWVM
    // -> wvm.dev/faucet to get testnet tokens

    let folder_path = Path::new("data");

    if !folder_path.exists() {
        panic!("Folder 'data' does not exist. Please create it and add text files.");
    }

    let mut envelopes = vec![];
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() {
            let file_content = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("Failed to read the file: {:?}", file_path));
            
            println!("Processing file: {:?}", file_path);

            let envelope_data = serde_json::to_vec(&file_content)?;
            let envelope = Envelope::new()
                .data(Some(envelope_data))
                .target(None) 
                .build()?;
            envelopes.push(envelope);
        }
    }

    // group the envelopes into a bundle
    let bundle = Bundle::new()
        .private_key(private_key)
        .envelopes(envelopes) 
        .build()?;

    // propagate the bundle
    let bundle_tx = bundle.propagate().await?;
    println!("Bundle sent successfully! Transaction Hash: {}", bundle_tx);

    // fetch our stored data from the endpoint
    let endpoint = format!("https://bundler.wvm.network/v1/envelopes/{}", bundle_tx);
    let client = Client::new();
    let response = client.get(&endpoint).send().await?;
    let body = response.text().await?;

    // parse & decode
    let json: Value = serde_json::from_str(&body)?;
    for (_i, envelope) in json["envelopes"].as_array().unwrap().iter().enumerate() {
        let input_hex = envelope["input"].as_str().expect("Failed to extract input");
        let decoded_input = hex::decode(input_hex.trim_start_matches("0x"))?;
        let _decoded_string = String::from_utf8(decoded_input.clone())
            .expect("Failed to convert decoded input to string");
    }

    // decoding logic above for demo purposes, let's not spam all that text to the terminal

    Ok(())
}

