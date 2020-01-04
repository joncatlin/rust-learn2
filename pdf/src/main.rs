use wkhtmltopdf::*;
use std::env;
use std::fs;

// Remember to ensure that the wkhtmltox.dll is in the PATH on windows!!!!!

fn main() {

    let html_file = format!("{}\\data-in\\test.html", env::var("CARGO_MANIFEST_DIR").unwrap());
    let pdf_file = format!("{}\\data-out\\test.pdf", env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("html_file={}", html_file);

    let html = fs::read_to_string(html_file).expect("Filed to open the file contining the html to be converted.");

    //
//    let object_settings = format!("{}\\data-cache\\", env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");

    let mut pdf_builder = pdf_app.builder();
/*
    unsafe {
        pdf_builder.global_setting("grayscale", "");
    }
*/
    let mut pdfout = pdf_builder
        .orientation(Orientation::Landscape)
        .margin(Size::Inches(2))
        .title("NO TITLE")
        .build_from_html(&html)
        .expect("failed to build pdf");

//    for index in 0..100 {
/*
    let mut pdfout = pdf_app.builder()
        .orientation(Orientation::Landscape)
        .margin(Size::Inches(2))
        .title("Awesome Foo")
        .build_from_html(&html)
        .expect("failed to build pdf");
*/
        let pdf_file = format!("{}\\data-out\\test{}.pdf", env::var("CARGO_MANIFEST_DIR").unwrap(), 0);
        pdfout.save(pdf_file).expect("Failed to save converted html");
//    }
}

