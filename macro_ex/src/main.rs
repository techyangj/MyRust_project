// e: Expression, t: TokenTree
// expr    tt
macro_rules! my_macro {
    ($num: expr) => {
        let x = $num;
        println!("{}", x);
    };
}

macro_rules! array_macro {
    ($($num: expr), *) => {
        $(
            let x = $num;
            println!("{}",x);
        )*
    };
}

trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_as_bytes {
    ($($ty: ty), *) => {
        $(
            impl AsBytes for $ty {
                fn as_bytes(&self) -> Vec<u8> {
                    Vec::from(<$ty>::to_ne_bytes(*self))
                }
            }
        )*
    }
}

mod macros {
    #[macro_export]
    macro_rules! macro1 {
        () => {
            println!("1");
            macro1!(@macro2);
        };

        (@macro2) => {
            println!("hello");
        }
    }

    // macro_rules! macro2 {
    //     () => {
    //         println!("2");
    //     };
    // }
}

fn main() {
    my_macro!(1);
    my_macro!("the");
    array_macro![1, "ff"];
    array_macro!(1, 2);

    impl_as_bytes!(u16, u32);
    let u32_item = 1287u32;
    println!("bytes: {:?}", u32_item.to_ne_bytes());
    println!("u32: {:?}", u32_item.as_bytes());

    macro1!();
}
