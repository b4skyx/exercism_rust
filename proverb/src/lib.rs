pub fn build_proverb(list: &[&str]) -> String {
    let len = list.len();
    let mut lyr: String = String::new();
    if len != 0 {
        for i in 0..len - 1 {
            lyr.push_str(&format!(
                "For want of a {} the {} was lost.\n",
                list[i],
                list[i + 1]
            ));
        }
        lyr.push_str(&format!("And all for the want of a {}.", list[0]));
    }
    return lyr;
}
