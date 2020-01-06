pub fn verse(n: u32) -> String {
    let ret : String;
    if n == 2{
    ret = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.\n", n, n, n-1);
    }
    else if n == 1{
    ret = format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n", n, n);
    }
    else if n == 0{
    ret = format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    else{ret=format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n", n, n, n-1);}
    return ret;
}
pub fn sing(start: u32, end: u32) -> String {
    let ret:String=verse(start);
    if start > end{
        return format!("{}\n{}",ret,sing(start-1,end));
    }
    else{
    return ret;
    }
}
