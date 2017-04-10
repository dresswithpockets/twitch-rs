
pub fn sub(str: &str, a: usize, b: usize) -> &str {
    &str[a .. str.char_indices().nth(b).unwrap().0]
}

pub fn args_with_quotes(args_string: &str) -> &[&str] {
    let mut args = Vec::new();

    if args_string.is_empty() {
        return &args;
    }

    let mut previous_quoted = !args_string.starts_with("\"");
    for arg in args_string.split("\"") {
        if arg.is_empty() {
            continue;
        }

        if !previous_quoted {
            args.push(arg);
            previous_quoted = true;
            continue;
        }

        if !arg.contains(" ") {
            continue;
        }

        for dyn_arg in arg.split(" ") {
            if dyn_arg.is_empty() ||
                dyn_arg.trim().is_empty()
            {
                continue;
            }

            args.push(dyn_arg);
            previous_quoted = false;
        }
    }

    return &args;
}