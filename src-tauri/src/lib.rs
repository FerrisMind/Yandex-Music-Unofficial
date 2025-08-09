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
