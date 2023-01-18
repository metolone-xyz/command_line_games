//use std::io;
use rand::Rng;

fn main() {
    show_amida();
}

fn show_amida(){
    let answer = rand::thread_rng().gen_range(0..6);//正解の番号
    let mut posision: [i32; 6] = [0; 6];
    posision[answer] = 1;//当たりくじの場所に1を代入

    for _i in 0..6{
        print!("{}  ",_i);
    }
    show_array(&mut posision);
    println!(" ");
    for i in 0..4{
        for j in 0..6{
            let horizontal_bar = rand::thread_rng().gen_range(0..5);
            if j != 5{
                if horizontal_bar == 1{
                    print!("|--");
                    posision.swap(j, j+1);//となりへ移動した分交換
                }else {
                    print!("|  ");
                }}else{
                    print!("|  ");
                }

        } 
        show_array(&mut posision);
        println!(" ");
    }
}

fn show_array(a: &mut [i32;6]){
    for i in a{
        print!("{}",i);
    }
    print!(" ");
    //配列の中身を表示
}
