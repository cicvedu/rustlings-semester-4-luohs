/*
 * @Author: 罗华胜 luohuasheng0225@163.com
 * @Date: 2024-07-06 14:05:42
 * @LastEditors: 罗华胜 luohuasheng0225@163.com
 * @LastEditTime: 2024-07-06 15:34:35
 * @FilePath: /exercises/enums/enums2.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.



#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move{x : i32, y : i32},
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
