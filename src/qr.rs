use crate::base38;
use crate::onboarding_payload::{pack_onboarding_payload, CustomFlow, DiscoveryCapabilities};
use anyhow::Result;

pub(crate) fn compute_qr_version(qr_data: &str) -> i16 {
    match qr_data.len() {
        0..=38 => 2,
        39..=61 => 3,
        62..=90 => 4,
        _ => 5,
    }
}

pub(crate) fn pack_qr_data(
    vid: u16,
    pid: u16,
    custom_flow: CustomFlow,
    discovery_capabilities: DiscoveryCapabilities,
    discriminator: u16,
    passcode: u32,
) -> Result<String> {
    let payload = pack_onboarding_payload(
        vid,
        pid,
        custom_flow,
        discovery_capabilities,
        discriminator,
        passcode,
    )?;
    let base38_encoded = base38::encode(&payload);
    Ok(format!("MT:{}", base38_encoded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_qr_encode() {
        {
            let payload = pack_onboarding_payload(
                0,
                0,
                CustomFlow::StandardCommissioningFlow,
                DiscoveryCapabilities::new(false, false, false),
                0,
                1,
            );
            let base38_encoded = base38::encode(&payload.unwrap());
            let qr_data = &format!("MT:{}", base38_encoded);
            assert_eq!(qr_data, "MT:0000000000ID0000000");
        }
        {
            let payload = pack_onboarding_payload(
                65535,
                0,
                CustomFlow::StandardCommissioningFlow,
                DiscoveryCapabilities::new(false, false, false),
                0,
                1,
            );
            let base38_encoded = base38::encode(&payload.unwrap());
            let qr_data = &format!("MT:{}", base38_encoded);
            assert_eq!(qr_data, "MT:W2L9000000ID0000000");
        }
        {
            let payload = pack_onboarding_payload(
                65521,
                32768,
                CustomFlow::StandardCommissioningFlow,
                DiscoveryCapabilities::new(false, true, false),
                3840,
                20202021,
            );
            let base38_encoded = base38::encode(&payload.unwrap());
            let qr_data = &format!("MT:{}", base38_encoded);
            assert_eq!(qr_data, "MT:Y.K9042C00KA0648G00");
        }
    }
}
