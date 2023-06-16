use anyhow::{Context, Result};

/// Logger configuration using [`fern`]
pub fn setup() -> Result<()> {
    let base_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or("/".to_owned());
    let current_datetime = chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]");
    let file_name = format!("{base_dir}/logs/{current_datetime}.log");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} {:<30}  |  {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                format!("[{}][{}]", record.target(), record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file(file_name).context("Unable to open log file")?)
        .apply()
        .context("Failed to dispatch logger")?;
    log::info!("Configured fern logger");

    Ok(())
}
