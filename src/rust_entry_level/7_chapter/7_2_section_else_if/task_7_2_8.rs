fn input(x: usize) -> Vec<u8> {
    let v = std::io::stdin()
        .lines()
        .take(x)
        .map(|v| v.unwrap().trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    return v;
}

fn check_element(user: &str, access: u8) {
    let access_rights = [
        "    - execute only",
        "    - write only",
        "    - write\n    - execute",
        "    - read only",
        "    - read\n    - execute",
        "    - read\n    - write",
        "    - read\n    - write\n    - execute"
    ];
    if access == 0 {
        println!("{user} (no access).");
    } else {
        let suffix = if access == 7 { " (full access)" } else { "" };
        println!("{user}{suffix}:");
        println!("{}", access_rights[access as usize - 1]);
    }
}

fn main() {
    let [user, group, other] = input(3)[..] else { todo!() };
    check_element("User", user);
    check_element("Group", group);
    check_element("Other", other);
}
