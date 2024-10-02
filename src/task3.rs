#[test]

/*

// Fill in the blank and fix the error
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::Move{x: 1, y: 2};

    if let Message::Move{__} = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success!");
}
*/






fn main() {
    let msg = Message::Move { x: 1, y: 2 };



    if let Message::Move { x, y: _y } = msg {
        assert_eq!(x, 1); // Пример сравнения; измените при необходимости
    } else {
        panic!("Никогда не позволяйте этому выполняться！");
    }

    println!("Успех!");
}
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
В операторе if let заменили __ на x, y, чтобы разложить вариант Move.
Добавили assert_eq!(x, 1); для сравнения значения x, извлеченного из варианта Message::Move.
Можно изменить второе значение в вызове assert_eq! в зависимости от требований.
*/