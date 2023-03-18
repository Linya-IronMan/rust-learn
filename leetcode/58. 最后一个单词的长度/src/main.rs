fn main() {
   let s = "Hello World";
   let arr: Vec<&str> = s.split_whitespace().collect();
   let num = arr[arr.len() - 1].len() as i32;
   println!("{}", num);
}
