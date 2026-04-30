file_path = "rust/src/realm667/nom_parser.rs"
with open(file_path, "r") as f:
    lines = f.readlines()

# Line 251 is index 250
# Current: "            let (next_input, _) = ws(is_not)<&str, &str, nom::error::Error<&str>>(" \t\n\r;}")(next_input)?;"
# Target: "            let (next_input, _) = ws(is_not(" \t\n\r;}"))(next_input)?;"

# The problem is the generic types.
# Just replace: is_not::<&str, &str, nom::error::Error<&str>>(" \t\n\r;}")
# with: ws(is_not(" \t\n\r;}"))

new_line = lines[250].replace("is_not::<&str, &str, nom::error::Error<&str>>", "is_not")
new_line = new_line.replace("is_not(\" \t\n\r;}\")", "ws(is_not(\" \t\n\r;}\"))")

lines[250] = new_line
with open(file_path, "w") as f:
    f.writelines(lines)
