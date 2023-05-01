#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{
    api, CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem,
};
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    let inject_script = r#"
    var style = document.createElement('style');
    if (navigator.appVersion.includes('Mac')) {
      style.innerHTML = 'body { background-color: transparent !important; margin: 0; } .arrow { position: relative; padding: 12px 0 0 0; } .arrow:before { content: ""; height: 0; width: 0; border-width: 0 8px 12px 8px; border-style: solid; border-color: transparent transparent #2f2f2f transparent; position: absolute; top: 0px; left: 50%; transform: translateX(-50%); } body > div { background-color: #343541 !important; border-radius: 7px !important; overflow: hidden !important; } @media (prefers-color-scheme: light) { body > div { background-color: white !important; }}';
    } else {
      style.innerHTML = 'body { background-color: transparent !important; margin: 0; } .arrow { position: relative; padding: 0 0 12px 0; } .arrow:after { content: ""; height: 0; width: 0; border-width: 12px 8px 0 8px; border-style: solid; border-color: #2f2f2f transparent transparent transparent; position: absolute; bottom: 0px; left: 50%; transform: translateX(-50%); } body > div { background-color: #343541 !important; border-radius: 7px !important; overflow: hidden !important; } @media (prefers-color-scheme: light) { body > div { background-color: white !important; }}';
    }
    document.head.appendChild(style);
    document.body.classList.add('arrow');
    document.addEventListener("keydown", e => {
        if (window.location.href.includes("https://chat.openai.com/chat") && e.code == "Enter" && e.target.tagName == "TEXTAREA") {
            if (e.target.nextSibling.tagName == "BUTTON" && e.shiftKey == false) {
                e.preventDefault()
                e.target.nextSibling.click()
            }
        }
    })
    "#;

    let open = CustomMenuItem::new("open".to_string(), "Open").accelerator("Cmd+Shift+O");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit").accelerator("Cmd+Q");
    let api_mode = CustomMenuItem::new("api_mode".to_string(), "API Mode");
    let web_mode = CustomMenuItem::new("web_mode".to_string(), "Web Mode");
    let github = CustomMenuItem::new("github".to_string(), "View on Github");
    let twitter = CustomMenuItem::new("twitter".to_string(), "Author on Twitter");
    let tray_menu = SystemTrayMenu::new()
        .add_item(open)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(web_mode)
        .add_item(api_mode)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(github)
        .add_item(twitter);

    let tray = SystemTray::new().with_menu(tray_menu);

    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();

            let mut shortcut = app.global_shortcut_manager();
            shortcut
                .register("Cmd+Shift+O", move || {
                    if main_window.is_visible().unwrap() {
                        main_window.hide().unwrap();
                    } else {
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                })
                .unwrap_or_else(|err| println!("{:?}", err));

            let main_window = app.get_window("main").unwrap();
            main_window.show().unwrap();
            main_window.set_focus().unwrap();
            Ok(())
        })
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .system_tray(tray)
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            tauri::WindowEvent::Focused(is_focused) => {
                // detect click outside of the focused window and hide the app
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    let _ = window.move_window(Position::TrayCenter);

                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();

                        window
                            .eval(inject_script)
                            .map_err(|err| println!("{:?}", err))
                            .ok();
                    }
                    // app.get_window("main").unwrap().show().unwrap();
                    // app.get_window("main").unwrap().set_focus().unwrap();
                }
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    //println!("system tray received a right click");
                }
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    //println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "twitter" => {
                        api::shell::open(
                            &app.get_window("main").unwrap().shell_scope(),
                            "https://github.com/TESLA2402".to_string(),
                            None,
                        )
                        .unwrap();
                    }
                    "github" => {
                        api::shell::open(
                            &app.get_window("main").unwrap().shell_scope(),
                            "https://github.com/TESLA2402".to_string(),
                            None,
                        )
                        .unwrap();
                    }
                    "web_mode" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window.eval(&format!(
                            "window.location.replace('https://chat.openai.com/chat')"
                        ));
                    }
                    "api_mode" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    "open" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .run(context)
        .expect("error while running tauri application");
}