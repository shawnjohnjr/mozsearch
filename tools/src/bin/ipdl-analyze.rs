use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::path::Path;
use std::path::PathBuf;

extern crate tools;
extern crate ipdl_parser;
extern crate getopts;

use getopts::Options;

use tools::find_source_file;
use tools::analysis::{read_analysis, read_source, read_jumps};
use tools::format::format_file_data;
use tools::config;

use ipdl_parser::parser;

use tools::output::{PanelItem, PanelSection};

fn get_options_parser() -> Options {
    let mut opts = Options::new();
    opts.optmulti("I", "include",
                  "Additional directory to search for included protocol specifications",
                  "DIR");
    opts.optopt("d", "outheaders-dir",
                "Directory into which C++ headers will be generated. \
                 A protocol Foo in the namespace bar will cause the headers \
                 dir/bar/Foo.h, dir/bar/FooParent.h, and dir/bar/FooParent.h \
                 to be generated",
                "HDR_DIR");
    opts.optopt("o", "outcpp-dir",
                "Directory into which C++ sources will be generated \
                A protocol Foo in the namespace bar will cause the sources \
                cppdir/FooParent.cpp, cppdir/FooChild.cpp \
                to be generated",
                "CPP_DIR");
    opts
}

fn main() {
    let args : Vec<String> = env::args().collect();

    let opts = get_options_parser();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(f) => { panic!(f.to_string()) },
    };

    if matches.free.is_empty() {
        panic!("Expected at least one IPDL file to be specified.");
    }

    let mut include_dirs = Vec::new();
    for i in matches.opt_strs("I") {
        include_dirs.push(PathBuf::from(i))
    }

    let mut file_names = Vec::new();
    for f in matches.free {
        file_names.push(PathBuf::from(f));
    }

    let maybe_tus = parser::parse(&include_dirs, file_names);

    if maybe_tus.is_none() {
        println!("Specification could not be parsed.");
        return;
    }

    let tus = maybe_tus.unwrap();

    for (path, tu) in tus {
        if let Some((ns, protocol)) = tu.protocol {
            for message in protocol.messages {
                println!("MSG: {:?}\n", message);
            }
        }
    }
}
