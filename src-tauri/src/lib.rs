use tauri::Manager;
use tray_icon::{TrayIcon, TrayIconBuilder, menu::{Menu, MenuItem, PredefinedMenuItem}};
use std::sync::Arc;
use tauri::AppHandle;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn show_window(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
fn hide_window(app_handle: AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        window.hide().unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, show_window, hide_window])
        .setup(|app| {
            let app_handle = app.handle();
            let window = app.get_webview_window("main").unwrap();
            
            // Создание меню для системного трея
            let quit_item = MenuItem::new("Выход", true, None);
            let show_item = MenuItem::new("Показать", true, None);
            let hide_item = MenuItem::new("Скрыть", true, None);
            
            let tray_menu = Menu::new();
            let tray_menu = tray_menu
                .append(&show_item)
                .append(&hide_item)
                .append(&PredefinedMenuItem::separator())
                .append(&quit_item);
            
            // Создание иконки в системном трее
            let tray_icon = TrayIconBuilder::new()
                .with_menu(Box::new(tray_menu))
                .with_tooltip("Яндекс Музыка (Unofficial)")
                .with_icon(tauri::Icon::File(std::path::PathBuf::from("icons/icon.png")))
                .build()
                .unwrap();
            
            let tray_icon = Arc::new(tray_icon);
            let tray_icon_clone = tray_icon.clone();
            
            // Обработка событий системного трея
            std::thread::spawn(move || {
                let tray_icon = tray_icon_clone;
                let receiver = tray_icon.event_receiver();
                
                for event in receiver {
                    match event {
                        tray_icon::TrayIconEvent::LeftClick { .. } => {
                            // Показать окно при левом клике
                            let _ = app_handle.emit_to("main", "show-window", ());
                        }
                        tray_icon::TrayIconEvent::RightClick { .. } => {
                            // Показать контекстное меню при правом клике
                        }
                        tray_icon::TrayIconEvent::MenuItemClick { id } => {
                            match id.as_str() {
                                "Показать" => {
                                    let _ = app_handle.emit_to("main", "show-window", ());
                                }
                                "Скрыть" => {
                                    let _ = app_handle.emit_to("main", "hide-window", ());
                                }
                                "Выход" => {
                                    std::process::exit(0);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            });
            
            // Обработка закрытия окна - скрыть в трей вместо закрытия
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = app_handle.emit_to("main", "hide-window", ());
                }
            });
            
            // Отключение контекстного меню через JavaScript
            window.eval("
                // Отключение контекстного меню на уровне документа
                document.addEventListener('contextmenu', function(event) {
                    event.preventDefault();
                    event.stopPropagation();
                    return false;
                }, true);
                
                // Отключение контекстного меню на уровне window
                window.addEventListener('contextmenu', function(event) {
                    event.preventDefault();
                    event.stopPropagation();
                    return false;
                }, true);
                
                // Отключение контекстного меню для всех элементов
                document.addEventListener('DOMContentLoaded', function() {
                    const allElements = document.querySelectorAll('*');
                    allElements.forEach(element => {
                        element.addEventListener('contextmenu', function(event) {
                            event.preventDefault();
                            event.stopPropagation();
                            return false;
                        }, true);
                    });
                });
                
                // Дополнительная защита через CSS
                const style = document.createElement('style');
                style.textContent = `
                    * {
                        -webkit-user-select: none;
                        -moz-user-select: none;
                        -ms-user-select: none;
                        user-select: none;
                    }
                `;
                document.head.appendChild(style);
            ").unwrap();
            
            // Глобальная политика для куки: path=/, Secure (на HTTPS), SameSite=Lax.
            // Также сохраняем маркер согласия на все куки в корневом пути.
            window
                .eval(r#"
                (function () {
                  function setCookie(name, value, options) {
                    var params = options || {};
                    var days = typeof params.days === 'number' ? params.days : 3650;
                    var sameSite = params.sameSite || 'Lax';
                    var path = params.path || '/';
                    var secure = params.secure !== false; // по умолчанию true на HTTPS
                    var expires = new Date(Date.now() + days * 864e5).toUTCString();
                    var cookie = name + '=' + encodeURIComponent(value) + '; expires=' + expires + '; path=' + path;
                    if (secure && location.protocol === 'https:') cookie += '; Secure';
                    if (sameSite) cookie += '; SameSite=' + sameSite;
                    document.cookie = cookie;
                  }

                  var cookieProp = Object.getOwnPropertyDescriptor(Document.prototype, 'cookie')
                    || Object.getOwnPropertyDescriptor(HTMLDocument.prototype, 'cookie');
                  if (cookieProp && cookieProp.set && cookieProp.get) {
                    Object.defineProperty(document, 'cookie', {
                      configurable: true,
                      enumerable: cookieProp.enumerable,
                      get: function () { return cookieProp.get.call(document); },
                      set: function (value) {
                        var v = String(value || '');
                        if (!/;\s*path=/i.test(v)) v += '; path=/';
                        if (location.protocol === 'https:' && !/;\s*secure/i.test(v)) v += '; Secure';
                        if (!/;\s*samesite=/i.test(v)) v += '; SameSite=Lax';
                        return cookieProp.set.call(document, v);
                      }
                    });
                  }

                  if (!/app_cookies_consent=/.test(document.cookie)) {
                    setCookie('app_cookies_consent', 'all', { days: 3650, path: '/', secure: true, sameSite: 'Lax' });
                  }
                })();
                "#)
                .unwrap();
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
