pub(crate) use self::{
    alloc_console::AllocConsole,
    close_pseudo_console::{ClosePseudoConsole, ClosePseudoConsoleBuilder},
    create_pseudo_console::{CreatePseudoConsole, CreatePseudoConsoleBuilder},
    get_number_of_console_input_events::{
        GetNumberOfConsoleInputEvents, GetNumberOfConsoleInputEventsBuilder,
    },
};
use anyhow::Result;

pub fn alloc_console() -> Result<()> {
    AllocConsole()
}

pub fn close_pseudo_console() -> ClosePseudoConsoleBuilder<((),)> {
    ClosePseudoConsole::builder()
}

pub fn create_pseudo_console() -> CreatePseudoConsoleBuilder<((), (), (), ())> {
    CreatePseudoConsole::builder()
}

pub fn get_number_of_console_input_events<'a>() -> GetNumberOfConsoleInputEventsBuilder<'a, ((),)> {
    GetNumberOfConsoleInputEvents::builder()
}

enum Flags {
    Standard,
    Inherit,
}

mod alloc_console;
mod close_pseudo_console;
mod create_pseudo_console;
mod get_number_of_console_input_events;
