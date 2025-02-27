use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Anniversary {
    name: String,
    date: String, // Store as ISO format string (YYYY-MM-DD)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anniversaries {
    items: Vec<Anniversary>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn save_anniversary(app_handle: AppHandle, name: String, date: String) -> Result<(), String> {
    let mut anniversaries =
        load_anniversaries(&app_handle).unwrap_or(Anniversaries { items: vec![] });

    anniversaries.items.push(Anniversary { name, date });

    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let anniversaries_path = app_dir.join("anniversaries.json");
    let json = serde_json::to_string(&anniversaries).map_err(|e| e.to_string())?;

    fs::write(anniversaries_path, json).map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_anniversaries(app_handle: AppHandle) -> Result<Vec<AnniversaryWithDays>, String> {
    let anniversaries = load_anniversaries(&app_handle).unwrap_or(Anniversaries { items: vec![] });

    let today = Utc::now().date_naive();
    let mut result = Vec::new();

    for anniversary in anniversaries.items {
        let date =
            NaiveDate::parse_from_str(&anniversary.date, "%Y-%m-%d").map_err(|e| e.to_string())?;

        let days = (date - today).num_days();

        result.push(AnniversaryWithDays {
            name: anniversary.name,
            date: anniversary.date,
            days,
        });
    }

    Ok(result)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnniversaryWithDays {
    name: String,
    date: String,
    days: i64,
}

#[tauri::command]
fn calculate_date(days: i64, is_future: bool) -> Result<String, String> {
    let today = Utc::now().date_naive();
    let days_abs = days.abs();

    let calculated_date = if is_future {
        today + Duration::days(days_abs)
    } else {
        today - Duration::days(days_abs)
    };

    Ok(calculated_date.format("%Y-%m-%d").to_string())
}

fn load_anniversaries(app_handle: &AppHandle) -> Result<Anniversaries, String> {
    let app_dir: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let anniversaries_path = app_dir.join("anniversaries.json");

    if !anniversaries_path.exists() {
        return Ok(Anniversaries { items: vec![] });
    }

    let data = fs::read_to_string(anniversaries_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            save_anniversary,
            get_anniversaries,
            calculate_date
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
