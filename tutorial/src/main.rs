use std::str::FromStr;
use std::env;

fn main() {
    // vector 같이 지속적으로 값이 변하는 경우 반드시 mutable로 선언한다.
    let mut numbers = Vec::new();

    // env를 통해 args를 가져올 수 있다, skip(1)로 한개의 값을 건너뛴다.
    for arg in env::args().skip(1) {
        // from_str 은 결과값의 성공여부를 반환 Ok(v) or Err(e)
        // expect를 통해 Ok(v)가 확인되면 v를 반환, 실패하면 Err(e)의 메세지 e를 출력하고 종료
        // 레퍼런스를 참조해보니 from_str(s: &str) 형태다. 따라서 &arg를 사용한다.
        numbers.push(u64::from_str(&arg)
                        .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        // 오류와 함께 프로그램을 종료해야한다. eprintln! 매크로로 오류 출력 스트림에 오류 메세지를 기록한다.
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // 러스트에게 벡터의 소유권이 계속해서 numbers에 남도록한다. 잠시 빌려오기만 한 것이다.
    // 덕분에 numbers가 main의 범위 끝을 벗어나면 자동으로 해제된다.
    // ㄴ 만약 numbers[1..]을 사용했다면? numbers[1..]는 누구의 소유가 되는가?
    for m in &numbers[1..] {
        // 레퍼런스가 m에 남아있기 때문에 (이터레이터?) 역참조(*)를 수행해준다.
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}",
             numbers, d);

}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t: u64 = m; // 지역변수는 let으로 선언한다. 반복문이 수행될 때 마다 선언되므로 mut 불필요
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
                3 * 11);
}