use std::io;

mod app;
mod tui;
mod views;
mod widgets;
mod words;

fn main() -> io::Result<()> {
  let mut terminal = tui::init().expect("Failed to init TUI");
  let app_result = app::App::default().run(&mut terminal);
  tui::restore().expect("Failed to gracefully exit");
  app_result
}
