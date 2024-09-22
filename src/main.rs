mod config;
mod custom_widgets;
/// Module: dreamhack
///
/// This module is for handling Dreamhack API.
mod dreamhack;
mod fs_tree;
mod render;
mod termui;

use color_eyre::Result;
use config::Config;
use dialoguer::Input;
use keyring::Entry;
#[cfg(debug_assertions)]
use log::LevelFilter;
#[cfg(debug_assertions)]
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
};
use rpassword::prompt_password;
use termui::App;

fn main() -> Result<()> {
    // Logger initialization (For debugging)
    #[cfg(debug_assertions)]
    {
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build("log/debug.log")?;

        let config = log4rs::Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

        log4rs::init_config(config)?;

        log::info!("Logger initialized");
    }

    let config = Config::read_or_new_config();

    // Create separate entries for email and password
    let email_entry = Entry::new("DreamhackService", "dreamhack_email").unwrap();
    let password_entry = Entry::new("DreamhackService", "dreamhack_password").unwrap();

    let email: String;
    let password: String;

    // Check if email and password are already stored
    let email_result = email_entry.get_secret();
    let password_result = password_entry.get_password();

    if email_result.is_err() || password_result.is_err() {
        email = Input::new().with_prompt("Email").interact().unwrap();
        password = prompt_password("Password: ").unwrap();

        // Save email and password
        match email_entry.set_secret(email.as_bytes()) {
            Ok(()) => println!("Successfully set email"),
            #[allow(unused_variables)]
            Err(err) => {
                #[cfg(debug_assertions)]
                log::error!("Error setting email: {err}")
            }
        }
        match password_entry.set_password(&password) {
            Ok(()) => println!("Successfully set password"),
            #[allow(unused_variables)]
            Err(err) => {
                #[cfg(debug_assertions)]
                log::error!("Error setting password: {err}")
            }
        }
    } else {
        email = String::from_utf8_lossy(&email_result.unwrap()).into_owned();
        #[cfg(debug_assertions)]
        {
            log::info!("Retrieved email: {}", email);
            log::info!("Retrieved password: (hidden)");
        }
    }

    println!("Logged in with email: {}", email);
    println!("Password is securely stored and retrieved.");

    // TUI initialization
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal, config, email_entry, password_entry);
    ratatui::restore();
    app_result
}
