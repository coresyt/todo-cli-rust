use std::fs::{self, File};
use std::io::prelude::*;

/// ## File Management Utility
///
/// The `FileManagement` struct provides methods to read from and write to a file,
/// handling file creation and error reporting.
pub struct FileManagement {
    /// Name or path of the file to manage
    file_name: String,
}

/// ## Generic Result Wrapper
///
/// `ResultList` is an enum used to wrap return values from file operations.
/// It allows returning either a boolean flag (success/failure)
/// or a text message (typically content or an error).
pub enum ResultList {
    /// Boolean flag, used to indicate success or failure
    Flag(bool),

    /// Text value, often used for content or error messages
    Text(String),
}

impl FileManagement {
    /// Creates a new instance of `FileManagement` with the given file name.
    ///
    /// # Arguments
    /// - `file_name`: The name or path of the file to manage.
    ///
    /// # Example
    /// ```rust
    /// let manager = FileManagement::new("data.txt".to_string());
    /// ```
    pub fn new(file_name: String) -> Self {
        Self {
            file_name: file_name
        }
    }

    /// Writes raw byte data to the file.
    ///
    /// # Arguments
    /// - `data`: A byte slice (`&[u8]`) containing the data to write.
    ///
    /// # Returns
    /// - `Ok(true)` if the write was successful.
    /// - `Err([ResultList; 2])` containing a flag and an error message if it fails.
    ///
    /// # Example
    /// ```rust
    /// let manager = FileManagement::new("data.txt".to_string());
    /// let result = manager.write_file(b"Hello, world!");
    /// ```
    pub fn write_file(&self, data: &[u8]) -> Result<bool, [ResultList; 2]> {
        let mut file = match File::create(&self.file_name) {
            Ok(f) => f,
            Err(e) => {
                return Err([ ResultList::Flag(false), ResultList::Text(e.to_string()) ]);
            }
        };

        match file.write_all(data) {
            Ok(_) => {
                return Ok(true)
            },
            Err(e) => {
                return Err([ ResultList::Flag(false), ResultList::Text(e.to_string()) ]);
            }
        };
    }

    /// Reads the contents of the file as a string.
    ///
    /// - If the file does not exist, it will be created automatically.
    ///
    /// # Returns
    /// - `Ok([ResultList::Flag(true), ResultList::Text(content)])` if successful.
    /// - `Err([ResultList::Flag(false), ResultList::Text(error)])` on failure.
    ///
    /// # Example
    /// ```rust
    /// let manager = FileManagement::new("data.txt".to_string());
    /// match manager.read_file() {
    ///     Ok([ResultList::Flag(_), ResultList::Text(content)]) => println!("Read: {}", content),
    ///     Err([_, ResultList::Text(e)]) => eprintln!("Error: {}", e),
    ///     _ => {}
    /// }
    /// ```
    pub fn read_file(&self) -> Result<[ResultList; 2], [ResultList; 2]> {
        match fs::exists(&self.file_name) {
            Ok(exist) => {
                if exist != true {
                    let _ = File::create(&self.file_name);
                }
            },
            Err(e) => {
                return Err([ ResultList::Flag(false), ResultList::Text(e.to_string()) ]);
            }
        };

        let mut content = String::new();
        let mut file = match File::open(&self.file_name) {
            Ok(f) => f,
            Err(e) => {
                return Err([ ResultList::Flag(false), ResultList::Text(e.to_string()) ]);
            }
        };

        match file.read_to_string(&mut content) {
            Ok(_) => {
                return Ok([ResultList::Flag(true), ResultList::Text(content)]);
            },
            Err(e) => {
                return Err([ ResultList::Flag(false), ResultList::Text(e.to_string()) ])
            }
        };
    }
}
