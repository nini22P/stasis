use fltk::dialog;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use wv::{SizeHint, Webview};

use crate::assets::Assets;
use crate::utils::set_window_size;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Source {
    uri: String,
    name: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Config {
    builtin_sources: Vec<Source>,
    custom_sources: Vec<Source>,
    selected_uri: String,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct StoredConfig {
    pub custom_sources: Vec<Source>,
    pub selected_uri: String,
}

fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("stasis");
        fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    })
}

pub fn load_config() -> StoredConfig {
    if let Some(path) = get_config_path() {
        if let Ok(data) = fs::read_to_string(path) {
            return serde_json::from_str(&data).unwrap_or_default();
        }
    }
    StoredConfig::default()
}

fn save_config(config: &StoredConfig) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(path) = get_config_path() {
        let json = serde_json::to_string_pretty(config)?;
        println!("Saving config: {}", json);
        fs::write(path, json)?;
    }
    Ok(())
}

pub fn run(_is_modal: bool) {
    let mut wv = Webview::create_no_win(true);

    let hwnd = wv.get_window();

    set_window_size(hwnd, 640, 480, true, false);

    wv.set_size(640, 480, SizeHint::None).unwrap();
    wv.set_title(&"Stasis Configuration").unwrap();

    let wv_clone = wv.clone();

    wv.bind("ready", |_, _| {
        let builtin_sources: Vec<Source> = Assets::iter()
            .filter(|path| path.starts_with("screensavers/"))
            .filter_map(|path| path.split('/').nth(1).map(String::from))
            .collect::<std::collections::HashSet<String>>()
            .into_iter()
            .map(|name| Source {
                uri: format!("screensavers/{}/index.html", name),
                name,
            })
            .collect();

        let stored_config = load_config();

        let config = Config {
            builtin_sources,
            custom_sources: stored_config.custom_sources,
            selected_uri: stored_config.selected_uri,
        };

        if let Ok(config_json) = serde_json::to_string(&config) {
            wv_clone
                .eval(&format!("window.loadConfig({})", config_json))
                .unwrap();
        }
    })
    .unwrap();

    let mut wv_clone = wv.clone();

    wv.bind("setTitle", |_, content| {
        if let Ok(values) = serde_json::from_str::<Vec<String>>(content) {
            if let Some(title) = values.get(0) {
                wv_clone.set_title(&title).unwrap();
            }
        }
    })
    .unwrap();

    let wv_clone = wv.clone();

    wv.bind("choose", |seq, _| {
        let mut dialog = dialog::FileDialog::new(dialog::FileDialogType::BrowseFile);
        dialog.set_filter("*.{htm,html}");
        dialog.show();
        let filename = dialog.filename().to_string_lossy().to_string();

        let result = format!("\"{}\"", filename.replace('\\', "\\\\"));
        wv_clone.return_(seq, 0, &result).unwrap();
    })
    .unwrap();

    wv.bind("quit", |_, _| {
        std::process::exit(0);
    })
    .unwrap();

    wv.bind("save", |seq, content| {
        let mut status = "failed";

        if let Ok(values) = serde_json::from_str::<Vec<String>>(content) {
            if let Some(config_str) = values.get(0) {
                if let Ok(stored_config) = serde_json::from_str::<StoredConfig>(config_str) {
                    println!("Parsed StoredConfig: {:?}", stored_config);
                    if save_config(&stored_config).is_ok() {
                        status = "success";
                    }
                }
            }
        }

        wv.return_(seq, 0, &format!("\"{}\"", status)).unwrap();
    })
    .unwrap();

    if let Some(content) = Assets::get("config/index.html") {
        if let Ok(html_str) = std::str::from_utf8(&content.data) {
            let encoded_html = urlencoding::encode(html_str);
            let data_uri = format!("data:text/html;charset=utf-8,{}", encoded_html);
            wv.navigate(&data_uri).unwrap();
        }
    } else {
        eprintln!("Fatal: Bundled 'config.html' not found!");
    }

    wv.run().unwrap();
}
