mod imp;
mod palette;

use mottle::theme::{ThemeBuilder, Type};
use std::io;

fn main() -> io::Result<()> {
    build_theme(&palette::Palette::default(), Type::Dark, "lc")?;
    build_theme(&palette::Palette::chroma(), Type::Dark, "lc chroma")?;
    build_theme(&palette::Palette::soft(), Type::Dark, "lc soft")?;
    build_theme(
        &palette::Palette::soft_chroma(),
        Type::Dark,
        "lc soft chroma",
    )?;

    build_theme(&palette::Palette::light(), Type::Light, "lc light")?;
    build_theme(
        &palette::Palette::light_chroma(),
        Type::Light,
        "lc light chroma",
    )?;
    build_theme(
        &palette::Palette::light_soft(),
        Type::Light,
        "lc light soft",
    )?;
    build_theme(
        &palette::Palette::light_soft_chroma(),
        Type::Light,
        "lc light soft chroma",
    )?;

    Ok(())
}

fn build_theme(palette: &palette::Palette, ty: Type, name: &str) -> io::Result<()> {
    let mut builder = ThemeBuilder::new(name.to_string(), ty);
    imp::add_rules(&mut builder, palette);
    builder.build().save()?;

    Ok(())
}
