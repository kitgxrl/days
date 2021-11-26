#[macro_use]
extern crate log;

use clap::clap_app;
use std::io::Write;

use env_logger::{fmt::Color, Builder};
use log::{Level, LevelFilter};

fn main() {
    let matches = clap_app!(day =>
        (version: "0.0.1")
        (author: "Rain <kitgxrl@protonmail.com")
        (about: "Portage like package manager")
        (@arg verbose: -v --verbose "Shows DEBUG logs")
        (@subcommand install =>
            (about: "Install a package")
            (@arg PACKAGE: +required "What package to install, i.e: `dev-lang/rust`")
            (@arg version: -v --version "What version of the package to install")
        )
        (@subcommand uninstall =>
            (about: "Uninstall a package")
            (@arg PACKAGE: +required "What package to uninstall, i.e: `dev-lang/rust`")
            (@arg version: -v --version "What version of the package to uninstall")
        )
        (@subcommand search =>
            (about: "Searches for packages")
            (@arg TO_SEARCH: +required "What's going to be searched")
            (@arg version: "Specify package version")
            (@arg category: "Specify package category")
        )
    )
    .get_matches();

    init_logger(matches.is_present("verbose"));
}

fn init_logger(verbose: bool) {
    let verbose = match verbose {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    Builder::new()
        .filter_level(verbose)
        .format(|buf, record| match record.level() {
            Level::Info => {
                let mut sty = buf.style();

                sty.set_color(Color::Green);
                sty.set_bold(true);
                sty.set_intense(true);

                writeln!(buf, "{} {}", sty.value("~"), record.args())
            }
            Level::Debug => {
                let mut sty = buf.style();
                sty.set_color(Color::Blue);
                sty.set_bold(true);
                sty.set_intense(true);

                writeln!(buf, "{} {}", sty.value("DEBUG ~"), record.args())
            }
            Level::Warn => {
                let mut sty = buf.style();
                sty.set_color(Color::Yellow);
                sty.set_bold(true);
                sty.set_intense(true);

                writeln!(buf, "{} {}", sty.value("WARN ~"), record.args())
            }
            Level::Error => {
                let mut sty = buf.style();
                sty.set_color(Color::Red);
                sty.set_bold(true);
                sty.set_intense(true);

                writeln!(buf, "{} {}", sty.value("FATAL ~"), record.args())
            }
            _ => writeln!(buf, "{} ~ {}", record.level(), record.args()),
        })
        .init();
}
