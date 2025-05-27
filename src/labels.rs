use printpdf::Mm;

#[derive(Debug)]
pub struct LabelInfo {
    pub labels_horizontal: u32,
    pub labels_vertical: u32,
    pub label_size: (Mm, Mm),
    pub gutter_size: (Mm, Mm),
    pub margin: (Mm, Mm),
    pub page_size: PaperSize,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum PaperSize {
    A4,
    Letter,
}

#[derive(Debug, clap::ValueEnum, Clone)]
pub enum PredefinedFormat {
    Avery3657,
    AveryL4731,
}

impl PredefinedFormat {
    pub fn get_label_info(&self) -> Option<LabelInfo> {
        match self {
            PredefinedFormat::Avery3657 => Some(LabelInfo {
                labels_horizontal: 4,
                labels_vertical: 10,
                label_size: (Mm(48.5), Mm(25.4)),
                gutter_size: (Mm(0.0), Mm(0.0)),
                margin: (Mm(8.0), Mm(21.75)),
                page_size: PaperSize::A4,
            }),
            PredefinedFormat::AveryL4731 => Some(LabelInfo {
                labels_horizontal: 7,
                labels_vertical: 27,
                label_size: (Mm(25.4), Mm(10.)),
                gutter_size: (Mm(2.5), Mm(0.)),
                margin: (Mm(9.0), Mm(13.5)),
                page_size: PaperSize::A4
            })
        }
    }
}
