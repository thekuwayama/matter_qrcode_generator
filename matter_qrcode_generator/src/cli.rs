use clap::{crate_description, crate_name, crate_version, Command, arg};

pub(crate) const VENDOR_ID: &str = "VENDOR_ID";
pub(crate) const PRODUCT_ID: &str = "PRODUCT_ID";
pub(crate) const PASSCODE: &str = "PASSCODE";
pub(crate) const DISCRIMINATOR: &str = "DISCRIMINATOR";
pub(crate) const FILE: &str = "FILE";

pub(crate) fn build() -> Command {
    Command::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(arg!(<VENDOR_ID>).help("Vendor ID").required(true))
        .arg(arg!(<PRODUCT_ID>).help("Product ID").required(true))
        .arg(arg!(<PASSCODE>).help("passcode").required(true))
        .arg(arg!(<DISCRIMINATOR>).help("discriminator").required(true))
        .arg(arg!(<FILE>).long("file").short('f').required(false).help("path to the .png output file"))
}
