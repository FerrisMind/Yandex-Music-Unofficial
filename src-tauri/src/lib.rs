use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            
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
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
