use serde;
use serde_json;

/// ## Basic Task Structure
///
/// The `ITask` struct defines a basic task model with an ID, a description,
/// and a completion status. It supports serialization and deserialization
/// via `serde`.
#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct ITask {
    /// Unique identifier of the task
    pub id: usize,

    /// Description of the task
    pub description: String,

    /// Indicates whether the task is completed
    pub completed: bool,
}

/// ## JSON Conversion Utilities
///
/// The `Formatter` struct provides methods for converting data
/// between JSON strings and Rust objects (and vice versa).
pub struct Formatter;

impl Formatter {
    /// Creates a new instance of `Formatter`.
    ///
    /// This method currently takes no parameters, but it's designed to be
    /// extendable in the future.
    pub fn new(/* None */) -> Self {
        Formatter
    }

    /// Converts a JSON string into a Rust object or list of objects.
    ///
    /// # Parameters
    /// - `data`: A `String` containing JSON-formatted data.
    ///
    /// # Returns
    /// - `Ok(T)` if deserialization succeeds.
    /// - `Err(String)` with the error message if it fails.
    ///
    /// # Example
    /// ```rust
    /// let formatter = Formatter::new();
    /// let json = r#"[{"id":1,"description":"Example","completed":false}]"#.to_string();
    /// let tasks: Result<Vec<ITask>, _> = formatter.str_to_object_or_list(json);
    /// ```
    pub fn str_to_object_or_list<T>(&self, data: String) -> Result<T, String>
    where
        for<'a> T: serde::Deserialize<'a>,
    {
        match serde_json::from_str::<T>(&String::from(data)) {
            Ok(c) => Ok(c),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Converts a serializable Rust object to a pretty-formatted JSON string.
    ///
    /// # Parameters
    /// - `data`: Any object that implements the `Serialize` trait.
    ///
    /// # Returns
    /// - `Ok(String)` if serialization succeeds.
    /// - `Err(String)` with the error message if it fails.
    ///
    /// # Example
    /// ```rust
    /// let task = ITask { id: 1, description: "Hello".to_string(), completed: false };
    /// let formatter = Formatter::new();
    /// let json = formatter.object_or_list_to_string(task).unwrap();
    /// ```
    pub fn object_or_list_to_string<T>(&self, data: T) -> Result<String, String>
    where
        T: serde::Serialize,
    {
        match serde_json::to_string_pretty(&data) {
            Ok(s) => Ok(s),
            Err(e) => Err(e.to_string()),
        }
    }
}
