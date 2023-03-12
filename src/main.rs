use clap::Parser;
use ics2000_rs::{get_devices, get_devices_encrypted, login, Crypto, Ics};
use serde_json::{Result, Value};
use std::{fs, thread};

#[derive(Parser, Debug)]
struct Args {
    /// Email of klikaan account
    #[arg(short, long)]
    email: String,

    /// Password of klikaan account
    #[arg(short, long)]
    password: String,
}

fn main() {
    let args = Args::parse();

    let mut ics = Ics::new(&args.email, &args.password, true);

    ics.login();
    // let devices = ics.get_devices();
    // println!("{:?}", devices);
    //
    // ics.turn_on(33500573);
    ics.get_entity_status(29023811);

    // let login_response = login(&args.email, &args.password);
    // let crypto = Crypto::new(login_response.homes[0].aes_key.to_string());
    // let devices_encrypted = get_devices_encrypted(
    //     &args.email,
    //     &args.password,
    //     &login_response.homes[0].mac,
    //     &login_response.homes[0].home_id,
    // );
    //
    // let unencrypted_data: String = devices_encrypted
    //     .iter()
    //     .map(|device| {
    //         let data = device.data.as_ref().unwrap();
    //         let decrypted = crypto.decrypt_base64(&data);
    //         String::from_utf8_lossy(&decrypted).to_string()
    //     })
    //     .collect::<Vec<String>>()
    //     .join("\n");

    // println!("{:?}", unencrypted_data);
    // fs::write("./devices.txt", unencrypted_data).expect("There was an error saving entities");
    // let data: &str = devices_encrypted
    //     .as_ref()
    //     .unwrap()
    //     .get(0)
    //     .unwrap()
    //     .data
    //     .as_ref()
    //     .unwrap();
    //
    // let filtered_devices: Vec<String> = devices_unencrypted
    //     .iter()
    //     .filter(|device| device.module.as_ref().unwrap().device.unwrap() == 1)
    //     .map(|device| {
    //         device
    //             .module
    //             .as_ref()
    //             .unwrap()
    //             .name
    //             .as_ref()
    //             .unwrap()
    //             .to_string()
    //     })
    //     .collect();
    //
    // println!("{:?}", filtered_devices);
    // // println!("{:?}", String::from_utf8(crypto.decrypt_base64(data)));
}
