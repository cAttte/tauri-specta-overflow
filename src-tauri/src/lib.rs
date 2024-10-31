#[derive(serde::Serialize, specta::Type)]
struct B { a: Option<Box<A>> }

#[derive(serde::Serialize, specta::Type)]
struct A { b: B }

#[tauri::command]
#[specta::specta]
fn test() -> A {
    A { b: B { a: None } }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![test])
        .export(specta_typescript::Typescript::default(), "../src/bindings.ts")
        .expect("Tauri Specta error.");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![test])
        .run(tauri::generate_context!())
        .expect("Tauri error.");
}
