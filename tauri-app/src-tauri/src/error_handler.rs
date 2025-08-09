// create the error type that represents all errors possible in our program
// #[derive(Debug, thiserror::Error)]
// pub enum Error {
//   #[error(transparent)]
//   Io(#[from] std::io::Error)
// }

// // we must manually implement serde::Serialize
// impl serde::Serialize for Error {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: serde::ser::Serializer,
//   {
//     serializer.serialize_str(self.to_string().as_ref())
//   }
// }

// example usage (return either OK or Error)
// #[tauri::command]
// fn my_custom_command() -> Result<(), Error> {
//   // This will return an error
//   std::fs::File::open("path/that/does/not/exist")?;
//   // Return `null` on success
//   Ok(())
// }