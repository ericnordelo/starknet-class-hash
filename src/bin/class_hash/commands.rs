pub mod get;
pub use get::Get;

/// Common trait for Cli commands
pub trait CliCommand {
    fn run(&self) -> anyhow::Result<()>;
}
