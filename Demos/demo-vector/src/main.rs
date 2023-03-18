fn main() {
    // let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    // // let mut v_1: Vec<i32>  = Vec::new();
    // 
    // for i in 0..v.len() {
    //     println!(".. len: {}", v[i]);
    // }
    // // 向 vector 末尾新增数据
    // v.push(32);
    //
    //
    // // 通过 vector 切片访问数据
    // for i in &v {
    //     println!("& : {}", i);
    // }
    //
    // // 通过 vector 下标移除一项
    // v.remove(3);
    //
    // for i in v.iter() {
    //     println!("iter : {}", i);
    // }
    //
    // println!("{} / {}", v[0], v.get(1).unwrap());
    //
    // // dedup 
    // // let v_2: Vec<i32> = vec![3,2,1,5,0,0,0];
    // // let result = v.into_iter().chain(v_2.into_iter()).collect()<Vec<i32>>();
    // // 
    // // println!("{:?}", result);
    //
    // let v_3: Vec<u32> = Vec::with_capacity(5);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        i += 50;
    }

    print!("{:?}", v);
    
}


