fn main() {
    // 부동소수점
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 연산
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    // 불리언
    let t = true;
    let f: bool = false; // with explicit type annotation    

    // 문자
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // 튜플
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // 배열
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];
    let index = 10;
    let element = a[index];
    println!("The value of element is: {}", element);
}