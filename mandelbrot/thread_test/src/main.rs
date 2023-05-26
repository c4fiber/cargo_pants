fn main() {
    let mut var: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    crossbeam::scope(|spawner| {
        let length = 3;
        let ranges: Vec<&mut [u8]> = var.chunks_mut(length).collect();

        for (i, range) in ranges.into_iter().enumerate() {
            spawner.spawn(move |_| {
                // range에 해당하는 부분이 한번에 실행되도록 의도했다.
                // 덕분에 한 range가 출력되는 도중에 다른 range의 값이 출력되는 경우를 볼 수 없다.
                for j in range {
                    println!("range[{}] A child thread borrowing `var`: {}", i, j);
                }

            });
        }

    }).unwrap();
}
