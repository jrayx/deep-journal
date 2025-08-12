pub mod commands;
pub mod entities;
// pub mod error_handler;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  
  // build Tauri app
  tauri::Builder::default()
    
    // set up Tauri app builder
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    
    // attach custom commands
    .invoke_handler(tauri::generate_handler![
        // commands::hello_world,
        // commands::read_file,
        commands::setup_database,
        commands::run_llm,
        commands::get_models,
        commands::create_model,
        commands::delete_model,
        commands::get_chats,
        commands::create_chat,
        commands::update_chat_title,
        commands::delete_chat,
        commands::get_messages_by_chat,
        commands::create_message,
    ])
    
    // run Tauri app
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
