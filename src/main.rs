use std::io;

fn main() {
    let mut map: [[u8; 15]; 15] = [[0; 15]; 15];
    let mut turn: u8 = 1;

    loop {
        let mut coor = String::new();

        io::stdin().read_line(&mut coor).expect("좌표 읽기 오류");
        let coorvec: Vec<usize> = coor.split_whitespace()
                                    .map(|s| s.trim().parse().expect("좌표 파싱 오류"))
                                    .collect::<Vec<_>>();

        if coorvec.len() != 2 {
            println!("좌표는 두 개의 숫자를 입력해야 합니다!");
            continue;
        }

        let x: usize = coorvec[0];
        let y: usize = coorvec[1];

        if x >= 16 || y >= 16 || x <= 0 || y <= 0 {
            println!("좌표는 1에서 15 사이여야 합니다!");
            continue;
        }

        if map[y - 1][x - 1] != 0 {
            println!("이 칸에는 수를 둘 수 없습니다!");
            continue;
        } else {
            map[y - 1][x - 1] = turn;
            if checky_win(&map, y - 1, x - 1) {
                println!("{}픽 승리!", turn);
                break;
            }
            turn = if turn == 1 { 2 } else { 1 };
        }

        println!();
        let box_str: String = "🟥".to_string();
        print!("{}", box_str.repeat(17));
        println!();
        for y in map.iter() {
            print!("🟥");
            for &x in y.iter() {
                print!("{}", if x == 0 { "➕" } else { if x == 1 { "⚫" } else { "⚪" }});
            }
            print!("🟥");
            println!();
        }
        println!("{}", box_str.repeat(17));
    }

    println!("게임이 종료되었습니다.");
}

fn checky_win(map: &[[u8; 15]; 15], x: usize, y: usize) -> bool { // 구현 오류. x자리에 y, y자리에 x를 대입할 것.
    let turn = map[x][y];
    if // 역슬래쉬 방향 대각선 체크
        (x > 1 && x <= 12 && y > 1 && y <= 12 && map[x-1][y-1] == turn && map[x-2][y-2] == turn && map[x+1][x+1] == turn && map[x+2][y+2] == turn) // ooxoo
        || (x > 2 && x <= 11 && y > 2 && y <= 11 && map[x-1][y-1] == turn && map[x+1][y+1] == turn && map[x+2][y+2] == turn && map[x+3][y+3] == turn) // oxooo
        || (x <= 10 && y <= 10 && map[x+4][y+4] == turn && map[x+1][y+1] == turn && map[x+2][y+2] == turn && map[x+3][y+3] == turn) // xoooo
        || (x > 2 && x <= 13 && y > 2 && y <= 13 && map[x-1][y-1] == turn && map[x+1][y+1] == turn && map[x-2][y-2] == turn && map[x-3][y-3] == turn) // oooxo
        || (x > 3 && x <= 14 && y > 3 && y <= 14 && map[x-1][y-1] == turn && map[x-4][y-4] == turn && map[x-2][y-2] == turn && map[x-3][y-3] == turn) // oooox
    {
        return true;
    } else if // 슬래쉬 방향 대각선 체크
        (x > 1 && x <= 12 && y > 1 && y <= 12 && map[x-1][y+1] == turn && map[x-2][y+2] == turn && map[x+1][x-1] == turn && map[x+2][y-2] == turn) // ooxoo
        || (x > 0 && x <= 11 && y >= 2 && y <= 12 && map[x-1][y+1] == turn && map[x+1][y-1] == turn && map[x+2][y-2] == turn && map[x+3][y-3] == turn) // oxooo
        || (x <= 10 && y >= 4 && map[x+4][y-4] == turn && map[x+1][y-1] == turn && map[x+2][y-2] == turn && map[x+3][y-3] == turn) // xoooo
        || (x > 2 && x <= 13 && y > 0 && y <= 11 && map[x-1][y+1] == turn && map[x+1][y-1] == turn && map[x-2][y+2] == turn && map[x-3][y+3] == turn) // oooxo
        || (x > 3 && x <= 14 && y <= 10 && map[x-1][y+1] == turn && map[x-4][y+4] == turn && map[x-2][y+2] == turn && map[x-3][y+3] == turn) // oooox
    {
        return true;
    }

    false
}