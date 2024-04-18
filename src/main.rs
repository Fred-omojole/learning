fn main() {
    //  QUESTIONS

    // let val1 = 5;
    // let val2 = 2;
    // let ans = val1 % val2;
    // println!("{:?}", ans )

    // let mut v: Vec<i32> = Vec::new();

    // v.push(2);
    // v.push(4);
    // v.push(6);
    // v.push(8);
    // v.push(10);
    // print!("{:?}", v);

    // v.pop();

    // print!("{:?}", v);

    // v.push(12);
    // print!("{:?}", v);

    let mut vec = vec![""];
    vec.pop();

    println!("{:?}", vec);

    // let vecc = vec.pop();
    // println!("{:?}",vecc);

    vec.push("");
    println!("{:?}", vec);

    // match  vecc  {
    //     Some(value) => println!("{:?}", value ),
    //    _None => println!("")
    // }

    // let mut v : Vec<i128> = (0..100000).collect();
    // print!("{:?}",v);

    let str1 = String::from("Hello");
    let ans = concat_string(str1);
    println!("{}", ans)
}

fn concat_string(val: String) -> String {
    val + " World!"
}
