use anyhow::{Context, Result};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    window::{Effect, EffectState, EffectsBuilder},
    AppHandle, Emitter, EventTarget, PhysicalPosition, TitleBarStyle, WebviewUrl,
    WebviewWindowBuilder,
};

use crate::{
    models::{d_app_state::DAppState, d_window_state::DWindowState},
    traits::app_control_layer::AppControlLayer,
};

pub struct TauriAppControlLayer {
    app_handle: AppHandle,
}

impl TauriAppControlLayer {
    pub fn new(app_handle: AppHandle) -> Self {
        TauriAppControlLayer { app_handle }
    }

    fn init_windows(&self, app_state: &DAppState) -> Result<()> {
        for (window_id, _window) in app_state.windows.clone() {
            let effects = EffectsBuilder::new()
                .effects(vec![Effect::Menu, Effect::Mica])
                .radius(10.0)
                .state(EffectState::Active)
                .build();

            WebviewWindowBuilder::new(
                &self.app_handle,
                window_id,
                WebviewUrl::App("index.html".into()),
            )
            .title("drift")
            .hidden_title(true)
            .inner_size(800.0, 600.0)
            .transparent(true)
            .decorations(true)
            .shadow(true)
            .effects(effects)
            .title_bar_style(TitleBarStyle::Overlay)
            .traffic_light_position(PhysicalPosition::new(36.0, 36.0))
            .build()?;
        }

        Ok(())
    }

    fn init_menus(&self) -> Result<()> {
        let item1 = MenuItemBuilder::new("My MenuItem")
            .accelerator("CmdOrCtrl+W")
            .build(&self.app_handle)?;

        let drift_menu = SubmenuBuilder::new(&self.app_handle, "Drift")
            .about(None)
            .separator()
            .quit_with_text("Quit Drift Editor")
            .build()?;

        let file_menu = SubmenuBuilder::new(&self.app_handle, "File")
            .text("open", "Open")
            .text("quit", "Quit")
            .item(&item1)
            .separator()
            .build()?;

        let edit_menu = SubmenuBuilder::new(&self.app_handle, "Edit")
            .undo()
            .redo()
            .separator()
            .cut()
            .copy()
            .paste()
            .separator()
            .build()?;

        let menu = MenuBuilder::new(&self.app_handle)
            .item(&drift_menu)
            .item(&file_menu)
            .item(&edit_menu)
            .build()?;

        self.app_handle.set_menu(menu)?;

        Ok(())
    }
}

impl AppControlLayer for TauriAppControlLayer {
    fn init_frontend(&self, app_state: &DAppState) -> Result<()> {
        self.init_menus()?;
        self.init_windows(app_state)?;
        Ok(())
    }

    // Emits all Window States to their corresponding windows
    fn emit_app_state(&self, app_state: &DAppState) -> Result<()> {
        println!("emit_app_state");
        for (window_id, window_state) in &app_state.windows {
            println!("Updating window: {}", window_id);
            if let Err(e) = self
                .app_handle
                .emit_to(window_id, "window_state_update", window_state)
            {
                println!("Failed to emit to window '{}': {}", window_id, e);
            }
        }
        Ok(())
    }

    // Emit a single Window State to its corresponding window
    fn emit_window_state(&self, window_state: &DWindowState) -> Result<()> {
        println!(
            "emit_window_state: Updating single window {}",
            window_state.id
        );
        self.app_handle
            .emit_to(
                EventTarget::webview_window(window_state.id.to_owned()),
                "window_state_update",
                window_state,
            )
            .with_context(|| {
                format!(
                    "emit_window_state: Failed to emit to window '{}'",
                    window_state.id
                )
            })?;

        Ok(())
    }
}
