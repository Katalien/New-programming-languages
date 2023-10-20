#![allow(dead_code)]
// 1. Стек и куча. section1()
// 2. Концепция владения. section2()
// 3. Пользовательские типы данных. section3()
// 4. Структуры. section4()
// 5. Перечисления. section5()
// 6. Имплементация структур. section6()
// 7. Имплементация перечислений. section7()

use std::num::FpCategory::Normal;
use chrono::{NaiveDate};

fn section1_1() {
    let point = Box::new((0.625, 0.5)); // здесь выделена память для point
    let label = format!("{:?}", point); // здесь выделена память для label
    assert_eq!(label, "(0.625, 0.5)");
} // здесь обе переменные уничтожены

fn section1_2() {
    struct Person {
        name: String,
        birth: i32,
    }
    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });
    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }
}

fn section1() {
    section1_1();
    section1_2();
}

fn section2_1() {
    println!("\n Rule 1 ");

    /// Правило 1: У каждого значения может быть только один «владелец»

    // С примитивными данными всегда будет выполняться копирование (а также кортежи с примитивными данными)
    let x = 10;
    let y = x;

    println!("{} {}", x, y);

    // С составными типами данных выполняется перемещение
    let str1 = "sample".to_string();
    /* let str2 = str1;
     * println!("{} {}", str1, str2) // ! будет ошибка, если попытаемся использовать str1
     */

    // если хотим пользоваться обеими переменными str1 и str2 нужно производить копирование
    let str2 = str1.clone();

    println!("{} {}", str1, str2);
}

fn section2_2() {
    println!("\n Rule 2");

    /// Правило 2: Когда «владелец» выходит из области видимости, выделенная память высвобождается
    { // Создадим скоуп в котором будут лежать переменные
        let str = "sample".to_string();
        let a = 10;
    }
    // println!("{} {}", str, a) // ! Ошибка, память, выделенная под str и a,
    // созданные в скоупе, очищены после выхода из него
}

fn section2_3() {
    println!("\n Rule 3");

    /// Правило 3: для передачи значения переменной необходимо явно выполнять копирование

    fn print_str_danger(str: String) { // Функция перенимает владение переменной
        println!("{}", str);
    } // При выходе из функции, "владелец" будет уничтожен, вместе со всеми данными

    let str = "string in function".to_string();

    /*
    print_str_danger(str);
    println!("{}", str); // будет ошибка: значение str освободится после работы функции print_str_danger(str)
    */

    print_str_danger(str.clone()); // передаем новый экземпляр строки в функцию,
    println!("{}", str);
}

fn section2_4() {
    println!("\n Rule 4");

    /// Правило 4: передача владения осуществляется через оператор "=" или явно через метод или функцию

    fn broker(str: String) -> String { // Если пытаться возвращать ссылку на строку,
        // будет ошибка компилляции - висячая ссылка,
        // объект уничтожен, а ссылка осталась
        str
    }

    let str1 = "Передача владения через функцию".to_string(); //
    let str2 = broker(str1);

    println!("{}", str2);
}

fn section2_5() {
    println!("\n Rule 5");

    /// Правило 5: передача объекта без передачи владения осуществляется с помощью ссылок

    fn get_length(str: &String) -> usize {
        str.len()
    }

    let str = "String with length".to_string();

    println!("{} {}", get_length(&str), str);
}

fn section2_6() {
    println!("\n Rule 6");

    /// Правило 6: Для изменения объекта необходимо использовать immutable ссылки («mut»)

    let mut str1 = "Some".to_string(); // Ссылка на str1 изменяемая, имеется ключевое слово "mut"
    let str2 = str1.clone();           // Ссылка неизменяемая

    str1.push_str("thing");
    // str2.push_str("body");                 // Ошибка! Ссылка неизменяемая
    println!("{}", str1);

    /// Аналогично с функциями
    fn append(str: &mut String) { // Внимание! Синтаксис
        str.push_str(" pushing");
    }

    append(&mut str1); // И тут внимание!
    println!("{}", str1);
}

fn section2_7() {
    println!("\n Rule 7");

    /// Правило 7: Нельзя иметь одновременно больше одной изменяемой ссылки на одно и то же значение

    let mut str = "sample text".to_string();

    let reference = &mut str;

    println!("{}", reference);

    /*
    let reference1 = &mut str;
    let reference2 = &mut str;
    println!("{} {}", reference1, reference2);
     */ // Ошибка компилляции! Читай правило 7
}

fn section2_8() {
    println!("\n Rule 8");

    /// Правило 8: Можно иметь сколько угодно неизменяемых ссылок на объект
    let mut str = "sample text".to_string();

    let reference1 = &str;
    let reference2 = &str;
    // let reference2 = &mut str; // будет ошибка компилляции, нельзя создавать изменяемую ссылку на объект,
    // если уже есть неизменяемая ссылка на объект
    println!("{}, {}", reference1, reference2);
}

fn section2() {
    section2_1();
    section2_2();
    section2_3();
    section2_4();
    section2_5();
    section2_6();
    section2_7();
    section2_8();
}

fn section3() {
    struct SomeStructure {
        some_integer: i32,
        some_double: f64,
        some_symbol: char,
    }

    enum Numbers {
        ZERO,
        TEN = 10,
        HUNDRED = 100,
    }
}

fn section4_1() {
    // Создадим структуру Person
    // Рассмотрим распространенные действия с ней
    #[derive(Debug)] // Для простого вывода в консоль
    struct Person {
        name: String,
        surname: String,
        age: i32,
        birthday: Option<NaiveDate>,
    }

    let name = "Stepan".to_string();
    let surname = "Syrkov".to_string();

    let person1 = Person {
        name, // Если переменные названы именами полей в структуре, то можно сократить написание кода
        surname,
        birthday: NaiveDate::from_ymd_opt(2000, 10, 10),
        age: 22,
    };

    println!("{:?}", person1);

    // person1.age = 23; // ошибка компилляции! ссылка неизменяемая
    // мы можем только читать поля структуры

    // создадим изменяемый объект

    let mut person2 = Person {
        name: "Darya".to_string(), // Если переменные названы именами полей в структуре, то можно сократить написание кода
        surname: "Syrkova".to_string(),
        birthday: NaiveDate::from_ymd_opt(2002, 1, 10),
        age: 20,
    };

    person2.age = 21; // теперь данные изменяются

    println!("{:?}", person2);

    // также можно создавать объекты на основе существующих

    let person3 = Person {
        name: "Vasya".to_string(),
        birthday: NaiveDate::from_ymd_opt(2000, 12, 20),
        ..person1 // с помощью данного синтаксического сахара
    };

    println!("{:?}", person3);

    let person4 = Box::new(Person { //
        name: "Gennadiy".to_string(),
        surname: "Ivanov".to_string(),
        birthday: NaiveDate::from_ymd_opt(1948, 6, 10),
        age: 75,
    });

    println!("{:?}", person4);
}

fn section4_2() {
    // Структуры имеют модификаторы доступа

    pub struct FullPublic {
        // Структура видима извне
        pub string: String,
        pub color: (i8, i8, i8),
    }

    pub struct Public {
        // Структура видима извне, но поля закрыты,
        // создать структурным выражением нельзя в другом модуле
        string: String,
        color: (i8, i8, i8),
    }

    FullPublic { string: "".to_string(), color: (0, 0, 0) };
}

fn section4_3() {
    // Кортежеподобные структуры

    // У таких структур также есть модификаторы доступа
    struct BoundsPrivate(usize, usize);

    pub struct BoundsPublic(pub usize, pub usize);

    // Пример создания кортежеподобной структуры
    let mut image_bounds = BoundsPrivate(1024, 768);

    // Обращаемся как к массиву
    image_bounds.0 = 512;
    image_bounds.1 = 256;
}

fn section4_4() {
    // Характеристики структур

    #[derive(Copy, Clone, Debug, PartialEq)] // ХАРАКТЕРИСТИКИ
    struct Point {
        x: f64,
        y: f64,
    }

    let p = Point { x: 0.0, y: 0.0 };
    let a = p; // Можем копировать (Характеристика Copy)
    let b = p.clone(); // Можем клонировать (Характеристика Clone)

    println!("{:?}", p); // Можем делать удобный вывод (Характеристика Debug)

    println!("{}", a == b); // Можем сравнивать (Характеристика PartialEq

    // Есть и другие характеристики, например PartialCmp - сравнение "<" ">" и т.п.
}

fn section4_5() {
    pub struct Queue<T> {
        older: Vec<T>,
        younger: Vec<T>,
    }

    impl<T> Queue<T> {
        pub fn new() -> Queue<T> {
            Queue { older: Vec::new(), younger: Vec::new() }
        }
        pub fn push(&mut self, t: T) {
            self.younger.push(t);
        }
        pub fn is_empty(&self) -> bool {
            self.older.is_empty() && self.younger.is_empty()
        }
    }

    let mut q = Queue::<char>::new();
}

fn section4() {
    section4_1();
    section4_2();
    section4_3();
    section4_4();
}


fn section5() {
    enum OperationType {
        Add,
        // сложение
        Subtract,
        // вычитание
        Multiply,
        // умножение
        Divide,    // деление
    }

    let a = 10;
    let b = 5;
    let mut op = OperationType::Add;

    let mut result = get_result(a, b, op);
    println!("result = {}", result);        // result = 15

    // меняем операцию
    op = OperationType::Subtract;
    result = get_result(a, b, op);          // result = 5
    println!("result = {}", result);

    // передаем еще одно значение
    result = get_result(a, b, OperationType::Divide);
    println!("result = {}", result);        // result = 2

    fn get_result(x: i32, y: i32, op: OperationType) -> i32 {
        match op
        {
            OperationType::Add => x + y,
            OperationType::Subtract => x - y,
            OperationType::Multiply => x * y,
            OperationType::Divide => x / y
        }
    }
}

fn section6() {
    // создадим структуру прямоугольника и имплементируем для него некоторые методы
    #[derive(Debug)] // Для простого вывода в консоль
    struct Rectangle {
        width: f64,
        height: f64,
    }
    impl Rectangle {
        fn get_width(&self) -> f64 {
            self.width
        }
        fn get_height(&self) -> f64 {
            self.height
        }
        fn calc_area(&self) -> f64 {
            self.height * self.width
        }
        fn calc_len_diagonal(&self) -> f64 {
            (self.width * self.width + self.height * self.height).sqrt()
        }

        fn create_square(side: f64) -> Rectangle { // связная функция
            Rectangle {
                width: side,
                height: side,
            }
        }
    }

    let rect = Rectangle {
        width: 3.0,
        height: 4.0,
    }; // создание структуры

    println!("width = {}, height = {}, area = {}, length of diagonal equal {}",
             rect.width, rect.height, rect.calc_area(), rect.calc_len_diagonal());

    let square = Rectangle::create_square(5.0); // вызываем ассоциированный со структурой метод
    println!("square sides {:?}", square);
}

fn section7() {
    enum Comment {
        Excellent(String),
        // Перечисления могут владеть данными
        Good(String),
        Normal(String),
        Awful(String),
    }
    impl Comment {
        fn express_correctly(&self) { // вариант для обработчика ошибок
            match self {
                Comment::Excellent(e) => println!("excellent {}", e),
                Comment::Good(g) => println!("good {}", g),
                Comment::Normal(n) => println!("normal {}", n),
                Comment::Awful(a) => println!("awful {}", a)
            }
        }
    }

    let comment = Comment::Good("hello".to_string());

    comment.express_correctly();
}

fn main() {
    section1();
    section2();
    section3();
    section4();
    section5();
    section6();
    section7();
}
