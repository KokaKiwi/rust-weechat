extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    const INCLUDED_TYPES: &[&str] = &["t_weechat_plugin", "t_gui_buffer"];
    const INCLUDED_VARS: &[&str] = &[
        "WEECHAT_PLUGIN_API_VERSION",
        "WEECHAT_RC_ERROR"
    ];
    let mut builder = bindgen::Builder::default()
        .rustfmt_bindings(true);

    builder = builder.header("src/wrapper.h");

    for t in INCLUDED_TYPES {
        builder = builder.whitelist_type(t);
    }

    for v in INCLUDED_VARS {
        builder = builder.whitelist_var(v);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
