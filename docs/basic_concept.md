Rust는 컴파일러가 변경되지 않은 값에 대한 보증을 하며, 실제로도 바뀌지 않음 

상수에는 mut 사용불가능하며, const 키워드 사용

## Shadowing vs mut

    - 가능
    let spaces = "   ";
    let spaces = spaces.len();
    - 불가능
    let mut spaces = "   ";
    spaces = spaces.len();

## Rust Data Types
rust 데이터 타입은 스칼라와 컴파운드로 크게 둘로 구분됨.

### 스칼라 타입
정수형, 부동소수점 숫자, boolean, 문자

정수형(i32가 제일 빠름)

문자(string 큰따옴표, char 작은 따옴표)

### 컴파운드 타입
튜플과 배열

튜플
 
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

배열

벡터도 존재

배열이 넘어갈 경우, 컴파일시 문제 없지만 프로그램 실행중 에러가 발생


## Rust Function
snake_case가 기본 원칙, 함수의 위치는 상관 없음.
매개변수 사용시, 함수 정의에 타입 정의 필수

    fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    }

{}는 표현식으로 사용됨
반환값을 가질 경우 명시할 필요는 없지만, 타입은 작성해야함.


## Rust 조건문

    fn main() {
        let number = 6;
    
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }


### let 구문에서의 if 사용하기
같은 타입을 가질 경우만 가능함.

    fn main() {
        let condition = true;
        let number = if condition {
            5
        } else {
            6
        };
    
        println!("The value of number is: {}", number);
    }


### 반복문

    fn main() {
    let mut number = 3;
    
        while number != 0 {
            println!("{}!", number);
    
            number = number - 1;
        }
    
        println!("LIFTOFF!!!");
    }


    fn main() {
    let a = [10, 20, 30, 40, 50];
    
        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }
