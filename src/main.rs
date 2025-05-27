use anyhow::Error;
use clap::Parser;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::path::PathBuf;

mod cli;
use cli::Args;
use paperless_asn_qr_codes_rust::{generate_pdf, LabelInfo};

fn main() -> Result<(), Error> {
    let args = Args::parse();

    generate_pdf(
        args.format.get_label_info().unwrap(),
        args.page_size,
        args.asn_start,
        args.digits,
        args.tag,
        args.code_format,
        args.border,
        args.output,
        args.font_size,
    )
    .inspect_err(|e| eprintln!("{}", e))?;

    Ok(())
}