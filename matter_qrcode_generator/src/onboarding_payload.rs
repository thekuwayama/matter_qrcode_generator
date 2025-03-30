use anyhow::{anyhow, Result};
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use std::cell::LazyCell;

const DISCRIMINATOR_MAX: LazyCell<u16> = LazyCell::new(|| u16::pow(2, 12) - 1);

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
    const VERSION_BITS_LEN: usize = 3;
    const VENDOR_ID_BITS_LEN: usize = 16;
    const PRODUCT_ID_BITS_LEN: usize = 16;
    const CUSTOM_FLOW_BITS_LEN: usize = 2;
    const DISCOVERY_CAPABILITIES_BITS_LEN: usize = 8;
    const DISCRIMINATOR_BITS_LEN: usize = 12;
    const PASSCODE_BITS_LEN: usize = 27;
    const PADDING_BITS_LEN: usize = 4;

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
        let mut bv: BitVec<u8, Lsb0> = BitVec::with_capacity(
            OnboardingPayload::VERSION_BITS_LEN
                + OnboardingPayload::VENDOR_ID_BITS_LEN
                + OnboardingPayload::PRODUCT_ID_BITS_LEN
                + OnboardingPayload::CUSTOM_FLOW_BITS_LEN
                + OnboardingPayload::DISCOVERY_CAPABILITIES_BITS_LEN
                + OnboardingPayload::DISCRIMINATOR_BITS_LEN
                + OnboardingPayload::PASSCODE_BITS_LEN
                + OnboardingPayload::PADDING_BITS_LEN,
        );

        let mut version = BitVec::<_, Lsb0>::from_element(self.version);
        version.truncate(OnboardingPayload::VERSION_BITS_LEN);
        bv.append(&mut version);

        let mut vendor_id = BitVec::<_, Lsb0>::from_element(self.vendor_id);
        // vendor_id.truncate(OnboardingPayload::VENDOR_ID_BITS_LEN);
        bv.append(&mut vendor_id);

        let mut product_id = BitVec::<_, Lsb0>::from_element(self.product_id);
        // product_id.truncate(OnboardingPayload::PRODUCT_ID_BITS_LEN);
        bv.append(&mut product_id);

        bv.append(&mut self.custom_flow.bits());

        bv.append(&mut self.discovery_capabilities.bits());

        let mut discriminator = BitVec::<_, Lsb0>::from_element(self.discriminator);
        discriminator.truncate(OnboardingPayload::DISCRIMINATOR_BITS_LEN);
        bv.append(&mut discriminator);

        let mut passcode = BitVec::<_, Lsb0>::from_element(self.passcode);
        passcode.truncate(OnboardingPayload::PASSCODE_BITS_LEN);
        bv.append(&mut passcode);

        let mut padding = BitVec::<_, Lsb0>::from_element(0u8);
        padding.truncate(OnboardingPayload::PADDING_BITS_LEN);
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
    fn bits(&self) -> BitVec {
        let mut bits = BitVec::<_, Lsb0>::from_element(match &self {
            CustomFlow::StandardCommissioningFlow => 0,
            CustomFlow::UserIntentCommissioningFlow => 1,
            CustomFlow::CustomCommissioningFlow => 2,
            CustomFlow::Reserved => 4,
        });
        bits.truncate(OnboardingPayload::CUSTOM_FLOW_BITS_LEN);
        bits
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

    fn bits(&self) -> BitVec<u8> {
        let mut bv = BitVec::<_, Lsb0>::from_element(0u8);
        bv.set(0, self.soft_ap);
        bv.set(1, self.ble);
        bv.set(2, self.on_ip_network);
        bv
    }
}
