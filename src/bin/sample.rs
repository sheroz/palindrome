fn main() {
    let sample1 = "A man, a plan, a canal: Panama!";
    println!("Check for palindrome: {}\n{}\n", sample1, palindrome::is_palindrome(sample1));

    let sample2 = "man, plan, canal: Panama!";
    println!("Check for palindrome: {}\n{}\n", sample2, palindrome::is_palindrome(sample2));

}