use anyhow::{Context, Result};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
    window::{Effect, EffectState, EffectsBuilder},
    AppHandle, Emitter, EventTarget, PhysicalPosition, TitleBarStyle, WebviewUrl,
    WebviewWindowBuilder,
};

use crate::{
    models::{
        d_app_state::DAppState,
        d_window_event_payload::{
            DWindowEventPayloadAll, DWindowEventPayloadContent, DWindowEventPayloadFileList,
            DWindowEventPayloadTabs,
        },
        d_window_state::DWindowState,
        d_window_state_scope::DWindowStateScope,
    },
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
            .traffic_light_position(PhysicalPosition::new(32.0, 40.0))
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

        let item_open = MenuItemBuilder::new("Open Project")
            .accelerator("CmdOrCtrl+O")
            .build(&self.app_handle)?;

        let file_menu = SubmenuBuilder::new(&self.app_handle, "File")
            .item(&item_open)
            .quit()
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
    fn emit_window_state(
        &self,
        scope: DWindowStateScope,
        window_state: &DWindowState,
    ) -> Result<()> {
        let event_name = "window_state_update";
        let target = EventTarget::webview_window(window_state.id.to_owned());

        println!(
            "[window_state_update]: scope: {} window-id {}",
            scope, window_state.id
        );

        match scope {
            DWindowStateScope::All => {
                let payload = DWindowEventPayloadAll {
                    scope,
                    window_state: window_state.to_owned(),
                };
                self.app_handle
                    .emit_to(target, &event_name, payload)
                    .with_context(|| {
                        format!(
                            "emit_window_state: Failed to emit to window '{}'",
                            window_state.id
                        )
                    })?;
            }
            DWindowStateScope::FileList => {
                let payload = DWindowEventPayloadFileList {
                    scope,
                    file_list: window_state.file_list.to_owned(),
                };
                self.app_handle
                    .emit_to(target, &event_name, payload)
                    .with_context(|| {
                        format!(
                            "emit_window_state: Failed to emit to window '{}'",
                            window_state.id
                        )
                    })?;
            }
            DWindowStateScope::Tabs => {
                let payload = DWindowEventPayloadTabs {
                    scope,
                    tabs: window_state.tabs.to_owned(),
                };
                self.app_handle
                    .emit_to(target, &event_name, payload)
                    .with_context(|| {
                        format!(
                            "emit_window_state: Failed to emit to window '{}'",
                            window_state.id
                        )
                    })?;
            }
            DWindowStateScope::Content => {
                let payload = DWindowEventPayloadContent {
                    scope,
                    content: window_state.content.to_owned(),
                };
                self.app_handle
                    .emit_to(target, &event_name, payload)
                    .with_context(|| {
                        format!(
                            "emit_window_state: Failed to emit to window '{}'",
                            window_state.id
                        )
                    })?;
            }
        }

        Ok(())
    }
}
