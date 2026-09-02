use tauri::Manager;

#[derive(serde::Serialize)]
struct Activity {
  id: i64,
  start_time: String,
  end_time: String,
  description: String,
  productivity: String,
}

#[derive(serde::Serialize)]
struct ProductivityTotal {
  productivity: String,
  count: i64,
}

#[tauri::command]
fn productivity_totals(app: tauri::AppHandle) -> Result<Vec<ProductivityTotal>, String> {
  use rusqlite::Connection;
  let db = Connection::open(app.path().app_data_dir().map_err(|e| e.to_string())?.join("timepulse.sqlite")).map_err(|e| e.to_string())?;
  let mut query = db.prepare("SELECT productivity, COUNT(*) FROM activities GROUP BY productivity").map_err(|e| e.to_string())?;
  let rows = query.query_map([], |row| Ok(ProductivityTotal { productivity: row.get(0)?, count: row.get(1)? })).map_err(|e| e.to_string())?;
  rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn productivity_totals_for_period(app: tauri::AppHandle, period: String) -> Result<Vec<ProductivityTotal>, String> {
  use rusqlite::Connection;
  let db = Connection::open(app.path().app_data_dir().map_err(|e| e.to_string())?.join("timepulse.sqlite")).map_err(|e| e.to_string())?;
  let modifier = match period.as_str() { "week" => "-6 days", "month" => "-1 month", _ => "0 days" };
  let sql = format!("SELECT productivity, COUNT(*) FROM activities WHERE start_time >= datetime('now', '{modifier}') GROUP BY productivity");
  let mut query = db.prepare(&sql).map_err(|e| e.to_string())?;
  let rows = query.query_map([], |row| Ok(ProductivityTotal { productivity: row.get(0)?, count: row.get(1)? })).map_err(|e| e.to_string())?;
  rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn list_activities(app: tauri::AppHandle) -> Result<Vec<Activity>, String> {
  use rusqlite::Connection;
  let db = Connection::open(app.path().app_data_dir().map_err(|e| e.to_string())?.join("timepulse.sqlite")).map_err(|e| e.to_string())?;
  let mut query = db.prepare("SELECT id, start_time, end_time, description, productivity FROM activities ORDER BY start_time DESC LIMIT 50").map_err(|e| e.to_string())?;
  let rows = query.query_map([], |row| Ok(Activity { id: row.get(0)?, start_time: row.get(1)?, end_time: row.get(2)?, description: row.get(3)?, productivity: row.get(4)? })).map_err(|e| e.to_string())?;
  rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

#[tauri::command]
fn export_activities(app: tauri::AppHandle, format: String) -> Result<String, String> {
  let activities = list_activities(app.clone())?;
  let (extension, content) = match format.as_str() {
    "json" => ("json", serde_json::to_string_pretty(&activities).map_err(|e| e.to_string())?),
    "csv" => {
      let mut csv = String::from("id,start_time,end_time,description,productivity\n");
      for item in activities {
        csv.push_str(&format!("{},{},{},\"{}\",{}\n", item.id, item.start_time, item.end_time, item.description.replace('"', "\"\""), item.productivity));
      }
      ("csv", csv)
    }
    _ => return Err("format must be csv or json".into()),
  };
  let path = app.path().app_data_dir().map_err(|e| e.to_string())?.join(format!("activities.{extension}"));
  std::fs::write(&path, content).map_err(|e| e.to_string())?;
  Ok(path.to_string_lossy().into_owned())
}

#[tauri::command]
fn save_activity(app: tauri::AppHandle, description: String, productivity: String) -> Result<(), String> {
  use rusqlite::Connection;
  let dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
  let db = Connection::open(dir.join("timepulse.sqlite")).map_err(|e| e.to_string())?;
  let now = chrono::Utc::now().to_rfc3339();
  db.execute("INSERT INTO activities (start_time, end_time, description, productivity, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)", rusqlite::params![now, now, description, productivity, now]).map_err(|e| e.to_string())?;
  Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_autostart::init(
      tauri_plugin_autostart::MacosLauncher::LaunchAgent,
      Some(vec!["--minimized"]),
    ))
    .invoke_handler(tauri::generate_handler![save_activity, list_activities, productivity_totals, productivity_totals_for_period, export_activities])
    .setup(|app| {
      use rusqlite::Connection;
      use std::fs;
      let data_dir = app.path().app_data_dir()?;
      fs::create_dir_all(&data_dir)?;
      let db = Connection::open(data_dir.join("timepulse.sqlite"))?;
      db.execute_batch(
        "CREATE TABLE IF NOT EXISTS projects (id INTEGER PRIMARY KEY, name TEXT NOT NULL, color TEXT NOT NULL DEFAULT '#5b9fd2', archived INTEGER NOT NULL DEFAULT 0);
         CREATE TABLE IF NOT EXISTS categories (id INTEGER PRIMARY KEY, name TEXT NOT NULL, color TEXT NOT NULL DEFAULT '#97a1ac');
         CREATE TABLE IF NOT EXISTS activities (id INTEGER PRIMARY KEY, start_time TEXT NOT NULL, end_time TEXT NOT NULL, description TEXT NOT NULL, project_id INTEGER, category_id INTEGER, productivity TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL, FOREIGN KEY(project_id) REFERENCES projects(id), FOREIGN KEY(category_id) REFERENCES categories(id));
         CREATE TABLE IF NOT EXISTS settings (id INTEGER PRIMARY KEY CHECK (id = 1), reminder_interval INTEGER NOT NULL DEFAULT 30, reminders_enabled INTEGER NOT NULL DEFAULT 1, launch_at_startup INTEGER NOT NULL DEFAULT 0, start_minimized INTEGER NOT NULL DEFAULT 1, pop_sound_enabled INTEGER NOT NULL DEFAULT 1);
         CREATE INDEX IF NOT EXISTS idx_activities_start_time ON activities(start_time);
         INSERT OR IGNORE INTO settings (id) VALUES (1);
         INSERT OR IGNORE INTO projects (name, color) VALUES ('TimePulse', '#8369d7'), ('Personnel', '#5b9fd2');
         INSERT OR IGNORE INTO categories (name, color) VALUES ('Travail', '#36b37e'), ('Pause', '#f1a33c'), ('Distraction', '#e56b73');",
      )?;
      use tauri::menu::{MenuBuilder, MenuItemBuilder};
      use tauri::tray::{TrayIconBuilder, TrayIconEvent};
      use tauri::{Manager, WindowEvent};

      let open = MenuItemBuilder::with_id("open", "Ouvrir TimePulse").build(app)?;
      let add = MenuItemBuilder::with_id("add", "Ajouter une activité").build(app)?;
      let quit = MenuItemBuilder::with_id("quit", "Quitter TimePulse").build(app)?;
      let menu = MenuBuilder::new(app).items(&[&open, &add, &quit]).build()?;
      let main_window = app.get_webview_window("main").expect("main window exists");
      TrayIconBuilder::new()
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_tray_icon_event(|tray, event| {
          if let TrayIconEvent::DoubleClick { .. } = event {
            if let Some(window) = tray.app_handle().get_webview_window("main") {
              let _ = window.show();
              let _ = window.set_focus();
            }
          }
        })
        .on_menu_event(move |app, event| match event.id().as_ref() {
          "open" | "add" => {
            if let Some(window) = app.get_webview_window("main") {
              let _ = window.show();
              let _ = window.set_focus();
            }
          }
          "quit" => app.exit(0),
          _ => {}
        })
        .icon(app.default_window_icon().unwrap().clone())
        .build(app)?;
      let _ = main_window.hide();

      if let Some(window) = app.get_webview_window("main") {
        let window_for_events = window.clone();
        window.on_window_event(move |event| {
          if let WindowEvent::CloseRequested { api, .. } = event {
            api.prevent_close();
            let _ = window_for_events.hide();
          }
        });
      }
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
