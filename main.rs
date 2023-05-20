fn main() {
    /* 
    комментарий
    */
    println!("Эта программа покажет, как я могу использовать разные возможности этого языка");
    let mynumber: f32 = 123.123; // константа
    // f32 f64
    let mut anothernumber: i32; // переменная
    anothernumber = 321542;
    //i8 i16 i32 i64 i128 isize - 64, так как у меня 64 битная система
    //u8 u16 u32 u64 u128 usize - unsigned
    println!("mynumber = {}", mynumber);
    println!("anothernumber = {}", anothernumber);
    anothernumber = 123456;
    println!("anothernumber = {}", anothernumber);
    
    let name: &str = "Svyatik"; // небезопасный способ создания строк
    let anothername: String = String::from("kotik"); // безопасный способ создания строк
    println!("{} == {}", name, anothername);
    // __________________________________

    // char, bool
    let bukva: char = 'r';
    let heart_eyed_cat: char = '😻';
    println!("{}, {}", bukva, heart_eyed_cat);
    let logic: bool = true; // +false
    println!("{}", logic);
    // __________________________________

    // https://doc.rust-lang.ru/book/appendix-02-operators.html
    // addition
    let sum: i32 = 5 + 10;

    // subtraction
    let difference: f64 = 95.5 - 4.3;

    // multiplication
    let product: i32 = 4 * 30;

    // division
    let quotient: f64 = 56.7 / 32.2;
    let truncated: i32 = -5 / 3; // Results in -1

    // remainder
    let remainder: i32 = 43 % 5;
    println!("{}, {}, {}, {}, {}, {}", sum, difference, product, quotient, truncated, remainder);
    // __________________________________

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    let five_hundred: i32 = tup.0;
    println!("The value of y is: {y}");
    println!("{}", five_hundred);

    let massiv: [i32; 5] = [1, 2, 3, 4, 5]; 
    let _massiv2: [i32; 5] = [3; 5];
    let _first: i32 = massiv[0];

    let age: i8 = 20;
    // > >= == !=
    if age >= 18 {
        print!("blabla");
    } else { //else if, || ИЛИ, && И
        print!("123123123");
    }
    let _num: bool = if logic {true} else {false}; // 1 или 0 вызовет ошибку
}
