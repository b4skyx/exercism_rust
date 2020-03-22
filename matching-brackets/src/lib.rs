use std::collections::HashMap;
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets = HashMap::new();
    brackets.insert('}' , '{');
    brackets.insert(')' , '(');
    brackets.insert(']' , '[');
    let mut stack : Vec<char> = Vec::new();
    for ch in string.chars(){
        if brackets.values().collect::<Vec<_>>().contains(&&ch){
            stack.push(ch);
        }
        else if brackets.keys().collect::<Vec<_>>().contains(&&ch){
            if stack.last().unwrap_or(&'x') == brackets.get(&ch).unwrap(){
                stack.pop();
            }
            else{
                return false;
            }
        }
    }
    if stack.is_empty(){
        return true;
    }
    false
}
