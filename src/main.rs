use std::error::Error;

use base32::Alphabet;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use clap::{CommandFactory, Parser};
use comfy_table::Table;
use prost::Message;
use rqrr;

include!(concat!(env!("OUT_DIR"), "/googleauth.rs"));

/// A command line tool to decode Google Authenticator export data
#[derive(Parser)]
struct Args {
    /// Path to image containing QR-code
    #[arg(short, long, value_name = "img file")]
    path: Option<String>,

    /// Migration link decoded from QR-code, starts with: otpauth-migration://offline?data=
    #[arg(short, long, value_name = "link")]
    link: Option<String>,
}

fn deserialise_migration_payload(data: &[u8]) -> Result<MigrationPayload, prost::DecodeError> {
    MigrationPayload::decode(data)
}

fn algo_string(algorithm: i32) -> String {
    match algorithm {
        1 => String::from("Sha1"),
        2 => String::from("Sha256"),
        3 => String::from("Sha512"),
        4 => String::from("Md5"),
        _ => String::from("Unspecified"),
    }
}

fn algo_type_string(otp_type: i32) -> String {
    match otp_type {
        1 => String::from("HOTP"),
        2 => String::from("TOTP"),
        _ => String::from("Unspecified"),
    }
}

fn digits_count_string(digit_count: i32) -> String {
    match digit_count {
        1 => String::from("6"),
        2 => String::from("8"),
        _ => String::from("Unspecified"),
    }
}

fn get_qrcode_data(path: &str) -> Result<String, Box<dyn Error>> {
    let img = match image::open(path) {
        Ok(img) => img.to_luma8(),
        Err(e) => return Err(e.into()),
    };

    //let img = image::open(path)?.to_luma8();

    let mut img = rqrr::PreparedImage::prepare(img);
    let grids = img.detect_grids();
    if grids.len() < 1 {
        return Err("No QR codes detected in image".into());
    }
    let (_, content) = grids[0].decode()?; //.unwrap();
    Ok(content)
}

fn decode_backup(link: &str) {
    let split = link
        .strip_prefix("otpauth-migration://offline?data=")
        .expect("Data is not valid migration data");

    let uri_decoded = urlencoding::decode(&split).expect("Could not url decode data");
    let base64_decoded = STANDARD
        .decode(uri_decoded.as_bytes())
        .expect("Could not base64 decode data");
    let payload =
        deserialise_migration_payload(&base64_decoded).expect("Could not deserialise data");

    let mut table = Table::new();
    table.set_header(vec![
        "Name",
        "Issuer",
        "Secret",
        "Algorithm",
        "DigitCount",
        "OtpType",
    ]);

    for otp in payload.otp_parameters {
        table.add_row(vec![
            otp.name,
            otp.issuer,
            base32::encode(Alphabet::Rfc4648 { padding: (false) }, &otp.secret),
            algo_string(otp.algorithm),
            digits_count_string(otp.digits),
            algo_type_string(otp.r#type),
        ]);
    }
    println!("{table}");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.link {
        Some(link) => decode_backup(&link),
        None => match args.path {
            Some(path) => match get_qrcode_data(&path) {
                Ok(link) => decode_backup(&link),
                Err(e) => return Err(e.into()),
            },
            None => {
                let mut cmd = Args::command();
                let _ = cmd.print_long_help()?;
            }
        },
    }
    Ok(())
}
