use std::path::PathBuf;

use clap::Parser;
use clap::ValueEnum;
use printpdf::Mm;

use paperless_asn_qr_codes_rust::labels::{PaperSize, PredefinedFormat};
use paperless_asn_qr_codes_rust::BarcodeFormat;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
pub struct Args {
    /// Starting ASN number
    pub asn_start: u32,

    /// Number of digits in the ASN number
    #[arg(short, long, default_value="7")]
    pub digits: usize,

    /// Label format ('avery3657' or 'custom')
    #[arg(value_enum)]
    pub format: PredefinedFormat,

    /// Page size ('letter' or 'a4')
    #[arg(value_parser = parse_paper_size)]
    pub page_size: PaperSize,

    /// Output filename
    #[arg(default_value = "labels.pdf")]
    pub output: PathBuf,

    /// Show label borders, helpful for test prints
    #[arg(long, short = 'b')]
    pub border: bool,

    /// Tag
    #[arg(short = 't', long, default_value = "ASN")]
    pub tag: String,

    /// Barcode format
    #[arg(short = 'c', long, default_value = "qr")]
    pub code_format: BarcodeFormat,

    /// Label width in millimeters (used if format is 'Custom')
    #[arg(long, value_parser = parse_mm, required_if_eq("format", "Custom"))]
    pub label_width: Option<Mm>,

    /// Label height in millimeters (used if format is 'Custom')
    #[arg(long, value_parser = parse_mm, required_if_eq("format", "Custom"))]
    pub label_height: Option<Mm>,

    /// Number of labels horizontally (used if format it 'Custom')
    #[arg(long, required_if_eq("format", "Custom"))]
    pub labels_horizontal: Option<u32>,

    /// Number of labels vertically (used if format it 'Custom')
    #[arg(long, required_if_eq("format", "Custom"))]
    pub labels_vertical: Option<u32>,

    /// Top offset of the first label in millimeters (used if format is 'Custom')
    #[arg(long, value_parser = parse_mm, required_if_eq("format", "Custom"))]
    pub top_offset: Option<Mm>,

    /// Left offset of the first label in millimeters (used if format is 'Custom')
    #[arg(long, value_parser = parse_mm, required_if_eq("format", "Custom"))]
    pub left_offset: Option<Mm>,

    /// Vertical gutter (space) between labels in millimeters (used if format is 'Custom'); defaults to 0
    #[arg(long, value_parser = parse_mm, required_if_eq("format", "Custom"))]
    pub gutter_v: Option<Mm>,

    /// Horizontal gutter (space) between labels in millimeters (used if format is 'Custom'), defaults to 0
    #[arg(long, value_parser = parse_mm, required_if_eq("format", "Custom"))]
    pub gutter_h: Option<Mm>,
}

fn parse_mm(s: &str) -> Result<Mm, String> {
    s.parse::<f32>().map(Mm).map_err(|e| e.to_string())
}

fn parse_paper_size(s: &str) -> Result<PaperSize, String> {
    match s {
        "a4" => Ok(PaperSize::A4),
        "letter" => Ok(PaperSize::Letter),
        _ => Err(format!("Unknown paper size: {s}")),
    }
}