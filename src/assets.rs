use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
pub struct Assets;

#[derive(Embed)]
#[folder = "locales/web/"]
pub struct Locales;
