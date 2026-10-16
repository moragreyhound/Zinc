    //STRUCTS
struct Human{
        username:String,
        email:String,
        is_active:bool,
        login_count:u32,
    }
    //end of structs


//
fn main() {
    println!("Hello, world!");
    let mut x = 10;
    println!("{x}");
    println!("{}",x);
    let  x = 7;
    println!("{x}");

    //conditions
    if x>9 {

    println!("Hello World!");
}//end of if 
    else {
        println!("evil hello world!");
    }//end of else
    //
//lets create a human
let mut linus = Human {
        username:String::from("linus"),
        email: String::from("torvalds@linux-foundation.org"),
        is_active:true,
        login_count:0,
    };//end of custom struct
println!("{} has logged in {} times!",linus.username,linus.login_count);
    linus.login_count += 1;
    //end of file
}
