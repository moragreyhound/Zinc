//STRUCTS
struct Human {
    username: String,
    email: String,
    is_active: bool,
    login_count: u32,
}
//end of structs

fn main() {
    //ascii
    // --- 1. Standard & Bright Foreground (Text) Colors ---
    let fgblack    = "\x1b[30m";
    let fgred      = "\x1b[31m";
    let fggreen    = "\x1b[32m";
    let fgyellow   = "\x1b[33m";
    let fgblue     = "\x1b[34m";
    let fgmagenta  = "\x1b[35m";
    let fgcyan     = "\x1b[36m";
    let fgwhite    = "\x1b[37m";

    // Bright / Light Foreground Variants
    let fglblack   = "\x1b[90m";
    let fglred     = "\x1b[91m";
    let fgllgreen  = "\x1b[92m";
    let fglyellow  = "\x1b[93m";
    let fglblue    = "\x1b[94m";
    let fgpink     = "\x1b[95m";
    let fglcyan    = "\x1b[96m";
    let fglwhite   = "\x1b[97m";

    // --- 2. Standard & Bright Background Colors ---
    let bgblack    = "\x1b[40m";
    let bgred      = "\x1b[41m";
    let bggreen    = "\x1b[42m";
    let bgyellow   = "\x1b[43m";
    let bgblue     = "\x1b[44m";
    let bgmagenta  = "\x1b[45m";
    let bgcyan     = "\x1b[46m";
    let bgwhite    = "\x1b[47m";

    // Bright Background Variants
    let bglblack   = "\x1b[100m";
    let bglred     = "\x1b[101m";
    let bgllgreen  = "\x1b[102m";
    let bglyellow  = "\x1b[103m";
    let bglblue    = "\x1b[104m";
    let bgpink     = "\x1b[105m";
    let bglcyan    = "\x1b[106m";
    let bglwhite   = "\x1b[107m";
    
    // Fixed escape sequences for RGB backgrounds
    let bgorange   = "\x1b[48;2;255;165;0m";
    let bgindigo   = "\x1b[48;2;75;0;130m";
    let bgviolet   = "\x1b[48;2;143;0;255m";

    // --- 3. Text Styling Modifiers ---
    let cbold      = "\x1b[1m";
    let cdim       = "\x1b[2m";
    let citalic    = "\x1b[3m";
    let cunderline = "\x1b[4m";
    let cblink     = "\x1b[5m";
    let creverse   = "\x1b[7m";
    let chidden    = "\x1b[8m";
    let cstrike    = "\x1b[9m";
    
    //reset
    let creset     = "\x1b[0m";
    //end

    println!("Hello, world!");
    let mut x = 10;
    println!("{x}");
    println!("{}", x);
    let x = 7;
    println!("{x}");

    //conditions
    if x > 9 {
        println!("Hello World!");
    } else {
        println!("evil hello world!");
    }

    //lets create a human
    let mut linus = Human {
        username: String::from("linus"),
        email: String::from("torvalds@linux-foundation.org"),
        is_active: true,
        login_count: 0,
    };
    
    println!("{} has logged in {} times!", linus.username, linus.login_count);
    linus.login_count += 1;
    
    println!("{fgred}ERRRORRRRRRRR!!!!{creset}");
    println!("{cbold}{bgred}{fgcyan}Hello?{creset}");
    
    //rainbow text <3
    println!("{bgred}R{bgorange}a{bgyellow}i{bggreen}n{bgblue}b{bgindigo}o{bgviolet}w{creset}");
    //niceee
    
    // --- COLOR GRID SHOWCASE ---
    println!("\n{cbold}{cunderline}--- ANSI COLOR & STYLE GRID ---{creset}\n");

    // Arrays of foregrounds and their names for display
    let fg_colors = [
        ("Black", fgblack), ("Red", fgred), ("Green", fggreen), ("Yellow", fgyellow),
        ("Blue", fgblue), ("Magenta", fgmagenta), ("Cyan", fgcyan), ("White", fgwhite),
        ("Light Black", fglblack), ("Light Red", fglred), ("Light Green", fgllgreen), ("Light Yellow", fglyellow),
        ("Light Blue", fglblue), ("Pink", fgpink), ("Light Cyan", fglcyan), ("Light White", fglwhite),
    ];

    // Arrays of backgrounds and their names
    let bg_colors = [
        ("Black", bgblack), ("Red", bgred), ("Green", bggreen), ("Yellow", bgyellow),
        ("Blue", bgblue), ("Magenta", bgmagenta), ("Cyan", bgcyan), ("White", bgwhite),
        ("L.Black", bglblack), ("L.Red", bglred), ("L.Green", bgllgreen), ("L.Yellow", bglyellow),
        ("L.Blue", bglblue), ("Pink", bgpink), ("L.Cyan", bglcyan), ("L.White", bglwhite),
        ("Orange", bgorange), ("Indigo", bgindigo), ("Violet", bgviolet),
    ];

    println!("{cbold}1. Foreground Colors:{creset}");
    for (name, color) in fg_colors.iter() {
        print!("{color} █ {name:<12} {creset}");
    }
    println!("\n");

    println!("{cbold}2. Background Colors (with dark text):{creset}");
    for (name, color) in bg_colors.iter() {
        print!("{color}{fgblack}  {name:<10}  {creset} ");
    }
    println!("\n");

    println!("{cbold}3. Text Styling Modifiers:{creset}");
    println!("{cbold}Bold Text{creset}");
    println!("{cdim}Dim Text{creset}");
    println!("{citalic}Italic Text{creset}");
    println!("{cunderline}Underlined Text{creset}");
    println!("{creverse}Reversed Colors Text{creset}");
    println!("{cstrike}Strikethrough Text{creset}");
}
