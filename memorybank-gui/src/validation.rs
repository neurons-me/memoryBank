use std::collections::HashSet;

pub fn validate_combinations(
    copy: [&str; 3],
    paste: [&str; 3],
) -> Vec<String> {
    let mut errors = vec![];

    if copy.iter().filter(|m| *m == &"None").count() > 1 {
        errors.push("Copy: Only one 'None' is allowed.".to_string());
    }

    if paste.iter().filter(|m| *m == &"None").count() > 1 {
        errors.push("Paste: Only one 'None' is allowed.".to_string());
    }

    let copy_set: HashSet<_> = copy.iter().filter(|m| **m != "None").collect();
    let paste_set: HashSet<_> = paste.iter().filter(|m| **m != "None").collect();

    if copy_set == paste_set {
        errors.push("Copy and Paste cannot use the same combination.".to_string());
    }

    errors
}