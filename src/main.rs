use std::io::Write;
use std::io::stdout;
use std::io::stdin;

use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;

fn main() -> anyhow::Result<()> {
    let mut stdout = stdout().into_raw_mode()?;
    write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide)?;

    let stdin = stdin();

    let mut state = GameState::new();
    state.render()?;

    for c in stdin.keys() {
        state.handle_input(c?)?;
    }

    Ok(())
}

pub struct GameState {
  width: usize,
  height: usize,
  x: usize,
  y: usize
}

impl GameState {
  pub fn new() -> Self {
    Self {
      width: 40,
      height: 20,
      x: 10,
      y: 10,
    }
  }

  pub fn handle_input(&mut self, c: Key) -> anyhow::Result<()> {
      match c {
          Key::Char('w') => if self.y != 0 { self.y -= 1 },
          Key::Char('s') => self.y = usize::min(self.y + 1, self.height - 1),
          Key::Char('d') => self.x = usize::min(self.x + 1, self.width - 1),
          Key::Char('a') => if self.x != 0 { self.x -= 1 },
          Key::Char('q') => {
              let mut stdout = stdout().into_raw_mode()?;
              write!(stdout, "{}{}", termion::clear::All, termion::cursor::Show)?;
              std::process::exit(0);
          },
          _ => {},
      }
      self.render()?;
      Ok(())
  }

  pub fn render(&self) -> anyhow::Result<()> {
    let mut stdout = stdout().into_raw_mode()?;
    let h_border = vec!["#"; self.width + 2].join("");
    let v_border = vec!["#", &vec![" "; self.width].join(""), "#"].join("");
    write!(stdout, "{}{} Press q to exit. Navigate with wasd\r\n", termion::clear::All, termion::cursor::Hide)?;
    write!(stdout, "{}", h_border)?;
    for i in 0..self.height {
        if i == self.y {
            // insert our position indicator
            let mut this_line = v_border.clone();
            this_line.replace_range((self.x + 1)..(self.x + 2), "x");
            write!(stdout, "\r\n{}", this_line)?;
            continue;
        }
        write!(stdout, "\r\n{}", v_border)?;
    }
    write!(stdout, "\r\n{}\r\n", h_border)?;
    Ok(())
  }
}

