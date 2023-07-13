use anyhow::{anyhow, Result};
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use once_cell::sync::Lazy;

static DISCRIMINATOR_MAX: Lazy<u16> = Lazy::new(|| u16::pow(2, 12) - 1);

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
        if discriminator > *DISCRIMINATOR_MAX {
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
        const VERSION_LEN: usize = 3;
        const VENDOR_ID_LEN: usize = 16;
        const PRODUCT_ID_LEN: usize = 16;
        const CUSTOM_FLOW_LEN: usize = 2;
        const DISCOVERY_CAPABILITIES_LEN: usize = 8;
        const DISCRIMINATOR_LEN: usize = 12;
        const PASSCODE_LEN: usize = 27;
        const PADDING_LEN: usize = 4;
        let mut bv: BitVec<u8, Lsb0> = BitVec::with_capacity(
            VERSION_LEN
                + VENDOR_ID_LEN
                + PRODUCT_ID_LEN
                + CUSTOM_FLOW_LEN
                + DISCOVERY_CAPABILITIES_LEN
                + DISCRIMINATOR_LEN
                + PASSCODE_LEN
                + PADDING_LEN,
        );

        let mut version = BitVec::<_, Lsb0>::from_element(self.version);
        version.truncate(VERSION_LEN);
        bv.append(&mut version);

        let mut vendor_id = BitVec::<_, Lsb0>::from_element(self.vendor_id);
        // bv.truncate(VENDOR_ID_LEN);
        bv.append(&mut vendor_id);

        let mut product_id = BitVec::<_, Lsb0>::from_element(self.product_id);
        // bv.truncate(PRODUCT_ID_LEN);
        bv.append(&mut product_id);

        let mut custom_flow = BitVec::<_, Lsb0>::from_element(self.custom_flow.bits());
        custom_flow.truncate(CUSTOM_FLOW_LEN);
        bv.append(&mut custom_flow);

        let mut discovery_capabilities =
            BitVec::<_, Lsb0>::from_element(self.discovery_capabilities.bits());
        discovery_capabilities.truncate(DISCOVERY_CAPABILITIES_LEN);
        bv.append(&mut discovery_capabilities);

        let mut discriminator = BitVec::<_, Lsb0>::from_element(self.discriminator);
        discriminator.truncate(DISCRIMINATOR_LEN);
        bv.append(&mut discriminator);

        let mut passcode = BitVec::<_, Lsb0>::from_element(self.passcode);
        passcode.truncate(PASSCODE_LEN);
        bv.append(&mut passcode);

        let mut padding = BitVec::<_, Lsb0>::from_element(0u8);
        padding.truncate(PADDING_LEN);
        bv.append(&mut padding);

        bv.into_vec()
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
pub enum CustomFlow {
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

pub struct DiscoveryCapabilities {
    soft_ap: bool,
    ble: bool,
    on_ip_network: bool,
}

impl DiscoveryCapabilities {
    pub fn new(soft_ap: bool, ble: bool, on_ip_network: bool) -> Self {
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
