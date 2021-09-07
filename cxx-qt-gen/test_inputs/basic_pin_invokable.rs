mod my_object {
    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        fn say_hi(&self, _cpp: Pin<&mut CppObj>, string: &QString, number: i32) {
            println!(
                "Hi from Rust! String is {} and number is {}",
                string, number
            );
        }

        fn say_bye(&self, _cpp: Pin<&mut CppObj>) {
            println!("Bye from Rust!");
        }
    }
}
