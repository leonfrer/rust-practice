fn main() {
    let msg_quit = Message::Quit;
    let msg_move = Message::Move { x: 1, y: 2 };
    let msg_write = Message::Write(String::from("Hello World"));
    msg_move.call();
    msg_write.call();
    msg_quit.call();

    // option
    // rust doesn't have the null feature like many other language have.
    let five = Some(5);
    let absent_number: Option<i32> = None;
    let six = plus_one(five);
    // five is still accessable
    match five {
        Some(i) => println!("i: {}", i),
        None => (),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // includes an anonymous struct inside it.
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // match are exhaustive
        // that mean we should cover every possible case
        match self {
            Message::Write(content) => {
                println!("msg: {}", content);
            }
            // () is the unit value
            // () is most comonly seen implicitly: function without a `-> ...` implicitly have return type ()
            // _ placehodler will match all the possible cases that aren't specified before it.
            _ => (),
        }

        // you also could use Concise Control Flow
        // use if let syntax
        // with if let you will lose exhaustive
        if let Message::Move { x, y } = self {
            println!("moving x: {}, y: {}", x, y);
        }

        if let Message::Quit = self {
            println!("quiting...");
        } else {
            println!("normal method called")
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
