use clap::{crate_description, crate_name, crate_version, Arg, Command};

pub(crate) const VENDOR_ID: &str = "VENDOR_ID";
pub(crate) const PRODUCT_ID: &str = "PRODUCT_ID";
pub(crate) const HARDWARE_VERSION: &str = "HARDWARE_VERSION";
pub(crate) const SOFTWARE_VERSION: &str = "SOFTWARE_VERSION";
pub(crate) const SERIAL_NUMBER: &str = "SERIAL_NUMBER";
pub(crate) const DEVICE_NAME: &str = "DEVICE_NAME";
pub(crate) const PASSCODE: &str = "PASSCODE";
pub(crate) const DISCRIMINATOR: &str = "DISCRIMINATOR";

pub(crate) fn build() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::new(VENDOR_ID).help("Vendor ID").required(true))
        .arg(Arg::new(PRODUCT_ID).help("Product ID").required(true))
        .arg(
            Arg::new(HARDWARE_VERSION)
                .help("hardware version")
                .required(true),
        )
        .arg(
            Arg::new(SOFTWARE_VERSION)
                .help("software version")
                .required(true),
        )
        .arg(Arg::new(SERIAL_NUMBER).help("serial number").required(true))
        .arg(Arg::new(DEVICE_NAME).help("device name").required(true))
        .arg(Arg::new(PASSCODE).help("passcode").required(true))
        .arg(Arg::new(DISCRIMINATOR).help("discriminator").required(true))
}
