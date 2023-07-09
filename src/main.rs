use anyhow::Result;
use onboarding_payload::{CustomFlow, DiscoveryCapabilities};
use qr::{compute_qr_version, pack_qr_data};
use qrcode::{render::unicode, QrCode, Version};
use std::process;

mod base38;
mod cli;
mod onboarding_payload;
mod qr;

fn main() -> Result<()> {
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
    let passcode = matches
        .get_one::<String>(cli::PASSCODE)
        .unwrap()
        .parse::<u32>()
        .unwrap_or_else(|_| {
            eprintln!("failed, <{}> should be integer", cli::PASSCODE);
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

    print_qr(vid, pid, passcode, discriminator)?;
    Ok(())
}

fn print_qr(vid: u16, pid: u16, passcode: u32, discriminator: u16) -> Result<()> {
    let qr_data = pack_qr_data(
        vid,
        pid,
        CustomFlow::StandardCommissioningFlow, //  Standard commissioning flow
        DiscoveryCapabilities::new(true, true, true),
        discriminator,
        passcode,
    )?;
    let needed_version = compute_qr_version(&qr_data);
    let code = QrCode::with_version(
        &qr_data,
        Version::Normal(needed_version),
        qrcode::EcLevel::M,
    )
    .unwrap();
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("\n{}", image);
    Ok(())
}
