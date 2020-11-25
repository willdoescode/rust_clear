use term_size;
use crossterm::{QueueableCommand};

fn main() {
  for n in 0..term_size::dimensions().unwrap().1 {
    std::io::stdout().queue(crossterm::cursor::MoveTo(0, 0)).unwrap();
    if n != term_size::dimensions().unwrap().1 - 1 {
      print!("\n")
    }
  }
}
