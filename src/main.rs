use oyster::App;
fn main() {
    let mut app = App::new();

    app.get("/hi", |req, mut res| {
        println!("host is {}", req.host);
        res.send("hey");
        res.status(201)
    });

    app.get("/hello", |req, mut res| {
        println!("host is {}", req.host);
        res.send("hello")
    });

    app.listen(3000).unwrap();
}
