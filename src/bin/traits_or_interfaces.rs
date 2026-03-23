use std::io::Error;

fn main() {
    let socket = Socket{};
    expect_reader(&socket);


    let stream = StreamHandle{
        stream: socket
    };

    stream.read();


    // this won't work because intigers do not implement the trait Reader
    // let stream = StreamHandle{
    //     stream: 1,
    // };
    //
    // stream.read();
}


fn expect_reader(r: &impl Reader) {

}

// implicit trait implementatio
struct Socket {}

impl Reader for Socket {
    fn read(&self) -> String {
        return String::from("some data")
    }
}

struct File {}



impl Reader for File {
    fn read(&self) -> String {
        return String::from("some data from a file")
    }
}


trait ReadCloser: Reader + Closer {}

trait Reader {
    fn read(&self) -> String;
}

trait Closer {
    fn close(&self) -> Option<Error>;
}



// traits with default behavior 
trait Summary {
    // can be overriden by another type for itself
    fn summarize(&self) -> String {
        String::from("here is the summary")
    }
}



// the where clause
fn expect_reader_and_closer<R, C>(reader: &R, closer: &C)
where 
    R: Reader,
    C: Closer
{
    let data = reader.read();
    println!("data read: {}", data);
    if let Some(err) = closer.close() {
        println!("didn't close properly: {}", err);
    }
}



// so now if T implements Reader trait, we can call read method on the StreamHandle
struct StreamHandle<T> {
    stream: T
}


impl<T: Reader> StreamHandle<T> {
    fn read(&self) -> String {
        return String::from("some data from a file")
    }
}

























