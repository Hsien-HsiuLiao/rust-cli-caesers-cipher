use std::io;

fn main() {
    /*
    

  rot13("SERR PBQR PNZC") should decode to the string FREE CODE CAMP

rot13("SERR CVMMN!") should decode to the string FREE PIZZA!

rot13("SERR YBIR?") should decode to the string FREE LOVE?

rot13("GUR DHVPX OEBJA SBK WHZCF BIRE GUR YNML QBT.") should decode to the string THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.
    */


    let mut encoded = String::new();
    println!("Enter a ROT13 encoded string");

    io::stdin()
        .read_line(&mut encoded)
        .expect("Failed to read line");

    println!("The decoded string is: {}", decode(&encoded));
}

fn decode(input: &String) -> String {
    let mut decoded = String::new();
    decoded = input.to_string();
    /* function rot13(str) { // LBH QVQ VG!
        var num;
        var newstr="";
        var decoded="";
        for (let i=0;i<str.length ;i++){ //iterate over length of string input str
          num= str.charCodeAt(i);       // get unicode of string at index i and store in num
       // num = num-13; a=65 Z=90
          if (num>77 && num <91){
        decoded += String.fromCharCode(num-13);
          //decoded += newstr;
          }
          else if (num>64 && num <78){
        decoded += String.fromCharCode(num+13);
          //decoded += newstr;
          }
          else {
              decoded += String.fromCharCode(num);
      
          }  
        }
        return decoded;
        
      }
      */
      decoded
}
