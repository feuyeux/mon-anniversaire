use chrono::{Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Anniversary {
    id: Option<u32>,
    name: String,
    date: String, // ISO format string (YYYY-MM-DD)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Anniversaries {
    items: Vec<Anniversary>,
    next_id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnniversaryWithDays {
    id: u32,
    name: String,
    date: String,
    days: i64,
}

fn get_anniversaries_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    Ok(app_dir.join("anniversaries.json"))
}

fn save_anniversaries_to_disk(
    app_handle: &AppHandle,
    anniversaries: &Anniversaries,
) -> Result<(), String> {
    let path = get_anniversaries_path(app_handle)?;
    let json = serde_json::to_string(&anniversaries).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

fn load_anniversaries(app_handle: &AppHandle) -> Result<Anniversaries, String> {
    let anniversaries_path = get_anniversaries_path(app_handle)?;

    if !anniversaries_path.exists() {
        return Ok(Anniversaries {
            items: vec![],
            next_id: 1,
        });
    }

    let data = fs::read_to_string(anniversaries_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&data).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_anniversary(app_handle: AppHandle, name: String, date: String) -> Result<(), String> {
    let mut anniversaries = load_anniversaries(&app_handle).unwrap_or(Anniversaries {
        items: vec![],
        next_id: 1,
    });

    let new_id = anniversaries.next_id;
    anniversaries.items.push(Anniversary {
        id: Some(new_id),
        name,
        date,
    });

    anniversaries.next_id += 1;
    save_anniversaries_to_disk(&app_handle, &anniversaries)
}

#[tauri::command]
fn update_anniversary(
    app_handle: AppHandle,
    id: u32,
    name: String,
    date: String,
) -> Result<(), String> {
    let mut anniversaries = load_anniversaries(&app_handle)?;
    let anniversary_index = anniversaries.items.iter().position(|a| a.id == Some(id));

    match anniversary_index {
        Some(index) => {
            anniversaries.items[index].name = name;
            anniversaries.items[index].date = date;
            save_anniversaries_to_disk(&app_handle, &anniversaries)
        }
        None => Err(format!("Anniversary with ID {} not found", id)),
    }
}

#[tauri::command]
fn get_anniversaries(app_handle: AppHandle) -> Result<Vec<AnniversaryWithDays>, String> {
    let anniversaries = load_anniversaries(&app_handle).unwrap_or(Anniversaries {
        items: vec![],
        next_id: 1,
    });

    let today = Utc::now().date_naive();
    let mut result = Vec::new();

    for anniversary in anniversaries.items {
        let date =
            NaiveDate::parse_from_str(&anniversary.date, "%Y-%m-%d").map_err(|e| e.to_string())?;

        let days = (date - today).num_days();

        result.push(AnniversaryWithDays {
            id: anniversary.id.unwrap_or(0),
            name: anniversary.name,
            date: anniversary.date,
            days,
        });
    }

    Ok(result)
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

#[tauri::command]
fn delete_anniversary(app_handle: AppHandle, id: u32) -> Result<(), String> {
    let mut anniversaries = load_anniversaries(&app_handle)?;
    let index = anniversaries.items.iter().position(|a| a.id == Some(id));

    match index {
        Some(idx) => {
            anniversaries.items.remove(idx);
            save_anniversaries_to_disk(&app_handle, &anniversaries)
        }
        None => Err(format!("Anniversary with ID {} not found", id)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            save_anniversary,
            get_anniversaries,
            calculate_date,
            delete_anniversary,
            update_anniversary
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
