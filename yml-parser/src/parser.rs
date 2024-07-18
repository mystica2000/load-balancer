pub(crate) fn preprocess_by_line(line: &str) -> Result<String, &'static str> {
  let mut result = Vec::new();
  let mut in_quotes: bool = false;
  let mut chars = line.chars().peekable();
  let mut prev_char = ' ';
  let mut count_colon = 0;
  let mut index = 0;

  while let Some(c) = chars.next() {
    match c {
      '"' => handle_quote(&mut result, &mut in_quotes, &mut chars, prev_char, index, &line)?,
      ':' => handle_colon(&mut result, &mut in_quotes, &mut count_colon)?,
      '#' => {
        if !handle_comment(in_quotes, prev_char) {
          break;
        }
        result.push(c);
      }
      _ => {
        result.push(c)
      },
    }
    index += 1;
    prev_char = c;
  }

  if in_quotes {
    return Err("Invalid syntax: Unclosed quote");
}

  Ok(result.iter().collect())
}

fn handle_quote(result: &mut Vec<char>, in_quotes: &mut bool, chars: &mut std::iter::Peekable<std::str::Chars>, prev_char: char, index: usize, line: &str)  -> Result<(), &'static str> {
  if *in_quotes {
    *in_quotes = false;
    result.push('"');
    if let Some(next_char) = chars.peek() {
      if !next_char.is_whitespace() {
        return Err("Invalid Syntax: Unexpected character after closing quote: Comments must be separated from other tokens by white space characters");
      } else {
        let remaining = &line[index + 1..].trim();
       if !remaining.starts_with("#") && !remaining.is_empty() {
          return Err("Invalid Syntax: Unexpected character after closing quote");
       }
      }
    }
  } else {
    // "server #1""test" that is not like name:"test"
    if !result.ends_with(&[' ']) && !result.is_empty() && !result.ends_with(&[':'])   {
      return Err("Invalid Syntax: Unexpected opening quote");
    }
    *in_quotes = true;
    result.push('"');
  }

  Ok(())
}

fn handle_colon(result: &mut Vec<char>, in_quotes: &mut bool, count_colon: &mut u8) -> Result<(), &'static str> {
  if *in_quotes {
    result.push(':');
  } else {
    *count_colon += 1;
    if *count_colon > 1 {
      return Err("Invalid Syntax: colon present multiple times on the single row");
    }
    result.push(':');
  }
  Ok(())
}

fn handle_comment(in_quotes: bool, prev_char: char) -> bool {
  if !in_quotes && prev_char.is_whitespace() { // if not in quotes then, remove others
      false
    } else {
      true
    }
  // !(in_quotes || prev_char.is_whitespace())
}

pub(crate) fn parse_to_object(line: &str) {

}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_preprocess_by_line() {
    let line = "load_balancer: ";
    let result = preprocess_by_line(line);
    assert_eq!(result.unwrap(), "load_balancer: ");

    let line = "name: my-load-balancer # name of the load balancer";
    let result = preprocess_by_line(line);
    assert_eq!(result.unwrap(), "name: my-load-balancer ");

    let line = r#"name: "test" another_name: "value""#;
    let result = preprocess_by_line(line);
    assert!(result.is_err());

    let line = r#"name: "test with a #" inside""#;
    let result = preprocess_by_line(line);
    assert!(result.is_err());

    let line = r#"name: inside""#;
    let result = preprocess_by_line(line);
    assert!(result.is_err());

    let line = r#"name: "inside"  ":test" "#;
    let result = preprocess_by_line(line);
    assert!(result.is_err());

    let line = r#"name: "inside" # testing "#;
    let result = preprocess_by_line(line);
    assert_eq!(result.unwrap(), "name: \"inside\" ");

  }
}