use anyhow::Result;
use onboarding_payload::{CustomFlow, DiscoveryCapabilities};
use image::Luma;
use qr::{compute_qr_version, pack_qr_data};
use qrcode::{render::unicode, QrCode, Version};
use std::process;
use std::fs::OpenOptions;
use regex::Regex;

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
    let file = matches.get_one::<String>(cli::FILE);

    print_qr(vid, pid, passcode, discriminator, file.cloned())?;
    Ok(())
}

fn do_print_qr(vid: u16, pid: u16, passcode: u32, discriminator: u16) -> Result<QrCode> {
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
    )?;

    Ok(code)
}

fn print_qr(vid: u16, pid: u16, passcode: u32, discriminator: u16, file: Option<String>) -> Result<()> {
    let code = do_print_qr(vid, pid, passcode, discriminator)?;
    match file {
        Some(f) => {
            Regex::new(r"^.*\.png$")
                .unwrap()
                .is_match(&f).then_some(())
                .unwrap_or_else(|| panic!("expected file path *.png, but got {}", &f));

            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&f)
                .unwrap_or_else(|_| panic!("failed to open the file {}", &f));

            let image = code.render::<Luma<u8>>().build();
            image.save(&f).unwrap_or_else(|_| panic!("failed to save the file {}", &f));
        },
        None => {
            let image = code
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();
            println!("\n{}", image);
        },
    };

    Ok(())
}
