use rainbow_text as rainbow;
pub fn show_help() -> std::io::Result<()> {
    let rain = rainbow::Rainbow::default();

    rain.write("┌───────┐  ┌────┐  ┌────╮  ┌────┐\n")?;
    rain.write("└──┐ ┌──┘  │    │  │    │  │    │\n")?;
    rain.write("   │ │     │    │  │    │  │    │\n")?;
    rain.write("   └─┘     └────┘  └────╯  └────┘\n")?;
    Ok(())
}