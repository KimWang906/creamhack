mod custom_widgets;
/// Module: dreamhack
///
/// This module is for handling Dreamhack API.
mod dreamhack;
mod fs_tree;
mod render;
mod termui;

use color_eyre::Result;
use dialoguer::Input;
use keyring::Entry;
#[cfg(debug_assertions)]
use log::LevelFilter;
#[cfg(debug_assertions)]
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use rpassword::prompt_password;
use termui::App;

fn main() -> Result<()> {
    // Logger initialization(For debugging)
    #[cfg(debug_assertions)]
    {
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build("log/debug.log")?;

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

        log4rs::init_config(config)?;

        log::info!("Logger initialized");
    }

    // TODO: Fix keyring
    // Create separate entries for email and password
    let email_entry = Entry::new("DreamhackService", "dreamhack_email").unwrap();
    let password_entry = Entry::new("DreamhackService", "dreamhack_password").unwrap();

    let mut email: String = String::new();
    let mut password: String = String::new();

    if email_entry.get_secret().is_err() && password_entry.get_password().is_err() {
        email = Input::new().with_prompt("Email").interact().unwrap();
        password = prompt_password("Password: ").unwrap();

        // Save email and password
        match email_entry.set_secret(email.as_bytes()) {
            Ok(()) => println!("Successfully set email to '{email}'"),
            Err(err) => {
                #[cfg(debug_assertions)]
                log::error!("Error setting email: {err}")
            }
        }
        match password_entry.set_password(&password) {
            Ok(()) => println!("Successfully set password"),
            Err(err) => {
                #[cfg(debug_assertions)]
                log::error!("Error setting password: {err}")
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        let email_info = String::from_utf8_lossy(&email_entry.get_secret().unwrap()).into_owned();
        log::info!("email: {}", email_info);
        assert_eq!(email_info, email);
        assert_eq!(password_entry.get_password().unwrap(), password);
    }

    // TUI initialization
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal, email_entry, password_entry);
    ratatui::restore();
    app_result
}
