pub mod commands;
pub mod db;
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
        commands::hello_world,
        commands::read_file,
        commands::run_llm,
        
        db::get_models,
        db::create_model,
        db::delete_model,
        db::get_chats,
        db::create_chat,
        db::update_chat_title,
        db::delete_chat,
        db::get_messages_by_chat,
        db::create_message,
    ])
    
    // run Tauri app
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
