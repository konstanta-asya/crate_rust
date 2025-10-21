peg::parser! {
    pub grammar list_parser() for str {
        rule number() -> i32
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule list() -> Vec<i32>
            = "[" l:(number() ** ",") "]" { l }
    }
}
