use std::{time::Duration, thread};

use ansi_term::Color::{Yellow,Red,Cyan,Green};
use indicatif::{ProgressStyle, ProgressBar, MultiProgress};
use rand::seq::SliceRandom;
use rand::Rng;

static PACKAGES: &[&str] = &[
    "ui-base",
    "distro",
    "kernel-core",
    "kernel-wrapper",
    "openssl",
    "driver",
];

static COMMANDS: &[&str] = &[
    "cmake .",
    "make",
    "make clean",
    "gcc foo.c -o foo",
    "gcc bar.c -o bar",
    "./helper.sh rebuild-cache",
    "make all-clean",
    "make test",
];

fn main() {
    println!("このシステムは{}ウイルスによって乗っ取られました。",Red.paint("ようじょ"));
    println!("{}成分をダウンロードしています...",Red.paint("ようじょ"));
    let mut rng = rand::thread_rng();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");
    let m = MultiProgress::new();
    let handles: Vec<_> = (0..8u32)
        .map(|i| {
            let count = rng.gen_range(30..80);
            let pb = m.add(ProgressBar::new(count));
            pb.set_style(spinner_style.clone());
            pb.set_prefix(format!("{}",Yellow.paint(format!("[{}/?]", i + 1))));
            thread::spawn(move || {
                let packages = PACKAGES.iter().map(|f| Green.paint(f.to_string())).collect::<Vec<_>>();
                let commands = COMMANDS.iter().map(|f| Cyan.paint(f.to_string())).collect::<Vec<_>>();
                let mut rng = rand::thread_rng();
                let pkg = packages.choose(&mut rng).unwrap();
                for _ in 0..count {
                    let cmd = commands.choose(&mut rng).unwrap();
                    pb.set_message(format!("{pkg}: {cmd}"));
                    pb.inc(1);
                    thread::sleep(Duration::from_millis(rng.gen_range(25..200)));
                }
                pb.finish_with_message("waiting...");
            })
        })
        .collect();
    for h in handles {
        let _ = h.join();
    }
    m.clear().unwrap();
    println!("{}ウイルスはインストールされました。",Red.paint("ようじょ"));
}