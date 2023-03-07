use crossterm::{
    event::{EventStream, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::{future::FutureExt, Stream, StreamExt};
use std::io::stdout;
use std::pin::Pin;
use tokio_stream::wrappers::IntervalStream;

use crate::font;

const SECONDS_IN_MINUTE: usize = 60;
const SECONDS_IN_HOUR: usize = 60 * SECONDS_IN_MINUTE;

struct Symbol(Vec<&'static str>);

struct Text(Vec<Symbol>);

impl Text {
    fn width(&self) -> usize {
        let mut width = 0;
        for symbol in self.0.iter() {
            width += symbol.0[0].chars().count();
        }
        width
    }

    fn height(&self) -> usize {
        let mut height = 0;
        for symbol in self.0.iter() {
            height = height.max(symbol.0.len());
        }
        height
    }
}

pub struct Countdown {
    // number of seconds to countdown
    target_secs: usize,
    // count up instead of down
    up: bool,
}

impl Countdown {
    pub fn new(target_secs: usize, up: bool) -> Self {
        Self {
            target_secs: target_secs,
            up: up,
        }
    }

    // start the countdown
    pub async fn run(&self) {
        enable_raw_mode().unwrap();
        execute!(stdout(), crossterm::terminal::EnterAlternateScreen).unwrap();
        execute!(stdout(), crossterm::cursor::Hide).unwrap();

        let mut timer: Pin<Box<dyn Stream<Item = tokio::time::Instant>>> = Box::pin(
            IntervalStream::new(tokio::time::interval(std::time::Duration::from_secs(1))),
        );
        let mut events = EventStream::new().fuse();

        let mut paused = false; // track if timer is currently paused

        let mut remaining = self.target_secs;

        self.draw(remaining);

        loop {
            futures::select! {
                _ = timer.next().fuse() => {
                        self.draw(remaining);
                        if remaining == 0 {
                            break;
                        }
                        remaining -= 1;
                }
                e = events.next() => {
                    match e {

                    Some(Ok(crossterm::event::Event::Resize(_, _))) => {
                        self.draw(remaining)
                    }

                    Some(Ok(crossterm::event::Event::Key(KeyEvent { code, modifiers, ..}))) => {
                        match (code ,modifiers ) {
                            (KeyCode::Char('q'), KeyModifiers::NONE)
                            | (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                                break;
                            }
                            (KeyCode::Char('p'), KeyModifiers::NONE) => {
                                if !paused {
                                    // pause the timer. This always returns pending for timer
                                    timer = Box::pin(futures::stream::pending().fuse());
                                }
                                paused = true;
                            }
                            (KeyCode::Char('c'), KeyModifiers::NONE) => {
                                if paused {
                                    timer = Box::pin(IntervalStream::new(tokio::time::interval(std::time::Duration::from_secs(1))).fuse()); // resume the timer
                                }
                                paused = false;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                    }
                }
            }
        }

        disable_raw_mode().unwrap();
        execute!(stdout(), crossterm::cursor::Show).unwrap();
        execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
    }

    fn draw(&self, remaining: usize) {
        execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
        .unwrap();

        let (w, h) = crossterm::terminal::size().unwrap();

        let value = if self.up {
            self.target_secs - remaining
        } else {
            remaining
        };

        let text = format(value);
        let text = to_text(text);

        let start_x = w / 2 - text.width() as u16 / 2;
        let start_y = h / 2 - text.height() as u16 / 2;

        let mut start = start_x;
        for s in text.0.iter() {
            for (i, line) in s.0.iter().enumerate() {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(start, start_y + i as u16),
                    crossterm::style::Print(line)
                )
                .unwrap();
            }
            start += s.0[0].chars().count() as u16;
        }
    }
}

// Where s in duration in seconds.
fn format(s: usize) -> String {
    let mut s = s;
    let h = s / SECONDS_IN_HOUR;
    s = s % SECONDS_IN_HOUR;
    let m = s / SECONDS_IN_MINUTE;
    s = s % SECONDS_IN_MINUTE;

    if h > 0 {
        return format!("{}:{:02}:{:02}", h, m, s);
    }

    return format!("{:02}:{:02}", m, s);
}

fn to_text(s: String) -> Text {
    let mut symbols = Vec::new();
    for c in s.chars() {
        if let Some(symbol) = font::KEYWORDS.get(&c) {
            symbols.push(Symbol(symbol.clone()));
        }
    }
    Text(symbols)
}
