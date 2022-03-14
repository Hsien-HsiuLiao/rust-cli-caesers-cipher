use std::io;

fn main() {
    let mut encoded = String::new();
    println!("Enter a ROT13 encoded string:");

    io::stdin()
        .read_line(&mut encoded)
        .expect("Failed to read line");

    println!("The decoded string is: {}", decode(&encoded));
}

fn decode(input: &String) -> String {
    let mut decoded = String::new();
    
    for mut b in input.bytes(){
        //println!("bytes: {}", b);
        match b {
            78..=90 => b = b - 13,
            64..=77 => b = b + 13,
            _ => ()
        }
        decoded.push(b as char);
    }
    
    /*
    for c in input.chars() {
       // println!("{}", c);
       match c {
        'N'..='Z' => decoded.push( (c.to_string().bytes() - 13) as char),
        'A'..='M' => decoded.push( c),
        _ => decoded.push( c)
        }

    }
    */
    /* 
       // A=65 Z=90
      */
      decoded
}
   /*
  rot13("SERR PBQR PNZC") should decode to the string FREE CODE CAMP
rot13("SERR CVMMN!") should decode to the string FREE PIZZA!
rot13("SERR YBIR?") should decode to the string FREE LOVE?
rot13("GUR DHVPX OEBJA SBK WHZCF BIRE GUR YNML QBT.") should decode to the string THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.
    */