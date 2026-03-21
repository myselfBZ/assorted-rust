enum Message {
    Quit,
    Write(String),
    Move {x: i32, y: i32}
}

impl Message {
    fn call(&self) {

    }
}

fn main() {
    let frame_opt = get_resp_frame();
    // match frame_opt {
    //     Some(f) => println!("{}", f),
    //     None => println!("None")
    // }

    //we could also do this 
    if let Some(f)  = frame_opt {
        println!("{}", f)
    }
}


fn get_resp_frame() -> Option<String> {
    return Some(String::from("SET THIS"))
}
