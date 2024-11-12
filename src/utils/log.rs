use fern::colors::{Color, ColoredLevelConfig};


pub fn setup_logger() -> Result<(), fern::InitError>
{
    let colors: ColoredLevelConfig = ColoredLevelConfig::new()
        // use builder methods
        .info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record|
        {
            out.finish(format_args!(
                "tape {} {}",
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}