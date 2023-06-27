use anyhow::{anyhow, Result};

struct OnboardingPayload {
    version: u8,
    vendor_id: u16,
    product_id: u16,
    custom_flow: CustomFlow,
    discovery_capabilities: DiscoveryCapabilities,
    discriminator: u16,
    passcode: u32,
    // TLVData is not supported.
}

impl OnboardingPayload {
    pub fn new(
        vid: u16,
        pid: u16,
        custom_flow: CustomFlow,
        discovery_capabilities: DiscoveryCapabilities,
        discriminator: u16,
        passcode: u32,
    ) -> Result<Self> {
        if discriminator > u16::pow(2, 12) - 1 {
            return Err(anyhow!(
                "A Discriminator SHALL be included as a 12-bit unsigned integer."
            ));
        }

        let payload = OnboardingPayload {
            version: 0,
            vendor_id: vid,
            product_id: pid,
            custom_flow,
            discovery_capabilities,
            discriminator,
            passcode,
        };

        if !payload.validate_passcode() {
            return Err(anyhow!("invalid Passcode."));
        }

        Ok(payload)
    }

    fn serialize(&self) -> Vec<u8> {
        let mut res = vec![0; 11];

        let d = self.discovery_capabilities.bits();
        res[0] = self.version.reverse_bits() as u8 | (self.vendor_id.reverse_bits() >> 11) as u8;
        res[1] = ((self.vendor_id.reverse_bits() << 5) >> 8) as u8;
        res[2] = ((self.vendor_id.reverse_bits() << 13) >> 8) as u8
            | (self.product_id.reverse_bits() >> 11) as u8;
        res[3] = ((self.product_id.reverse_bits() << 5) >> 8) as u8;
        res[4] = ((self.product_id.reverse_bits() << 13) >> 8) as u8
            | (self.custom_flow.bits().reverse_bits() >> 3)
            | (d.reverse_bits() >> 5);
        res[5] = (d.reverse_bits() << 3) | (self.discriminator.reverse_bits() >> 13) as u8;
        res[6] = ((self.discriminator.reverse_bits() << 3) >> 8) as u8;
        res[7] = ((self.discriminator.reverse_bits() << 11) >> 8) as u8
            | (self.passcode.reverse_bits() >> 25) as u8;
        res[8] = ((self.passcode.reverse_bits() << 7) >> 24) as u8;
        res[9] = ((self.passcode.reverse_bits() << 15) >> 24) as u8;
        res[10] = ((self.passcode.reverse_bits() << 23) >> 24) as u8;
        res.into_iter().map(|byte| byte.reverse_bits()).collect()
    }

    fn validate_passcode(&self) -> bool {
        const PASSCODE_VALID_MIN: u32 = 1;
        const PASSCODE_VALID_MAX: u32 = 99999998;

        !(self.passcode < PASSCODE_VALID_MIN
            || self.passcode > PASSCODE_VALID_MAX
            || self.passcode == 11111111
            || self.passcode == 22222222
            || self.passcode == 33333333
            || self.passcode == 44444444
            || self.passcode == 55555555
            || self.passcode == 66666666
            || self.passcode == 77777777
            || self.passcode == 88888888
            || self.passcode == 12345678
            || self.passcode == 87654321)
    }
}

pub(crate) fn pack_onboarding_payload(
    vid: u16,
    pid: u16,
    custom_flow: CustomFlow,
    discovery_capabilities: DiscoveryCapabilities,
    discriminator: u16,
    passcode: u32,
) -> Result<Vec<u8>> {
    let payload = OnboardingPayload::new(
        vid,
        pid,
        custom_flow,
        discovery_capabilities,
        discriminator,
        passcode,
    );

    Ok(payload?.serialize())
}

#[allow(dead_code)]
pub(crate) enum CustomFlow {
    StandardCommissioningFlow,
    UserIntentCommissioningFlow,
    CustomCommissioningFlow,
    Reserved,
}

impl CustomFlow {
    fn bits(&self) -> u8 {
        match &self {
            CustomFlow::StandardCommissioningFlow => 0,
            CustomFlow::UserIntentCommissioningFlow => 1,
            CustomFlow::CustomCommissioningFlow => 2,
            CustomFlow::Reserved => 4,
        }
    }
}

pub(crate) struct DiscoveryCapabilities {
    soft_ap: bool,
    ble: bool,
    on_ip_network: bool,
}

impl DiscoveryCapabilities {
    pub(crate) fn new(soft_ap: bool, ble: bool, on_ip_network: bool) -> Self {
        DiscoveryCapabilities {
            soft_ap,
            ble,
            on_ip_network,
        }
    }

    fn bits(&self) -> u8 {
        let mut res = 0u8;
        if self.soft_ap {
            res |= 1;
        }

        if self.ble {
            res |= 1 << 1;
        }

        if self.on_ip_network {
            res |= 1 << 2;
        }

        res
    }
}
