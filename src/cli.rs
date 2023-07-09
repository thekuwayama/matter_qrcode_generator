use clap::{crate_description, crate_name, crate_version, Arg, Command};

pub(crate) const VENDOR_ID: &str = "VENDOR_ID";
pub(crate) const PRODUCT_ID: &str = "PRODUCT_ID";
pub(crate) const PASSCODE: &str = "PASSCODE";
pub(crate) const DISCRIMINATOR: &str = "DISCRIMINATOR";

pub(crate) fn build() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::new(VENDOR_ID).help("Vendor ID").required(true))
        .arg(Arg::new(PRODUCT_ID).help("Product ID").required(true))
        .arg(Arg::new(PASSCODE).help("passcode").required(true))
        .arg(Arg::new(DISCRIMINATOR).help("discriminator").required(true))
}
