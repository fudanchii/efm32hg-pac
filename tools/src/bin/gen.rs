extern crate form;
use form::create_directory_structure;
use std::io::Write;
use svd2rust::util::build_rs;

use std::fs::File;

use std::io::Read;

extern crate svd2rust;
use svd2rust::{generate, target::Target, util::SvdError};

extern crate svd_parser as svd;

extern crate quote;
use quote::{multi_zip_expr, nested_tuples_pat, pounded_var_names, quote, quote_each_token};

extern crate glob;
use glob::glob;

extern crate rustfmt_nightly;
use rustfmt_nightly::{Config, Input, Session};

use std::path::PathBuf;

pub fn main() -> Result<(), SvdError> {
    let name = "svd/EFM32HG309F64.svd";
    let output_dir = "src/";

    //read svd
    let xml = &mut String::new();
    File::open(name)?.read_to_string(xml)?;
    let device = svd::parse(xml);

    //parse svd
    let mut device_x = String::new();
    let items = generate::device::render(&device, &Target::CortexM, true, &mut device_x)?;

    //save
    writeln!(File::create("device.x").unwrap(), "{}", device_x).unwrap();
    writeln!(File::create("build.rs").unwrap(), "{}", build_rs()).unwrap();

    //form lib and save
    let input = format!("{}", quote!(#(#items)*));
    create_directory_structure(output_dir, input)?;

    //glob src and rustfmt
    let config = Config::default();
    let mut buf = Vec::<u8>::new();
    let mut session = Session::new(config, Some(&mut buf));

    for path in glob("src/**/*.rs").unwrap().filter_map(Result::ok) {
        println!("{:?}", path);
        let input = Input::File(PathBuf::from(path));
        let _something = session.format(input).unwrap();
    }

    Ok(())
}
