extern crate qrcode;
extern crate image;
extern crate argparse;

use argparse::{ArgumentParser, Store};

use qrcode::QrCode;
use image::Luma;

struct ArgOptions {
    data: String,
    out: String,
}

fn main() {
    let mut opts = ArgOptions {
        out: "./qr.png".to_string(),
        data: String::new(),
    };

    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Create image with qr");
        ap.refer(&mut opts.data).add_option(
            &["--data"],
            Store,
            "Data to convert",
        );

        ap.refer(&mut opts.out).add_option(
            &["--out"],
            Store,
            "Output filepath",
        );
        ap.parse_args_or_exit();
    }

    // Encode some data into bits.
    let code = QrCode::new(&opts.data).unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    image.save(&opts.out).unwrap();
}
