fn main() {
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(7..8, "R");
    dbg!(string_replace_range);

    let mut primer_f = String::from("5\'-AGCTGATCGATGCTAGCTGA-3\'");
    // dbg!(primer_f); // 此处无法通过编译
    println!("primer_f: {}", primer_f);
    println!("len: {}", primer_f.len());
    primer_f.replace_range(6..7, "X");
    dbg!(primer_f);
    let mut primer_r = String::from("5'-TCAGCTAGCATCGATCAGCT-3'"); // 删去反斜杠也可以
    println!("primer_r: {}", primer_r);
    println!("len: {}", primer_r.len());
    primer_r.replace_range(3..23, "X");
    dbg!(primer_r);
}
