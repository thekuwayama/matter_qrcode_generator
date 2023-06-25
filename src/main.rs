use std::env;

use env_logger;
use matter::core::CommissioningData;
use matter::data_model::cluster_basic_information::BasicInfoConfig;
use matter::pairing::{print_pairing_code_and_qr, DiscoveryCapabilities};
use matter::secure_channel::spake2p::VerifierData;

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let dev_info = BasicInfoConfig {
        vid: 0xFFF1,
        pid: 0x8002,
        hw_ver: 2,
        sw_ver: 1,
        sw_ver_str: "1".to_string(),
        serial_no: "aabbccdd".to_string(),
        device_name: "Smart Speaker".to_string(),
    };
    let comm_data = CommissioningData {
        // TODO: Hard-coded for now
        verifier: VerifierData::new_with_pw(123456),
        discriminator: 250,
    };
    let discovery_capabilities = DiscoveryCapabilities::new(true, true, true);
    print_pairing_code_and_qr(&dev_info, &comm_data, discovery_capabilities);
}
