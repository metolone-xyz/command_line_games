use std::io;
use rand::Rng;

fn main() {
    println!("棒が同列に並んだ場合、常に左側が高いとする");
    show_amida();
}

fn show_amida(){

    //正解の番号
    let answer: usize = rand::thread_rng().gen_range(0..6);
    let mut posision: [u32; 6] = [0; 6];

    //当たりくじの場所に１を代入
    posision[answer] = 1;

    for _i in 0..6{
        print!("{}  ",_i);
    }
    //show_array(&mut posision);
    println!(" ");
    //あみだのルートを生成
    let mut i = 0;
    while i != 10 {
        for j in 0..6{
            let horizontal_bar = rand::thread_rng().gen_range(0..4);

            //最後の行には横棒を追加しない
            if j != 5{
                //もし乱数が1なら横棒を追加
                if horizontal_bar == 1{
                    print!("|--");
                    //となりへ移動した分交換
                    posision.swap(j, j+1);
                }else {
                    print!("|  ");
                }
            }else{
                //最後の列には横棒を追加しない
                print!("|  ");
            }

        } 
        //show_array(&mut posision);
        println!(" ");
        i += 1;
    }

    for _i in posision{
        if _i == 1{
            print!("O  ");
        }else{
            print!("X  ");
        } 
    }
    println!(" ");


    //ユーザーから入力を受け取る
    println!("あたりの籤の番号を入力してください");
    let user_input = get_input();
    if user_input == answer {
        println!("あたり!!");
    }else{
        println!("はずれ");
    }
}

fn get_input() -> usize{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");
    let input: usize = input.trim().parse().expect("変換エラー");
        input
}
fn show_array(a: &mut [u32;6]){//配列の中身を表示
    for i in a{
        print!("{}",i);
    }
    print!(" ");
    //配列の中身を表示
}
