use oyster::App;
fn main() {
    let mut app = App::new();

    app.get("/hi", |req, mut res| {
        println!("HTTP request info: {:#?}", req);
        res.send("hey");
        res.status(201)
    });

    app.get("/hello", |req, mut res| {
        println!("HTTP request info: {:#?}", req);
        res.send("hello").status(201)
    });

    app.listen(3000).unwrap();
}
