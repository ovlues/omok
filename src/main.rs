use std::io;

fn main() {
    loop {
        let end = false;

        let mut coor = String::new();
        io::stdin().read_line(&mut coor).expect("좌표 읽기 오류");
        let coorvec:Vec<i16> = coor.split_whitespace()
                                    .map(|s| s.trim().parse().expect("좌표 파싱 오류"))
                                    .collect::<Vec<_>>();
    
        println!("{:?}", coorvec);
        let mut x: &i16 = &coorvec[0];
        let mut y: &i16 = &coorvec[1];

        if end {
            break;
        }
    }

    println!("Hello, world!");
}
