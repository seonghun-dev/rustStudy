Rust ownership

1. 러스트의 각각의 값은 해당값의 오너(owner)라고 불리우는 변수를 갖고 있다.
2. 한번에 딱 하나의 오너만 존재할 수 있다.
3. 오너가 스코프 밖으로 벗어나는 때, 값은 버려진다(dropped).

double free -> memory corruption -> security issue

    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);

위에서, rust는 변수를 이동시킴, double free를 방지하기 위해서
rust에서 깊은 복사를 위해서는 clone() 메서드를 활용하여 복사

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);


### copy 트레잇
특별할 어노테이션, copy 트레잇을 가지고 있을 경우 대입 후에도 예전변수를 그대로 사용가능
u32같은 정수 타입, bool, f64, copy가 가능한 타입으로 구성된 튜플


### 소유권과 함수
