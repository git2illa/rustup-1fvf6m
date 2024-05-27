fn main() {
    let five = Some(5);
    let six = plus_on(five);
    let none = plus_on(None);

}

fn plus_on(x: Option<i32>) -> Option<i32>{
    match x {
        None => {
            None
        },
        Some(i) => Some(i+1)
    }
}
//fn match_some(x: Option<u8>) -> Option<u8>{
//    match x {
//        1 => print!("one"), 
//       3 => print!("three"), 
//      5 => print!("five"), 
//     _ => print!(), 
//}
//}

