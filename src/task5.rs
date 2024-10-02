#[test]

/*
// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    if let __ = six {
        println!("{}", n);

        println!("Success!");
    }

    panic!("NEVER LET THIS RUN！");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        __ => None,
        __ => Some(i + 1),
    }
}*/


// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    if let Some(n) = six { // Заполните пропуск здесь
        println!("{}", n);
        println!("Успех!");
    } else {
        println!("Не удалось прибавить единицу к None."); // Добавляем сообщение для случая None
    }

    // Убираем panic!
    // panic!("NEVER LET THIS RUN！");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Заполните пропуск здесь
        Some(i) => Some(i + 1), // Заполните пропуск здесь
    }
}

/*
В if let Some(n) = six добавлено сопоставление с Some, чтобы извлечь значение n.
В match x добавлены варианты None и Some(i).
Добавлено сообщение для случая, когда six равно None, что позволяет избежать вызова panic!.
*/
