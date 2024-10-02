#[test]

/*

// Fill in the blank and fix the errors
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs: __ = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
}

fn show_message(msg: Message) {
    println!("{}", msg);
}
*/





fn main() {
    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message(msg);
    }
}

fn show_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}
#[allow(dead_code)]

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
Тип для msgs должен быть массивом Message.
Функция show_message пытается напечатать вариант перечисления напрямую,
что невозможно, так как Message не реализует трейт Display.
Вместо этого можно использовать сопоставление с образцом для вывода значений.

Изменено let msgs: __ на let msgs: [Message; 3] для явного указания типа массива.
Использован оператор match в функции show_message для обработки каждого варианта перечисления Message,
предоставляя соответствующие строки для вывода.
*/
