#[test]

/*

// Fix the errors
enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2 {
    Zero = 0.0,
    One = 1.0,
    Two = 2.0,
}


fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One, Number1::One);
    assert_eq!(Number1::One, Number2::One);

    println!("Success!");
}
*/

fn main() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number::One as u8, Number1::One as u8);
    assert_eq!(Number1::One as u8, Number2::One as u8);

    println!("Success!");
}
// Fix the errors
#[allow(dead_code)]
enum Number {
    Zero,
    One,
    Two,
}
#[allow(dead_code)]
enum Number1 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
#[allow(dead_code)]
enum Number2 {
    Zero = 0,
    One = 1,
    Two = 2,
}
/*
Убрано использование плавающих чисел в перечислении Number2,
так как Rust не поддерживает это.
В выражениях assert_eq! теперь явно приводим перечисления к u8,
чтобы обеспечить корректное сравнение.
*/


