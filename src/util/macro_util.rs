pub(crate) mod macro_util {
    #[macro_export]
    macro_rules! ll {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_ll= LinkedList::new();
                $(
                    temp_ll.push($x);
                )*
                temp_ll
            }
        };
    }

    pub use ll;
}
