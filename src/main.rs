use std::env;
use std::io::Write;
use std::process;

use env_logger;
use matter::core::CommissioningData;
use matter::data_model::cluster_basic_information::BasicInfoConfig;
use matter::pairing::{print_pairing_code_and_qr, DiscoveryCapabilities};
use matter::secure_channel::spake2p::VerifierData;

mod cli;

fn main() {
    let matches = cli::build().get_matches();
    let vid = matches
        .get_one::<String>(cli::VENDOR_ID)
        .unwrap()
        .parse::<u16>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::VENDOR_ID);
            process::exit(1);
        });
    let pid = matches
        .get_one::<String>(cli::PRODUCT_ID)
        .unwrap()
        .parse::<u16>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::PRODUCT_ID);
            process::exit(1);
        });
    let hw_ver = matches
        .get_one::<String>(cli::HARDWARE_VERSION)
        .unwrap()
        .parse::<u16>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::HARDWARE_VERSION);
            process::exit(1);
        });
    let sw_ver = matches
        .get_one::<String>(cli::SOFTWARE_VERSION)
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::SOFTWARE_VERSION);
            process::exit(1);
        });
    let serial_no = matches
        .get_one::<String>(cli::SERIAL_NUMBER)
        .unwrap_or_else(|| {
            eprintln!("failed, <{}> should be hex string", cli::SERIAL_NUMBER);
            process::exit(1);
        });
    let device_name = matches
        .get_one::<String>(cli::DEVICE_NAME)
        .unwrap_or_else(|| {
            eprintln!("failed, <{}> should be string", cli::DEVICE_NAME);
            process::exit(1);
        });
    let verifier = matches
        .get_one::<String>(cli::VERIFIER)
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::VERIFIER);
            process::exit(1);
        });
    let discriminator = matches
        .get_one::<String>(cli::DISCRIMINATOR)
        .unwrap()
        .parse::<u16>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::DISCRIMINATOR);
            process::exit(1);
        });

    do_print_pairing_code_and_qr(
        vid,
        pid,
        hw_ver,
        sw_ver,
        serial_no.to_string(),
        device_name.to_string(),
        verifier,
        discriminator,
    );
}

fn do_print_pairing_code_and_qr(
    vid: u16,
    pid: u16,
    hw_ver: u16,
    sw_ver: u32,
    serial_no: String,
    device_name: String,
    verifier: u32,
    discriminator: u16,
) {
    env::set_var("RUST_LOG", "info");
    env_logger::Builder::from_default_env()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .init();

    let dev_info = BasicInfoConfig {
        vid,
        pid,
        hw_ver,
        sw_ver,
        sw_ver_str: sw_ver.to_string(),
        serial_no,
        device_name,
    };
    let comm_data = CommissioningData {
        verifier: VerifierData::new_with_pw(verifier),
        discriminator,
    };
    let discovery_capabilities = DiscoveryCapabilities::new(true, true, true);
    print_pairing_code_and_qr(&dev_info, &comm_data, discovery_capabilities);
}
