#[cfg(test)]
mod tests{
    use crate::Client;
    use crate::messages::Train;

    #[test]
    fn test_connect(){
        let client = Client::new("localhost", 5959);
        let _ = client.connect().unwrap();
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
            let value = connection.receive_msg().unwrap();
            println!("{:?}", value);
        }
    }

    #[test]
    fn test_admin() {
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
        assert!(connection.admin().is_ok());

        let client = Client::new("localhost", 8686);
        let connection = client.connect().unwrap();
        assert!(connection.admin().is_err());
    }

    #[test]
    fn test_disconnect() {
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
        drop(connection);

        for _ in 0..10 {
            let client = Client::new("localhost", 5959);
            let connection = client.connect().unwrap();
        }
    }

    #[test]
    fn test_create_plan() {
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
        let mut admin = connection.admin().unwrap();

        let id = admin.create_plan("Test Plan", "0--1--2").unwrap();
    }

    #[test]
    fn test_delete_plan() {
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
        let mut admin = connection.admin().unwrap();
        let id = admin.create_plan("Test Plan", "0--1--2").unwrap();
        assert!(admin.delete_plan(id).is_ok())
    }

    #[test]
    fn test_get_plans() {
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
        let mut admin = connection.admin().unwrap();
        let amount = admin.get_plans().unwrap().len();

        let _ = admin.create_plan("Test Plan", "0--1--2").unwrap();
        assert_eq!(admin.get_plans().unwrap().len(), amount + 1);
    }

    #[test]
    fn test_plan_start_stop() {
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
        let mut admin = connection.admin().unwrap();
        let id = admin.create_plan("Test Plan", "0--1--2").unwrap();

        admin.start_plan(id).unwrap();
        admin.stop_plan(id).unwrap();

    }


    #[test]
    fn test_wordcount() {
        let client = Client::new("localhost", 5959);
        let connection = client.connect().unwrap();
        let mut admin = connection.admin().unwrap();

        let input_port = 6565;
        let output_port = 6767;

        let id = admin.create_plan(
            "Word Count",
            "\
            0--1{sql|SELECT * FROM UNWIND(SELECT SPLIT($0, '\\s+') FROM $0)}--2\n\
            \n\
            In\n\
            Tpc{\"url\":\"127.0.0.1\",\"port\":6565}:0\n\
            Out\n\
            Tpc{\"url\":\"127.0.0.1\",\"port\":6767}:2"
        ).unwrap();

        admin.start_plan(id).unwrap();

        let client = Client::new("localhost", input_port);
        let mut connection = client.connect().unwrap();


        for _ in 0..100_000 {
            connection.send("This is a test sentence.").unwrap();
        }

        let client = Client::new("localhost", output_port);
        let mut connection = client.connect().unwrap();

        println!("{:?}", connection.receive::<Train>().unwrap());

    }
    
}