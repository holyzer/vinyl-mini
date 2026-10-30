#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lofty::prelude::*;
use std::{fs, path::{Path, PathBuf}};
use tauri::Manager;

fn state_path(app: &tauri::AppHandle) -> PathBuf {
    let d = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
    let _ = fs::create_dir_all(&d);
    d.join("state.json")
}
#[tauri::command]
fn load_state(app: tauri::AppHandle) -> String { fs::read_to_string(state_path(&app)).unwrap_or_default() }
#[tauri::command]
fn save_state(app: tauri::AppHandle, json: String) { let _ = fs::write(state_path(&app), json); }

fn walk(dir: &Path, out: &mut Vec<String>) {
    let Ok(rd) = fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() { walk(&p, out); }
        else if let Some(x) = p.extension().and_then(|s| s.to_str()) {
            if matches!(x.to_lowercase().as_str(), "mp3" | "flac" | "wav" | "ogg" | "m4a") {
                out.push(p.to_string_lossy().into_owned());
            }
        }
    }
}
#[tauri::command]
fn scan(folders: Vec<String>) -> Vec<String> {
    let mut out = vec![];
    for f in folders { walk(Path::new(&f), &mut out); }
    out.sort(); out.dedup(); out
}
#[tauri::command]
fn meta(path: String) -> String {
    let stem = Path::new(&path).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
    if let Ok(f) = lofty::read_from_path(&path) {
        if let Some(t) = f.primary_tag().or_else(|| f.first_tag()) {
            if let Some(title) = t.title() {
                return match t.artist() { Some(a) => format!("{} – {}", a, title), None => title.to_string() };
            }
        }
    }
    stem
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![load_state, save_state, scan, meta])
        .run(tauri::generate_context!())
        .expect("error while running app");
}
