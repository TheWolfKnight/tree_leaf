
mod services;
mod models;

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::services::WebPath;
    use crate::services::Server;

    use crate::models::FolderNode;

    #[test]
    fn web_server_path_parse() {
        let prefix: &str = "/home/www/www.test.com/";
        let web_path: WebPath = WebPath::new(prefix);
        let web_path_result: Option<Vec<&str>> = web_path.parse_path("/home/www/www.test.com/hello/world.html");

        assert_eq!(web_path_result, Some(vec!["hello", "world.html"]));
    }

    #[test]
    fn server_ipv4_parsing() {
        let prefix: &str = "/home/www/www.test.com/";
        let server: Server = Server::new([127, 0, 0, 1], prefix);

        assert_eq!(Ok(server.connection), "127.0.0.1".parse())
    }

     #[test]
     fn folder_tree_instance() {
        let path: &str = "/home";
        let folder_tree: FolderNode = FolderNode::new(path);

        assert_eq!(folder_tree.name,     "home");
        assert_eq!(folder_tree.files.len(),   0);
        assert_eq!(folder_tree.folders.len(), 0);
     }
}
