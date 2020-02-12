#[cfg(feature = "derive")]
pub mod derive {
    use clingo::ClingoError;
    use clingo::Symbol;
    use clingo::ToSymbol;

    #[derive(Copy, Clone, ToSymbol)]
    struct Test;

    #[derive(Copy, Clone, ToSymbol)]
    struct Test2;

    #[derive(ToSymbol)]
    struct Bla<'a> {
        test: Test,
        s: String,
        u_32: u32,
        tup: (u32, String),
        str1: bool,
        str2: &'a str,
    }

    #[derive(ToSymbol)]
    struct Blub(Test, Test2);

    #[derive(ToSymbol)]
    pub enum Signs<'a> {
        Minus,
        Mix(u32, String),
        Tup((u32, String)),
        Plus { uuu: u32, tup: (u32, String) },
        Strange { sds: &'a str },
    }

    #[test]
    fn to_symbol() {
        let t = Test;
        let s1 = t.symbol().unwrap();
        let s2 = clingo::parse_term("test").unwrap();
        assert_eq!(s1, s2);

        let bla = Bla {
            test: t,
            s: "bala".to_string(),
            u_32: 1,
            tup: (47, "bum".to_string()),
            str1: false,
            str2: &"ddbb",
        };
        let s1 = bla.symbol().unwrap();
        let s2 = clingo::parse_term("bla(test,\"bala\",1,(47,\"bum\"),false,\"ddbb\")").unwrap();
        assert_eq!(s1, s2);

        let t2 = Test2;
        let blub = Blub(t, t2);
        let s1 = blub.symbol().unwrap();
        let s2 = clingo::parse_term("blub(test,test_2)").unwrap();
        assert_eq!(s1, s2);

        let sign = Signs::Minus;
        let s1 = sign.symbol().unwrap();
        let s2 = clingo::parse_term("minus").unwrap();
        assert_eq!(s1, s2);

        let sign = Signs::Mix(42, "bla".to_string());
        let s1 = sign.symbol().unwrap();
        let s2 = clingo::parse_term("mix(42,\"bla\")").unwrap();
        assert_eq!(s1, s2);

        let sign = Signs::Tup((42, "bla".to_string()));
        let s1 = sign.symbol().unwrap();
        let s2 = clingo::parse_term("tup((42,\"bla\"))").unwrap();
        assert_eq!(s1, s2);

        let sign = Signs::Plus {
            uuu: 3,
            tup: (4, "HHHR".to_string()),
        };
        let s1 = sign.symbol().unwrap();
        let s2 = clingo::parse_term("plus(3,(4,\"HHHR\"))").unwrap();
        assert_eq!(s1, s2);
    }
}
