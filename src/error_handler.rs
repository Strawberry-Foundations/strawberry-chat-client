use std::{fmt, iter};
use std::error::Error;
use std::fmt::Formatter;

use eyre::EyreHandler;
use owo_colors::OwoColorize;

use crate::STRING_LOADER;

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

        writeln!(f, "{}", errors.next().unwrap().red())?;

        writeln!(f, "{}:", STRING_LOADER.str("Cause"))?;

        for e in errors {
            writeln!(f, "- {e}")?;
        }

        Ok(())
    }
}