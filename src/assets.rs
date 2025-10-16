use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
#[prefix = "/"]
pub struct Assets;
