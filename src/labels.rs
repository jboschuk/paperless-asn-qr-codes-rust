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
    // Custom,
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
            // PredefinedFormat::Custom => {
            //     if let (Some(width), Some(height), Some(left_offset), Some(top_offset)) = (
            //         self.label_width,
            //         self.label_height,
            //         self.left_offset,
            //         self.top_offset,
            //     ) {
            //         Some(LabelInfo {
            //             label_size: (width, height),
            //             labels_horizontal: self.labels_horizontal.unwrap_or(1),
            //             labels_vertical: self.labels_vertical.unwrap_or(1),
            //             margin: (left_offset, top_offset),
            //             gutter_size: (
            //                 self.gutter_h.unwrap_or(Mm(0.0)),
            //                 self.gutter_v.unwrap_or(Mm(0.0)),
            //             ),
            //             page_size: self.page_size.clone(),
            //         })
            //     } else {
            //         None
            //     }
            // }
        }
    }
}
