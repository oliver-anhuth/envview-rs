slint::include_modules!();

use slint::{ModelRc, SharedString, StandardListViewItem, VecModel};
use std::env;

fn main() -> Result<(), slint::PlatformError> {
    let mut vars: Vec<(String, String)> = env::vars().collect();
    vars.sort_by(|a, b| a.0.cmp(&b.0));

    let ui = AppWindow::new()?;

    let rows: Vec<ModelRc<StandardListViewItem>> = vars
        .iter()
        .map(|(name, value)| {
            ModelRc::new(VecModel::from(vec![
                StandardListViewItem::from(SharedString::from(name.as_str())),
                StandardListViewItem::from(SharedString::from(value.as_str())),
            ]))
        })
        .collect();
    ui.set_rows(ModelRc::new(VecModel::from(rows)));

    {
        let vars = vars.clone();
        let weak = ui.as_weak();
        ui.on_request_copy(move |row, field| {
            let Some(ui) = weak.upgrade() else { return };
            if let Some((name, value)) = vars.get(row as usize) {
                let text = match field {
                    0 => name.clone(),
                    1 => value.clone(),
                    _ => format!("{name}={value}"),
                };
                ui.invoke_copy(text.into());
            }
        });
    }

    ui.on_copy(|text| {
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            let _ = clipboard.set_text(text.to_string());
        }
    });

    ui.run()
}
