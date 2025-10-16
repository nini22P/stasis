use fltk::dialog::{FileDialog, FileDialogType};
use fltk::enums::{Color, Event};
use fltk::{app, button::Button, input::Input, prelude::*, window::Window};
use fltk_theme::widget_schemes::fluent::colors::*;
use fltk_theme::widget_schemes::fluent::frames::*;
use fltk_theme::{SchemeType, WidgetScheme};
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Config {
    url: String,
}

fn get_config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("stasis");
        fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    })
}

pub fn save_url(url: &str) -> Result<(), std::io::Error> {
    if let Some(path) = get_config_path() {
        let config = Config {
            url: url.to_string(),
        };
        let json = serde_json::to_string_pretty(&config)?;
        fs::write(path, json)?;
    }
    Ok(())
}

pub fn load_url() -> Option<String> {
    if let Some(path) = get_config_path() {
        if let Ok(data) = fs::read_to_string(path) {
            let config: Result<Config, _> = serde_json::from_str(&data);
            return config.ok().map(|c| c.url).filter(|url| !url.is_empty());
        }
    }
    None
}

pub fn run(is_modal: bool) {
    let app = app::App::default();

    app::background(0xFC, 0xFC, 0xFC);
    app::background2(0xFF, 0xFF, 0xFF);
    app::foreground(0x00, 0x00, 0x00);
    app::set_color(
        Color::Selection,
        SELECTION_COLOR.0,
        SELECTION_COLOR.1,
        SELECTION_COLOR.2,
    );

    let theme = WidgetScheme::new(SchemeType::Fluent);
    theme.apply();

    let mut win = Window::default()
        .with_size(400, 96)
        .center_screen()
        .with_label(&t!("app-title"));

    if is_modal {
        win.make_modal(true);
    }

    let mut url_input = Input::new(48, 16, 336, 28, t!("url-label").as_ref());
    if let Some(url) = load_url() {
        url_input.set_value(&url);
    }

    let mut browse_btn = Button::new(70, 56, 120, 28, t!("browse-button").as_ref());
    let mut save_btn = Button::new(210, 56, 120, 28, t!("save-button").as_ref());

    for btn in [&mut save_btn, &mut browse_btn] {
        btn.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
        btn.set_down_frame(OS_DEFAULT_DEPRESSED_DOWN_BOX);
        btn.handle(|b, ev| match ev {
            Event::Enter => {
                b.set_frame(OS_HOVERED_UP_BOX);
                b.redraw();
                true
            }
            Event::Leave => {
                b.set_frame(OS_DEFAULT_BUTTON_UP_BOX);
                b.redraw();
                true
            }
            _ => false,
        });
    }

    win.end();
    win.show();

    let mut url_input_clone = url_input.clone();

    browse_btn.set_callback(move |_| {
        let mut dialog = FileDialog::new(FileDialogType::BrowseFile);
        dialog.set_title(&t!("browse-dialog-title"));
        dialog.set_filter("*.{htm,html}");
        dialog.show();

        let filename = dialog.filename();

        if !filename.to_string_lossy().is_empty() {
            url_input_clone.set_value(&filename.to_string_lossy());
        }
    });

    save_btn.set_callback(move |_| {
        let url = url_input.value();

        if let Err(e) = save_url(&url) {
            eprintln!("Error saving config: {}", e);
        }
        app.quit();
    });

    app.run().unwrap();
}
