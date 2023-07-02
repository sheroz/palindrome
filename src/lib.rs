use regex::Regex;
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

pub fn is_palindrome(sample: &str) -> bool {
    is_palindrome_raw(sample)
}

pub fn is_palindrome_regex(sample: &str) -> bool {
    // make all characters lower case
    let txt = sample.to_string().to_lowercase();

    // regex for removing non-word characters
    let re = Regex::new(r"[\W]").unwrap();

    // removing non-word characters
    let clean_txt = re.replace_all(&txt, "").to_string();

    // reverse the text
    let reverse_txt: String = clean_txt.graphemes(true).rev().collect();

    // compare directions
    clean_txt == reverse_txt
}

pub fn is_palindrome_raw(sample: &str) -> bool {
    // make all characters lower case
    let mut txt = sample.to_string().to_lowercase();

    // remove excluded symbols
    let excluded_symbols = [" ", ".", ",", "!", ":", ";", "'", "’", "\"", "-"];
    for c in excluded_symbols {
        txt = txt.replace(c, "");
    }

    let mut iterator = txt.graphemes(true).into_iter();
    loop {
        let char_start = iterator.next();
        if char_start.is_none() {
            break;
        }

        let char_end = iterator.next_back();
        if char_end.is_none() {
            break;
        }

        if char_start != char_end {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn palindrome_test() {
        use super::*;

        // palindome samples
        assert!(is_palindrome("Madam, I'm Adam!"));
        assert!(is_palindrome("A man, a plan, a canal: Panama!"));
        assert!(is_palindrome("Dammit I’m mad."));
        assert!(is_palindrome("Engage le jeu que je le gagne."));
        assert!(is_palindrome("I topi non avevano nipoti."));
        assert!(is_palindrome("Autore, ero tua."));
        assert!(is_palindrome(
            "Socorram-me subi no onibus em Marrocos."
        ));
        assert!(is_palindrome("A mala nada na lama."));
        assert!(is_palindrome("А тот суп – пустота."));
        assert!(is_palindrome("Нажал кабан на баклажан."));

        // non-palindome samples
        assert!(!is_palindrome("Lorem ipsum dolor sit amet,"));
        assert!(!is_palindrome("consectetur adipiscing elit,"));
        assert!(!is_palindrome(
            "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
        ));
        assert!(!is_palindrome("A plan, a man, a canal: Panama!"));
        assert!(!is_palindrome("Нажал баклажан на кабан."));
    }
}
