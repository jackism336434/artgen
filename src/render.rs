use figlet_rs::FIGfont;

pub fn render_text(text: &str) -> Result<String, String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Err("text cannot be empty or whitespace".to_string());
    }

    let font = FIGfont::standard().map_err(|err| format!("failed to load standard FIGlet font: {err}"))?;
    let figure = font
        .convert(trimmed)
        .ok_or_else(|| "failed to render ASCII art for the provided text".to_string())?;

    Ok(figure.to_string())
}
