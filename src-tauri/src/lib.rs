use tauri::menu::{AboutMetadata, SubmenuBuilder};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::Manager;

mod cmd;
mod db;
mod models;
mod schema;

mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let settings = MenuItemBuilder::new("Settings...")
                .id("settings")
                .accelerator("CmdOrCtrl+,")
                .build(app)?;

            let app_submenu = SubmenuBuilder::new(app, "App")
                .about(Some(AboutMetadata {
                    ..Default::default()
                }))
                .separator()
                .item(&settings)
                .separator()
                .services()
                .separator()
                .hide()
                .hide_others()
                .quit()
                .build()?;
            let quit = MenuItemBuilder::new("Quit").id("quit").build(app).unwrap();

            let hide = MenuItemBuilder::new("Hide").id("hide").build(app).unwrap();
            let show = MenuItemBuilder::new("Show").id("show").build(app).unwrap();

            let menu = MenuBuilder::new(app)
                .items(&[&app_submenu, &quit, &hide, &show])
                .build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app, event| match event.id().as_ref() {
                // if event.id() == settings.id() {
                //     // emit a window event to the frontend
                //     let _event = app.emit("custom-event", "/about");
                // }
                "quit" => app.exit(0),
                "hide" => {
                    dbg!("menu item hide clicked");
                    let window = app.get_webview_window("main").unwrap();
                    window.hide().unwrap();
                }
                "show" => {
                    dbg!("menu item show clicked");
                    let window = app.get_webview_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            cmd::create_collection,
            cmd::get_collections,
            cmd::create_request,
            cmd::get_request,
            cmd::save_request,
            cmd::send_request,
            cmd::toggle_collection,
            cmd::rename_collection,
            cmd::rename_request,
            cmd::get_tabs,
            cmd::close_tab
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
