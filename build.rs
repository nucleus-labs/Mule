
use peacock::build::WidgetTreeContext;
use glob::glob;

use std::fs::{File, remove_file};
use std::ffi::OsString;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut ctx = WidgetTreeContext::new();
    for xml_file in glob("static/xml/**/*.xml").expect("Failed to read glob pattern") {
        if xml_file.is_err() {
            panic!("Errored while searching for XML files");
        }
        let os_xml_file: OsString = xml_file.unwrap().into();
        
        let result = ctx.parse_xml(os_xml_file.clone());
        if result.is_err() {
            panic!("Failed to parse XML file: {:?}", result.unwrap_err());
        }

        println!("cargo::rerun-if-changed={}", os_xml_file.into_string().unwrap());
    }
    for css_file in glob("static/css/**/*.css").expect("Failed to read glob pattern") {
        if css_file.is_err() {
            panic!("Errored while searching for CSS files");
        }
        let os_css_file: OsString = css_file.unwrap().into();
        
        let result = ctx.parse_css(os_css_file.clone());
        if result.is_err() {
            panic!("Failed to parse CSS file: {:?}", result.unwrap_err());
        }

        println!("cargo::rerun-if-changed={}", os_css_file.into_string().unwrap());
    }

    let gen_os_str: OsString = format!("{}/{}", std::env::var("OUT_DIR").unwrap(), "widgets-gen.rs").into();
    let gen_path = Path::new(&gen_os_str);
    if gen_path.exists() {
        remove_file(gen_path).unwrap();
    }

    File::create_new(gen_path).unwrap().write_all(ctx.gen_source().as_bytes()).unwrap();
}
