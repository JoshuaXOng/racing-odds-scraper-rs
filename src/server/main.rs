use clap::{Parser};
mod scrapers; 
mod cli;
use crate::cli::args::Args;
use crate::cli::handlers::root_command_handler::root_command_handler;

fn main() {
  root_command_handler(Args::parse());
}
