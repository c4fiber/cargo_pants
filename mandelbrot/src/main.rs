use num::Complex;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::env;

// ///로 시작하는 주석은 문서 주석이다. 이후에 파싱해서 온라인 문서를 만들 수 있다.
/// `C`가 망델브로 집합게 속하는지 안니지를 판단하며, 결론 내리는데 필요한 반복횟수는 최대 `limit`회로 제한한다.
///
/// `C`가 망델브로 집합에 속하지 않으면 `Some(i)`를 반환하는데
/// 여기서 `i`는 `c`가 원점을 중심으로 반경이 2인 원을 벗어나는데 걸린 반복 횟수다.
/// `c`가 망델브로 집합에 속하는 것 같으면
/// (좀더 정확히 말해서 반복횟수가 `limit`이 될 때까지도 `c`가 망델브로 집합에 속하지 않는다는걸 입증하지 못하면)
/// `None`을 반환한다.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    // 0 부터 limit까지, limit은 포함하지 않는다. (이상, 미만)
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i)
        }
        z = z * z + c
    }

    None
}

/// `s`를 `"400x600"`이나 `"1.0,0.5"`와 같은 좌표 쌍으로 파싱한다.
///
/// `s`는 정확히 <left><sep><right> 형식으로 되어 있어야 하는데, 여기서 <sep>은
/// `separator` 인수에 넘기는 문자이고 <left>와 <right>는 둘 다 `T::from_str`로
/// 파싱될 수 있는 문자열이다. `separator`는 반드시 ascii 문자여야 한다.
///
/// `s`가 올바른 형식으로 되어 있으면 `Some<(x,y)>`를 반환한다. 제대로 파싱되지 않으면
/// `None`을 반환한다.
// parse_pair 는 제네릭함수(generic function)이다.
// T: FromStr 은 'FromStr 트레이트를 구현하고 있는 임의의 타입 T에 대해서...' 라고 읽으면 된다.
// T를 parse_pair의 타입 매개변수(type parameter)라고 부른다.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        // Some(T) 는 T 타입의 특정한 값을 의미한다.
        // s.find(separator)의 결과가 <sep>의 인덱스 값인데 이걸 Some(index)로 받아온다.
        Some(index) => {
            // T::from_str(&str)을 통해서 str의 값을 T 타입의 값으로 파싱한다.
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                // 와일드카드 패턴 _은 어떤 것과도 매칭 될 수 있지만 그 값은 무시된다.
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",          ','), None);
    assert_eq!(parse_pair::<i32>("10,",       ','), None);
    assert_eq!(parse_pair::<i32>(",10",       ','), None);
    assert_eq!(parse_pair::<i32>("10,20",     ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy",   ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",      'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",   'x'), Some((0.5, 1.5)));
}

/// 쉼표로 구분된 부동소수점 수 쌍을 복소수로 파싱한다.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"),
                                Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None)
}

/// 결과 이미지의 픽셀 좌표가 주어지면 여기에 대응하는 복소평면 위의 점을 반환한다.
///
/// `bounds`는 픽셀 단위로 된 이미지의 폭과 높이를 갖는 쌍이다. `pixel은 이미지의 특정
/// 픽셀을 나타내는 (열, 행)으로 된 상이다. `upper_left`와 `lower_left` 매개변수는
/// 이미지가 커버하는 영역을 지정하는 복소평면 위의 두 점이다.
fn pixel_to_point(bounds: (usize, usize),
                    pixel: (usize, usize),
                    upper_left: Complex<f64>,
                    lower_right: Complex<f64>)
                -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re,
                            upper_left.im - lower_right.im);
    Complex {
        // pixel.0은 튜플의 첫번째 값을 의미한다.
        // as f64는 Rust의 타입 변환 문법이다.
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        // 여기서 뺄셈을 하는 이유는 pixel.1의 경우 아래로 갈수록 증가하지만
        // 허수 부분은 위로 갈수록 증가하기 때문이다.
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point((100,200), (25, 175),
                    Complex { re: -1.0, im: 1.0 },
                    Complex { re: 1.0, im: -1.0}),
                Complex { re: -0.5, im: -0.75 });
}

/// 직사각형 모양의 망델브로 집합을 픽셀 버퍼에 렌더링한다.
///
/// `bounds` 인수는 한 바이트 안에 회색조로 된 픽셀 하나가 들어가는 `pixels` 버퍼의 폭과
///  높이를 갖는다. `upper-left`와 `lower_right` 인수는 픽셀 버퍼의 왼쪽 위 모서리와
/// 오른쪽 아래 모서리에 해당하는 복소평면 위의 두 점을 지정한다.
fn render(pixels: &mut [u8],
            bounds: (usize, usize),
            upper_left: Complex<f64>,
            lower_right: Complex<f64>)
{
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row),
                                        upper_left, lower_right);
            pixels[row * bounds.0 + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}

/// `bounds` 크기의 `pixels` 버퍼를 `filename` 파일에 기록한다.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize))
    -> Result<(), std::io::Error>
{
    // ? 연산자를 이 패턴 전체에 대한 축약 표기로 설정되어있다. 일일이 적을 필요도 없고 읽기 좋다.
    // let output = match File::create(filename) {
    //     Ok(f) => f,
    //     Error(e) => { return Err(e); }
    // };
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(pixels,
                    bounds.0 as u32, bounds.1 as u32,
                    ColorType::Gray(8))?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT",
                    args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
                    args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3])
        .expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4])
        .expect("error parsing lower right corner point");
    // vec![v; n]은 요소가 모두 v로 초기화된 길이 n의 벡터를 만든다.
    let mut pixels = vec![0; bounds.0 * bounds.1];

    /* 싱글 스레드 사용 부분 */
    // &mut pixels 표현식은 픽셀 버퍼의 변겨할 수 있는 레퍼런스를 빌려온다.
    // render(&mut pixels, bounds, upper_left, lower_right);

    /* 멀티 스레드 사용 부분 */
    let threads = 8;
    let rows_per_band = bounds.1 / threads + 1;

    {
        // chunks_mut 메서드는 버퍼에 있는 픽셀을 rows_per_band * bounds.0개씩 겹치지 않게 나눈다음
        // 이들 각각을 변경할 수 있는 슬라이스를 넘겨주는 이터레이터를 반환한다.
        // collect() 메서드는 이 슬라이스들을 벡터에 담아서 반환한다.
        let bands: Vec<&mut [u8]> =
            pixels.chunks_mut(rows_per_band * bounds.0).collect();
        // |spawner| {...}는 인수 spawner 하나를 받는 러스트 클로저다
        // (이름이 없고 필요할때 즉석에서 생성되는 동적 함수)
        // crossbeam::scope() 함수는 복귀 하기 전에 이를 통해 생성된 스레드가 모든 실행을 마칠때 까지 기다린다.
        // 러스트는 범위를 한 번 벗어난 스레드가 자신이 맡았던 pixels영역에 다시 접근하는 일이 생기지 않는다고 확신
        // 사용자는 crossbeam::scope()가 복귀하는 순간 이미지 계산이 완료되었다고 간주할 수있다.
        // crossbeam::scope()의 반환값은 Ok(()) 혹은 Err(e)를 반환하므로 unwrap() 호출해서
        // 스레드가 패닉에 빠졌을 때(한번이라도 스레드가 실패하면 패닉) 프로그램도 같이 패닉에 빠지도록 한다.
        crossbeam::scope(|spawner| {
            // into_iter() 메서드를 통해서 루프 본문을 반복할 때 마다 개별 band의 독점 소유권을 넘겨준다
            // ㄴ 덕분에 한 번에 하나의 스레드만 그곳에 기록할 수 있다.
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / bounds.0;
                let band_bounds = (bounds.0, height);
                let band_upper_left =
                    pixel_to_point(bounds, (bounds.0, top), upper_left, lower_right);
                let band_lower_right =
                    pixel_to_point(bounds, (bounds.0, top + height),
                                    upper_left, lower_right);

                // move 키워드는 이 클로저가 자신이 사용하는 변수들의 소유권을 갖는다는 것을 나타낸다.
                // |_|는 이 클로저가 인수(중첩 스레드를 만들 때 쓰는 다른 스퍼너)를 받지만 사용하지 않는다는 뜻.
                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        }).unwrap();
    }

    write_image(&args[1], &pixels, bounds)
        .expect("error writing PNG file");
}