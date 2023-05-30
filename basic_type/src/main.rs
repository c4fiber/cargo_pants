fn main() {
    println!("Hello, world!");

    // println!("{}", (-4).abs()); 는 오류가 발생한다. 값이 정확히 어떤 정수타입인지 모르기 때문.
    // 만약 u32 같은 unsigned 값이면 abs() 메서드가 없다.

    // 괄호를 제거하면 결과값이 -4가된다.. 메서드 호출은 단항 전위 연산자보다 우선순위가 높음.
    println!("{}", (-4_i32).abs());
    println!("{}", i32::abs(-4));
}

#[test]
fn test_type() {
    assert_eq!(   10_i8 as u16, 10_u16);        // 표현 범위 안에 있음
    assert_eq!(2525_u16 as i16, 2525_i16);      // 표현 범위 안에 있음

    assert_eq!(   -1_i16 as i32, -1_i32);       // 빈 공간을 부호로 채움
    assert_eq!(65535_u16 as i32, 65535_i32);    // 빈 공간을 0으로 채움

    // 좁은 타임으로 가는 변환은 원래 값을 2^N으로 나눈 나머지에 해당하는 값을 산출하는데,
    // 여기서 N은 비트 단위로 된 대상 타입의 크기다. 이를 두고 '잘림'이라고도 한다.
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!(65535_u16 as i32, 65535_i32);

    assert_eq!( -1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);               // 거듭제곱
    assert_eq!((-4_i32).abs(), 4);              // 절댓값
    assert_eq!(0b101101_u8.count_ones(), 4);    // 개수 세기
}

// 디버그 빌드에서(릴리즈는 해당X) 정수 산술 연산이 오버플로를 일으키면 러스트는 패닉에 빠진다.
// 릴리즈 빌드에서는 음수를 순환하면서 무한루프에 빠진다. 패닉은 발생하지 않는다.
fn panic_overflow1() {
    let mut i = 1;
    loop {
        i *= 10; // 패닉: 오버플로를 일으키는 곱셈
    }
}

// 빌드에 관계없이 반드시 패닉에 빠진다.
fn panic_overflow2() {
    let mut i: i32 = 1;
    loop {
        // 패닉: 오버플로를 일으키는 곱셈
        i = i.checked_mul(10).expect("multiplication overflowed");
    }
}


/* 정수 산술 메서드는 크게 네 가지 범주로 나뉜다. */
// 점검(checked): 결과를 Option에 담하 반환한다. Some(v) 또는 None을 반환한다.
#[test]
fn checked() {
    // 10과 20의 합은 u8로 표현할 수 있다.
    assert_eq!(10_u8.checked_add(20), Some(30));

    // 안타깝게도 100과 200의 합은 그럴 수 없다.
    assert_eq!(100_u8.checked_add(200), None);

    // 덧셈을 하는데 오버플로가 발생하면 패닉에 빠진다.
    let x: u8 = 100;
    let y: u8 = 200;
    // let sum = x.checked_add(y).unwrap();

    // 이상하지만 부호 있는 나눗셈도 오버플로를 일으키는 경우가 있다.
    // 부호 있는 n비트 타입은 -2^n-1은 표현할 수 있지만 2^n-1은 표현할 수 없다.
    assert_eq!((-128_i8).checked_div(-1), None);
}

// 순환(wrapping): 수학적으로 옳은 결과를 주어진 값의 범위로 나눈 나머지에 해당하는 값을 반환한다.
#[test]
fn wrapping() {
    // 첫 번째 곱은 u16으로 표현할 수 잇다.
    // 두번째 곱은 그럴 수 없으므로 250_000을 2^16으로 나눈 나머지가 산출된다.
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);

    // 부호 있는 타입을 대상으로 하는 연산은 음숫값으로 순환될 수도 있다.
    assert_eq!(500_i16.wrapping_mul(500), -12144);

    // 비트별 자리 이동 연산에서는 이동 거리가 값의 크기 안에 들어가도록 순환된다.
    // 따라서 16비트 타입을 대상으로 하는 17비트 자리 이동은 1비트 자리 이동과 같다.
    assert_eq!(5_i16.wrapping_shl(17), 10);
    assert_eq!(0b0101_i16.wrapping_shr(1), 0b0010_i16);     // 간단한 비트 쉬프트 연산
}

// 포화(saturating): 표현할 수 있는 값 내에서 수학적으로 옳은 결과게 가장 가까운 값을 반환한다.
#[test]
fn saturating() {
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
}

// 넘짐(overflowing): (result, overflowed)튜플을 반환한다.
#[test]
fn overflowing() {
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    // `u16`을 대상으로 하는 17비트 자리 이동은 거리가 타입 자체의 비트 크기를 넘어선다.
    // 따라서 17을 16으로 나눈 나머지인 1이 실제로 적용되는 자리 이동 거리다.
    assert_eq!(5_u16.overflowing_shl(17), (10, true));
}