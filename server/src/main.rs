fn main() {

    // let string  = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..];
    // let string_borrow = &string;
    // let string_literal = "1234";


    // println!("{}",string);
    // println!("{}",string_slice);
    // println!("{}",string_borrow);
    // println!("{}",string_literal);


    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server{
    addr: String,
}

impl Server{
    fn new(addr: String) -> Self{
        Self {
            addr
        }
    }

    fn run(self){
        println!("Listening on {}",self.addr)
    }
}