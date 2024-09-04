// https://crates.io/crates/nostr-sdk

use nostr_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> { 
    // Create keys
    // let my_keys: Keys = Keys::generate();
    // let hex_pubkey: String = my_keys.public_key().to_hex();
    // let nsec = my_keys.secret_key();
    // match nsec{
    //     Ok(nsec) => println!("Secret key: {:?}", nsec.to_bech32()),
    //     Err(error) => eprintln!("Error: {:?}", error),
    // }
    // println!("Hex Pub: {}" , hex_pubkey);

    // -----------

    // or use these already generated ones for testing purposes
    // nsec: nsec1ddvx2kul4frw0l4nsl0np3r7nc6m2kpgrqzxz7lr84u4mucgnntqds9xqx

    let nsec_to_use = "nsec1ddvx2kul4frw0l4nsl0np3r7nc6m2kpgrqzxz7lr84u4mucgnntqds9xqx";
    let my_keys = match Keys::parse(nsec_to_use) {
        
        Ok(keys) => keys,
        Err(error) => {
            eprintln!("Error parsing keys: {:?}", error);
            return Err(error.into());
        }
    };

    let bech32_pubkey = match my_keys.public_key().to_bech32() {
        Ok(pubkey) => pubkey,
        Err(error) => {
            eprintln!("Error converting to Bech32: {:?}", error);
            return Err(error.into());
        }
    };

    println!("Bech32 PubKey: {}", bech32_pubkey);

    // Directly create a client without matching against Result
    let client = Client::new(&my_keys);

    client.add_relay("wss://relay.damus.io").await?;
    client.connect().await;

    // Publish a text note
    let note_result =client.publish_text_note("Rustification process continues; first bot -> checked!", []).await?;
    println!("Note ID is: {}", note_result.to_string());


    Ok(())
}
