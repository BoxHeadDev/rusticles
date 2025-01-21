fn main() {
    // Why is the program unsafe?
    return_a_string();

    ownership_error1();

    add_big_strings();

    ownership_error2();

    ownership_error3();

    ownership_error4();

    ownership_error5();

    ownership_error6();

    ownership_error7();

    ownership_error8();

    ownership_error9();
}

fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}

fn ownership_error1() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}

// ideally: ["Ferris", "Jr."] => "Ferris Jr. Esq."
fn stringify_name_with_title(name: &Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
}

fn ownership_error2() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    let s: String = *s_ref;
}

fn ownership_error3() {
    let mut n = 0;
    let a = &mut n;
    let b = a;
}

fn ownership_error4() {
    let mut name = (String::from("Ferris"), String::from("Rustacean"));
    let first = get_first(&name);
    name.1.push_str(", Esq.");
    println!("{first} {}", name.1);
}

fn get_first(name: &(String, String)) -> &String {
    &name.0
}

fn ownership_error5() {
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];
    let y = &a[2];
    *x += *y;
}

fn ownership_error6() {
    let s = String::from("Hello world");
    let s_ref = &s;
    let s2 = *s_ref;
    println!("{s2}");
}

fn ownership_error7() {
    let mut v = vec![1, 2, 3];
    copy_to_prev(&mut v, 1);
}

fn copy_to_prev(v: &mut Vec<i32>, i: usize) {
    let n = &mut v[i];
    *n = v[i - 1];
}

fn ownership_error8() {
    let name = String::from("Ferris");
    let name_ref = &name;
    award_phd(&name);
    println!("{}", name_ref);
}

fn award_phd(name: &String) {
    let mut name = *name;
    name.push_str(", Ph.D.");
}

fn ownership_error9() {
    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1;
    *y += 1;
    println!("{} {}", point[0], point[1]);
}
