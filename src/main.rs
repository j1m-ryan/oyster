use oyster::HTTPServer;
use oyster::Oyster;

fn main() {
    let mut oyster = Oyster::new();

    let mut main_server = HTTPServer {
        http_port: 3000,
        server_name: "localhost".to_string(),
        ..HTTPServer::default()
    };

    main_server.get("/hi", |req, mut res| {
        println!("HTTP request info: {:#?}", req);

        res.send("hey");
        res.status(201)
    });

    main_server.get("/hello", |req, mut res| {
        println!("HTTP request info: {:#?}", req);
        res.send("hello").status(201)
    });

    oyster.http.add_server(main_server);

    let mut other_server = HTTPServer {
        http_port: 3001,
        server_name: "localhost".to_string(),
        ..HTTPServer::default()
    };

    other_server.get("/hello", |req, mut res| {
        println!("HTTP request info: {:#?}", req);
        res.send("hello").status(201)
    });

    oyster.http.add_server(other_server);

    oyster.start();
}
