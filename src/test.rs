#[cfg(test)]
mod tests{
    use crate::Client;
    use crate::connection::Permission::Admin;

    #[test]
    fn test_connect(){
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
    }

    #[test]
    fn test_send_values(){
        let client = Client::new("localhost", 9999);
        let mut connection = client.connect().unwrap();
        connection.send("Hello world").unwrap();
    }

    #[test]
    fn test_receive_values(){
        let client = Client::new("localhost", 8686);
        let mut connection = client.connect().unwrap();

        for _ in 0..10{
            let value = connection.receive().unwrap();
            println!("{:?}", value);
        }
    }

    #[test]
    fn test_admin() {
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
        assert!(connection.has_permission(&Admin))
    }
    
}