// Сборник задач по Rust от Дмитрия Трепачева. Уровень 1.1
fn main(){
    // № 1
    // Дано число:
    // Проверьте, отрицательное оно или нет. Выведите об этом информацию в консоль.
    let num: i32 = 123;
    if num < 0 {
        println!("Число отрицательное");
    } else {
        println!("Число положительное");
    }
}