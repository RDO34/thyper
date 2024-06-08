use std::io;

use clap::Parser;

mod app;
mod errors;
mod file_io;
mod persistence;
mod scoring;
mod tui;
mod views;
mod widgets;
mod words;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Play a quick game
    #[arg(short, long)]
    quick: bool,

    /// Print scores
    #[arg(short, long)]
    scores: bool
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.scores {
        let persisted_scores = persistence::get_persisted_scores();
        println!("pb = {}, avg = {}", persisted_scores.pb, persisted_scores.avg);
        return Ok(());
    }

    errors::install_hooks().expect("failed to install error hooks");
    let mut terminal = tui::init().expect("failed to init tui");
    let (app_result, wpm) = app::App::default().run(&mut terminal, &args.quick);
    tui::restore().expect("failed to gracefully exit");

    if args.quick {
        let persisted_scores = persistence::get_persisted_scores();
        println!("wpm = {}, pb = {}, avg = {}", wpm, persisted_scores.pb, persisted_scores.avg);
    }

    app_result
}

