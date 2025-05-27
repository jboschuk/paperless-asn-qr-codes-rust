# Paperless ASN QR Code Generator

A tool to generate ASN QR Codes for paperless-ngx with a built-in cli.

Inspired by the idea of a Rust port of [paperless-asn-qr-codes](https://github.com/entropia/paperless-asn-qr-codes).

**Note: This is still work in progress and in it's early stages. It's also a free-time project, so please expect delays in the implementation of missing functionalities.**


## Usage

### As a CLI
```bash
Usage: paperless-asn-qr-codes-rust.exe [OPTIONS] <ASN_START> <FORMAT> <PAGE_SIZE> [OUTPUT]

Arguments:
  <ASN_START>  Starting ASN number
  <FORMAT>     Label format ('avery3657' or 'custom') [possible values: avery3657, avery-l4731]
  <PAGE_SIZE>  Page size ('letter' or 'a4')
  [OUTPUT]     Output filename [default: labels.pdf]

Options:
  -d, --digits <DIGITS>
          Number of digits in the ASN number [default: 7]
  -f, --font-size <FONT_SIZE>
          Font size of the label text [default: 9]
  -b, --border
          Show label borders, helpful for test prints
  -t, --tag <TAG>
          Tag [default: ASN]
  -c, --code-format <CODE_FORMAT>
          Barcode format [default: qr] [possible values: qr]

# The following will generate one A4 page with Avery 3657 labels, starting at ASN 0
paperless_asn_qr_codes_rust.exe 0 avery3657 a4 labels.pdf
```

### As a crate
You could also include this in your own Rust project. You will want to use the `generate_pdf` method to generate the pdf:
```rust
use paperless_asn_qr_codes_rust::*;

// Predefined label size
let label_info = PredefinedFormat::Avery3657.get_label_info();
// or provide your own; see the predefined formats for details
let label_info = LabelInfo { /* .. */ };
generate_pdf(label_info, page_size, asn_start, tag, code_format, border, output);
```


## Supported sheets

Some different sheet types are supported with the -f/--format argument, however, not all are tested.

- Avery 3657 (40 Labels on A4)

## Attribution

This tool started as a Rust port of the Python tool by @entropia [paperless-asn-qr-codes](https://github.com/entropia/paperless-asn-qr-codes), which in turn "is based upon a public domain label generation class from @timrprobocom https://gist.github.com/timrprobocom/3946aca8ab75df8267bbf892a427a1b7".

This tool includes the OpenSans Regular font from Google Fonts, desiged by Steve Matteson, which is licensed under the SIL Open Font License, Version 1.1. The full license statement can be found in [LICENSE.SIL.OpenFont](LICENSE.SIL.OpenFont).