
// include files in the module tree
pub mod folder_tree;
pub mod thread_pool;
pub mod worker;
pub mod settings;

// include the specific structs in the module tree
pub use folder_tree::FolderNode;
pub use thread_pool::ThreadPool;
pub use worker::Worker;
pub use settings::Settings;
