extern crate rand;
/*
    외부 의존 크레이트가 있음을 알림.
 */

use std::io;
use std::cmp::Ordering;
use rand::Rng;
/*
    Rng는 정수 생성기가 구현한 메소드를 정의한 trait
 */

/*
    사용자의 입력받기 위해서, std::io 라이브러리 사용해야함.
    러스트는 모든 프로그램의 스코프에 prelude내의 타입들을 가져옴.
    prelude에 없다면 use를 사용해서 명시적으로 가져와야함.
 */

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    /*
        get_range(), 하한선 포함, 상한선 비포
     */
    println!("The secret number is: {}", secret_number);
loop{
    println!("Please input your guess.");

    let mut guess = String::new();
    /*
       변수 생성을 let을 사용
       러스트에서 변수는 기본적으로 불변
       mut을 사용해서 가변변수를 사용할 수 있음.
       ::는 new가 String의 연관함수임을 나타냄, 연관함수는 하나의 타입을 위한 함수
       String 인스턴스 X, String 타입을 위함 함수, 정적 메서드
     */
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    /*
        read_line은 사용자가 표준 입력시 입력된 문자를 저장
        &는 코드의 여러 부분에서 데이터를 여러 번 복사하지 않고 접근하기 위한 참조자
        참조자를 통해서 안정성과 용이성을 확보
        참조자가 기본적으로 불변이므로 가변으로 변경하기 위해서
        &mut guess로 사용
        .foo() 형태의 문법으로 가독성 증대
        readline의 결과로 io::Result가 반환, Result의 variants는 Ok와 Err,
        Ok는 처리 성공과 성공적으로 생성된 결과
        Err은 처리가 실패했음과 그 이유
    */
    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };
    /*
        except 메소드 match 표현식으로 변경시 에러 발생시 종료에서 처리로 바꿈
        Ok, Err을 처리
     */



    /*
        이전에 사용된 guess의 값을 가리는 것을 허락
        Shadowing 고유의 변수명 만들기 대신 재사용을 허락
        parse() 메소드는 문자열을 숫자형으로 파싱
     */

    println!("You guessed: {}", guess);
    /*
        {}는 형식 표현자로, 변경자로써 값이 표시되는 위치를 나타냄
     */

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal   => {
            println!("You win!");
            break;
        },
    }
}
}
