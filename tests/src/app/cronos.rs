use clap::Command;

pub fn cronos() -> Command<'static> {
    Command::new("Cronos")
        .bin_name("cronos")
        .about("Cronos is an instruction scheduler for Solana")
        .version(version!())
        .subcommand_required(true)
        .subcommand(super::admin::app())
        .subcommand(super::bench::app())
}