use std::{fs::File, io::{BufWriter, Write}, path::PathBuf};

use anyhow::Error;
use ::image::{ImageBuffer, Luma};
use printpdf::{
    Color, Mm, Op, ParsedFont, PdfDocument, PdfPage, PdfSaveOptions, Point, Pt, RawImage, Rect,
    Rgb, TextItem, XObjectTransform,
};
use qrcode::{QrCode, Version};

pub mod labels;
pub use labels::{LabelInfo, PaperSize};

const BLACK: Color = Color::Rgb(Rgb {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    icc_profile: None,
});

fn get_page_dimensions(paper: &PaperSize) -> (Mm, Mm) {
    match paper {
        PaperSize::A4 => (Mm(215.0), Mm(297.0)),
        PaperSize::Letter => todo!(),
    }
}

/// Main fuction which generates the pdf file with the barcodes
pub fn generate_pdf(label_info: LabelInfo, page_size: PaperSize, asn_start: u32, digits: usize, tag: String, code_format: BarcodeFormat, border: bool, output: PathBuf) -> Result<(), Error> {
    let (page_width, page_height) = get_page_dimensions(&page_size);

    let mut doc = PdfDocument::new("Paperless ASN Barcode Generator");
    let mut msg_vec = Vec::new();

    let font_size_text_pt = Pt(9.0);
    let font_size_text_mm = Mm::from(font_size_text_pt);

    // add font file
    let opensans_bytes = include_bytes!("../assets/fonts/OpenSans-Regular.ttf");
    let font = ParsedFont::from_bytes(opensans_bytes, 0, &mut msg_vec).unwrap();
    let font_id = doc.add_font(&font);

    // Generate content
    let mut page1_contents = vec![
        Op::SetOutlineColor {
            col: Color::Rgb(Rgb::new(0.75, 1.0, 0.64, None)),
        },
        Op::SetOutlineThickness { pt: Pt(2.0) },
        Op::SetFillColor { col: BLACK },
        Op::SetFontSize {
            size: font_size_text_pt,
            font: font_id.clone(),
        },
        // Op::SetWordSpacing { pt: Pt(33.0) },
        // Op::SetCharacterSpacing { multiplier: 10.0 },
        // Op::StartTextSection,
    ];

    // calculate values which are the same for all labels
    let label_width = label_info.label_size.0;
    let label_height = label_info.label_size.1;
    let label_width_pt = Pt::from(label_width);
    let label_height_pt = Pt::from(label_height);

    let mut text_x_offset = Pt(2.0); // place text with 2pt distance to the right of QR code
    let text_y_offset = (label_height_pt - font_size_text_pt) / 2.0; // text centered vertically

    let dpi = 300.0;

    for row in 0..label_info.labels_vertical {
        for col in 0..label_info.labels_horizontal {
            let running_number = asn_start + row * label_info.labels_horizontal + col;
            let label_text = format!("{}{:0width$}", tag, running_number, width=digits);
            println!("Putting {} at position ({}, {})", &label_text, &row, &col);

            // Calculate label position
            // this is lower left corner of label
            let label_x = label_info.margin.0
                + (label_info.label_size.0 + label_info.gutter_size.0) * col as f32;
            let label_y = label_info.margin.1
                + (label_info.label_size.1 + label_info.gutter_size.1) * row as f32;
            let label_x_pt = Pt::from(label_x);
            let label_y_pt = Pt::from(label_y);

            let barcode_img_data = generate_barcode(
                &code_format,
                &label_text,
                label_info.label_size.1,
                label_info.label_size.0,
            ).map_err(|err| anyhow::anyhow!(err) )?;


            // Generate label content
                    let mut image_buf = Vec::new();
                    barcode_img_data.write_to(
                        &mut std::io::Cursor::new(&mut image_buf),
                        image::ImageFormat::Png,
                    )?;
                    
                    let image = RawImage::decode_from_bytes(&image_buf.as_slice(), &mut msg_vec)
                        .expect("Failed to generate RAW image");
                
                    // Not sure why height and width are 4-fold?
                    let bw = image.width / 4;
                    let bh = image.height / 4;
                    let bw_pt = Pt(bw as f32);
                    let bh_pt = Pt(bh as f32);

                    let barcode_x_pt = label_x_pt + Pt(7.0);
                    let barcode_y_pt = label_y_pt + ((label_height_pt - bh_pt) / 2.0); // Adjust vertical position

                    // update text x offset
                    text_x_offset = bw_pt + Pt(4.0);

                    let img_xobj_id = doc.add_image(&image);
                    page1_contents.extend([
                        Op::SaveGraphicsState,
                        Op::UseXobject {
                            id: img_xobj_id.clone(),
                            transform: XObjectTransform {
                                translate_x: Some(barcode_x_pt),
                                translate_y: Some(barcode_y_pt),
                                rotate: None,
                                scale_x: None,
                                scale_y: None,
                                dpi: Some(dpi),
                            },
                        },
                        Op::RestoreGraphicsState
                    ]);

            // draw label border if enabled (via `--border` flag)
            if border {
                let rect = Rect {
                    x: label_x_pt,
                    y: label_y_pt + label_height_pt, // start point is TOP left corner of RECT
                    width: label_width_pt,
                    height: label_height_pt,
                };
                page1_contents.push(Op::DrawLine {
                    line: rect.to_line(),
                });
            };

            // write label_text as plain text
            let text_x_pt = label_x_pt + text_x_offset;
            let text_y_pt = label_y_pt + text_y_offset;
            let pos = Point {
                x: text_x_pt,
                y: text_y_pt,
            };
            page1_contents.push(Op::StartTextSection);
            page1_contents.push(Op::SetTextCursor { pos });
            page1_contents.push(Op::WriteText {
                items: vec![TextItem::Text(label_text)],
                font: font_id.clone(),
            });
            page1_contents.push(Op::EndTextSection);
        }
    }

    let page1 = PdfPage::new(page_width, page_height, page1_contents);

    // Save the file
    let save_options = PdfSaveOptions {
        subset_fonts: true,
        ..Default::default()
    };
    let pdf_bytes = doc
        .with_pages(vec![page1])
        .save(&save_options, &mut msg_vec);

    let file = File::create(&output)?;
    let mut writer = BufWriter::new(file);
    let _ = writer.write_all(&pdf_bytes);

    println!("PDF generated successfully: {}", output.display());
    Ok(())
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum BarcodeFormat {
    Qr,
    // Code128,
    // EAN,
}

fn generate_barcode(
    format: &BarcodeFormat,
    data: &str,
    label_height: Mm,
    label_width: Mm,
) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, String> {
    match format {
        BarcodeFormat::Qr => {
            // use Micro QR codes for label height < 20mm
            let version = if label_height < Mm(20.0) {
                Version::Micro(4)
            } else {
                Version::Normal(1)
            };
            let code = QrCode::with_version(data, version, qrcode::EcLevel::M)
                .expect("Failed to create QR code");
            let code = code.render::<Luma<u8>>().build();
            Ok(code)
        }
        _ => todo!(),
    }
}
