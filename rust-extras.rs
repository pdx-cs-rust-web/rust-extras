fn array_slice() {
    fn use_seq(s: &mut [u8]) {
        s[2] = 4;
        println!("{:?}", s);
    }

    let mut a: [u8; 3] = [0, 1, 2];
    use_seq(&mut a);

    let mut v = vec![0, 1, 2];
    v.push(5);
    let five = v.pop().unwrap();
    assert_eq!(5, five);
    use_seq(&mut v);
}

fn from_impl() {
    #[derive(Debug, Clone)]
    struct Header {
        name: String,
        value: String,
    }

    impl From<Header> for String {
        fn from(h: Header) -> String {
            format!("{}: {}", h.name, h.value)
        }
    }

    let h = Header { name: "name".into(), value: "value".into() };
    let s: String = h.into();
    println!("{}", s);
}

fn pattern_matching() {
    let mut nums: Vec<u8> = Vec::new();
    match nums.pop() {
        Some(v) => println!("{}", v),
        None => println!("no value"),
    }
}

fn main() {
    array_slice();
    from_impl();
    pattern_matching();
}
