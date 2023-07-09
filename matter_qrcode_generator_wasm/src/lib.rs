use cfg_if::cfg_if;
use matter_qrcode_generator::onboarding_payload::{CustomFlow, DiscoveryCapabilities};
use matter_qrcode_generator::qr::{compute_qr_version, pack_qr_data};
use qrcode::render::svg;
use qrcode::{QrCode, Version};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn do_print_qr(
    vid: u16,
    pid: u16,
    passcode: u32,
    discriminator: u16,
) -> Result<String, JsValue> {
    let qr_data = match pack_qr_data(
        vid,
        pid,
        CustomFlow::StandardCommissioningFlow, //  Standard commissioning flow
        DiscoveryCapabilities::new(true, true, true),
        discriminator,
        passcode,
    ) {
        Ok(qr_data) => qr_data,
        _ => return Err(JsValue::from("failed to pack QR code")),
    };
    let needed_version = compute_qr_version(&qr_data);
    let code = match QrCode::with_version(
        &qr_data,
        Version::Normal(needed_version),
        qrcode::EcLevel::M,
    ) {
        Ok(code) => code,
        _ => return Err(JsValue::from("failed to pack QR code")),
    };

    Ok(code
        .render::<svg::Color>()
        .min_dimensions(36, 36)
        .quiet_zone(false)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build()
        .replace(r#"<?xml version="1.0" standalone="yes"?>"#, ""))
}