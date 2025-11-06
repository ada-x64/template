use std::process::exit;

use app::{AppSettings, prelude::*};
use bevy::reflect::{DynamicEnum, DynamicTuple, DynamicVariant};
use clap::{Parser, arg};

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[derive(Parser, Debug)]
struct Cli {
    /// Name of the screen to start on
    #[arg(short, long)]
    screen: Option<String>,

    /// Show build info
    #[arg(long)]
    build_info: bool,
}

pub fn parse_args() -> AppSettings {
    let args = Cli::parse();

    if args.build_info {
        println!("Build timestamp: {}", built_info::BUILT_TIME_UTC);
        exit(0);
    }

    let initial_screen = args
        .screen
        .and_then(|scr| {
            let mut tup = DynamicTuple::default();
            tup.insert(ScreenStatus::Loading);
            let dynamic = DynamicEnum::new(scr, DynamicVariant::Tuple(tup));
            let res = Screens::from_reflect(&dynamic);
            println!("opening to {res:?}");
            res
        })
        .unwrap_or_default();

    AppSettings {
        initial_screen,
        ..Default::default()
    }
}
