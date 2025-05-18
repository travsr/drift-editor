use anyhow::Result;
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    window::{Effect, EffectState, EffectsBuilder},
    AppHandle, PhysicalPosition, TitleBarStyle, WebviewUrl, WebviewWindowBuilder,
};

use crate::{models::d_app_state::DAppState, traits::app_control_layer::AppControlLayer};

pub struct TauriAppControlLayer {
    app_handle: AppHandle,
}

impl TauriAppControlLayer {
    pub fn new(app_handle: AppHandle) -> Self {
        TauriAppControlLayer { app_handle }
    }
}

impl AppControlLayer for TauriAppControlLayer {
    fn init_frontend(&self, app_state: &DAppState) -> Result<()> {
        let effects = EffectsBuilder::new()
            .effects(vec![Effect::Menu, Effect::Mica])
            .radius(10.0)
            .state(EffectState::Active)
            .build();

        WebviewWindowBuilder::new(
            &self.app_handle,
            "test-label",
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
    fn emit_event(&self, window_id: &str, tab_id: &str) -> Result<()> {
        Ok(())
    }
}
