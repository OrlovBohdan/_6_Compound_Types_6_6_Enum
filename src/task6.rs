
#[test]

/*

use crate::List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> __ {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // After Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // Instead take a reference to the tail
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, __ tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.__())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
*/






/*fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        List::Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List { // Указан возвращаемый тип
        // `Cons` also has type List
        List::Cons(elem, Box::new(self))
    }

    // Return the length of the list
    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => { // Изменено на List::Cons
                format!("{}, {}", head, tail.stringify()) // Изменен вызов на tail.stringify()
            },
            List::Nil => {
                format!("Nil")
            },
        }
    }
}*/



fn main() {
    // Создаем пустой связный список
    let mut list = List::new();

    // Добавляем элементы в начало списка
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Показываем конечное состояние списка
    println!("Длина связного списка: {}", list.len());
    println!("{}", list.stringify());
}

// Определяем перечисление List для связного списка
enum List {
    // Cons: Кортежная структура, которая оборачивает элемент и указатель на следующий узел
    Cons(u32, Box<List>),
    // Nil: Узел, который обозначает конец связного списка
    Nil,
}

// Методы могут быть добавлены к перечислению
impl List {
    // Создать пустой список
    fn new() -> List {
        // `Nil` имеет тип `List`
        List::Nil
    }

    // Употребить список и вернуть тот же список с новым элементом в начале
    fn prepend(self, elem: u32) -> List {
        // `Cons` также имеет тип List
        List::Cons(elem, Box::new(self))
    }

    // Вернуть длину списка
    fn len(&self) -> u32 {
        // Используем сопоставление для определения длины
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    // Вернуть представление списка в виде (выделенной в куче) строки
    fn stringify(&self) -> String {
        match *self {
            List::Cons(head, ref tail) => {
                // Используем `format!` для создания строкового представления
                format!("{}, {}", head, tail.stringify())
            },
            List::Nil => {
                format!("Nil")
            },
        }
    }
}

/*
Добавлено префиксирование вариантов перечисления в методах new, prepend и stringify с помощью List::,
чтобы указать, к какому варианту мы обращаемся.
Обновлен возвращаемый тип для функции prepend на List.
Использован tail.stringify() вместо tail.__(), чтобы правильно вызвать метод stringify для хвоста.

Cons: Представляет узел в списке, содержащий значение и указатель на следующий узел.
Nil: Представляет конец списка.
Методы: Реализованы функции для создания нового списка, добавления элементов в начало, вычисления длины и преобразования списка в строку.
Box: Используется для хранения List в памяти, выделенной в куче, что позволяет создавать рекурсивные типы данных.
*/



