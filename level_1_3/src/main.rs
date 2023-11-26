fn main() {
    // № 2
    // Выведите в консоль все целые числа от 1 до 10.
    for num in 1..11 {
        println!("{}", num);
    }

    // № 3

    // № 4
    // Дано целое число. Проверьте, что первая и последняя цифры этого числа совпадают.
    let num = 848;
    let ch_num = num.to_string();
    let n1 = ch_num.chars().next().unwrap();
    let n2 = ch_num.chars().last().unwrap();

    if n1 == n2 {
        println!("Числа {} и {} совпадают", n1, n2);
    } else {
        println!("Числа {} и {} не совпадают", n1, n2);
    }

    // № 5
    // Выведите в консоль большее из этих чисел.
    let num1: u16 = 32;
    let num2: u16 = 16;
    if num1 > num2 {
        println!("{} больше {}", num1, num2);
    } else {
        println!("{} меньше {}", num1, num2);
    }

    // № 6
    // Дана строка, содержащая целое число.
    // Преобразуйте эту строку в число.
    let str = "123";
    let numbers: u8 = str.parse().unwrap();
    println!("{}", numbers);
}
