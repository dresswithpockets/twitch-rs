
pub fn sub(str: &String, a: usize, b: usize) -> String {
    str[a .. str.char_indices().nth(b).unwrap().0].to_owned()
}

pub fn args_with_quotes(args_string: &String) -> Vec<String> {
    let mut args = Vec::<String>::new();

    if args_string.is_empty() {
        return Vec::<String>::new();
    }

    let mut previous_quoted = !args_string.starts_with("\"");
    for arg in args_string.split("\"") {
        if arg.is_empty() {
            continue;
        }

        if !previous_quoted {
            args.push(arg.to_owned());
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

            args.push(dyn_arg.to_owned());
            previous_quoted = false;
        }
    }

    return args.clone();
}