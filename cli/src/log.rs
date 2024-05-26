use std::io;
use colored::Colorize;
use spinoff::{Color, Spinner, spinners};
use spinoff::spinners::Dots;
macro_rules! info {
    ($($t:tt)*) => {{
        println!($($t)*);
    }};
}

macro_rules! error {
    ($($t:tt)*) => {{
        use colored::Colorize;

        let formatted = format!($($t)*);
        println!("{}{}", "error: ".red().bold(), formatted);
    }};
}

pub struct Loading {
    spinner: Option<Spinner>
}

impl Loading {
    pub fn start(what: &str) -> Self {
        if atty::is(atty::Stream::Stdout) {
            
            let clone = what.to_string();
            
            // only show spinner when tty
            Self { spinner: Some(
                Spinner::new(spinners::Dots, clone, Color::White)
            )}
            
        } else {
            Self { spinner: None }
        }
    }
    
    pub fn next(&mut self, what: &str) {
        if let Some(spinner) = &mut self.spinner {
            let clone = what.to_string();
            spinner.update_text(clone);
        }
    }

    pub fn succeed(self, what: &str) {
        if let Some(mut spinner) = self.spinner {
            spinner.success(what)
        }
    }

    pub fn fail(self, what: &str) {
        if let Some(mut spinner) = self.spinner {
            spinner.fail(what)
        } else {
            eprintln!("error: {what}");
        }
    }
    
    pub fn success(what: &str) {
        if atty::is(atty::Stream::Stdout) {
            println!("{} {}", "✓".green().bold(), what);
        }
    }

    pub fn failure(what: &str) {
        if atty::is(atty::Stream::Stdout) {
            println!("{} {}", "✗".red().bold(), what);
        } else {
            eprintln!("error: {}", what)
        }
    }
}
