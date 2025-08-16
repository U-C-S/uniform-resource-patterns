// Order of delimiters in a URI:
// (SCHEME_PATH | SCHEME_AUTHORITY)
// + PATH?
// + (PRE_QUERY + QUERY?)?
// + PRE_FRAGMENT?

use crate::primitives::Delimiter;

fn validate_delimiter_order(demlimiter_order: &Vec<Delimiter>) -> Result<(), &'static str> {
    let mut last_delimiter: Option<&Delimiter> = None;

    for delimiter in demlimiter_order {
        match delimiter {
            Delimiter::SCHEME_PATH | Delimiter::SCHEME_AUTHORITY => {
                if last_delimiter.is_some() {
                    return Err("Scheme delimiters must be at the start of the URI.");
                }
            }
            Delimiter::PATH => {
                if ![
                    Delimiter::SCHEME_PATH,
                    Delimiter::SCHEME_AUTHORITY,
                    Delimiter::PATH,
                ]
                .contains(last_delimiter.unwrap())
                {
                    return Err("Path delimiter must follow scheme delimiters.");
                }
            }
            Delimiter::PRE_QUERY => {
                if ![
                    Delimiter::SCHEME_PATH,
                    Delimiter::SCHEME_AUTHORITY,
                    Delimiter::PATH,
                ]
                .contains(last_delimiter.unwrap())
                {
                    return Err("Query delimiter must follow path delimiter.");
                }
            }
            Delimiter::QUERY => {
                if ![Delimiter::PRE_QUERY, Delimiter::QUERY].contains(last_delimiter.unwrap()) {
                    return Err("Query delimiters must follow path delimiter.");
                }
            }
            Delimiter::PRE_FRAGMENT => {
                if ![
                    Delimiter::SCHEME_PATH,
                    Delimiter::SCHEME_AUTHORITY,
                    Delimiter::PATH,
                    Delimiter::PRE_QUERY,
                ]
                .contains(last_delimiter.unwrap())
                {
                    return Err("Fragment delimiter must follow path or query delimiters.");
                }
            }
        }
        last_delimiter = Some(delimiter);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_delimiter_order() {
        let valid_order = vec![
            Delimiter::SCHEME_PATH,
            Delimiter::PATH,
            Delimiter::PATH,
            Delimiter::PRE_QUERY,
            Delimiter::QUERY,
            Delimiter::PRE_FRAGMENT,
        ];
        let validatation_result = validate_delimiter_order(&valid_order);
        // if err, print error message too
        println!("{:?}", validatation_result);
        assert!(validatation_result.is_ok());
    }
}

// hello:/meow
