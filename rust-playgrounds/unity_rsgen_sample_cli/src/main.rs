// このファイルは、プロジェクトルートで実行されるコードを含みます

fn main() {

    let add = unity_rsgen_sample::my_add(1, 2);
    //print result add
    println!("add result: {}", add);

    let sub = unity_rsgen_sample::my_sub(1, 2);
    println!("sub result: {}", sub);

    let multi = unity_rsgen_sample::my_multiply(11, 2);
    //print result add
    println!("multiply result: {}", multi);

    let div = unity_rsgen_sample::my_divide(55, 11);
    println!("divide result: {}", div);

}
