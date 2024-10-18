use std::{fmt, iter};
use std::error::Error;
use std::fmt::Formatter;

use eyre::EyreHandler;
use stblib::colors::{BOLD, C_RESET, LIGHT_RED};

use crate::STRINGS;

pub fn install() -> Result<(), impl Error> {
    eyre::set_hook(Box::new(move |_| Box::new(Handler)))
}

struct Handler;

impl EyreHandler for Handler {
    fn debug(&self, error: &(dyn Error + 'static), f: &mut Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return fmt::Debug::fmt(error, f);
        }

        let mut errors = iter::successors(Some(error), |error| (*error).source());

        writeln!(f, "{LIGHT_RED}{BOLD}{}{C_RESET}", errors.next().unwrap())?;

        writeln!(f, "{}:", STRINGS.load("Cause"))?;

        for e in errors {
            writeln!(f, "- {e}")?;
        }

        Ok(())
    }
}