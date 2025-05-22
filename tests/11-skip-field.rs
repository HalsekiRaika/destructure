mod entities {
    use destructure::Destructure;

    #[derive(Destructure)]
    pub struct Domain {
        pub a: String,
        pub b: String,
        pub c: String,
        #[destructure(skip)]
        d: String,
    }

    impl Default for Domain {
        fn default() -> Self {
            Domain {
                a: "a".to_string(),
                b: "b".to_string(),
                c: "c".to_string(),
                d: "d".to_string(),
            }
        }
    }
}

use crate::entities::Domain;

fn main() {
    let domain = Domain::default();
    let mut destruct = domain.into_destruct();

    destruct.a = "aa".to_string();
    // destruct.d = "dd".to_string(); << error[E0616]: field `d` of struct `DestructDomain` is private

    let domain = destruct.freeze();
    assert_eq!(domain.a, "aa");
}
