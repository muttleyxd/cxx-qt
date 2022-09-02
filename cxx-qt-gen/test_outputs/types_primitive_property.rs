#[cxx::bridge(namespace = "cxx_qt::my_object")]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-gen/include/my_object.cxxqt.h");

        #[cxx_name = "MyObject"]
        type MyObjectQt;

        #[rust_name = "emit_boolean_changed"]
        fn emitBooleanChanged(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_float_32_changed"]
        fn emitFloat32Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_float_64_changed"]
        fn emitFloat64Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_int_8_changed"]
        fn emitInt8Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_int_16_changed"]
        fn emitInt16Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_int_32_changed"]
        fn emitInt32Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_uint_8_changed"]
        fn emitUint8Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_uint_16_changed"]
        fn emitUint16Changed(self: Pin<&mut MyObjectQt>);

        #[rust_name = "emit_uint_32_changed"]
        fn emitUint32Changed(self: Pin<&mut MyObjectQt>);
    }

    extern "Rust" {
        #[cxx_name = "MyObjectRust"]
        type MyObject;

        #[cxx_name = "getBoolean"]
        fn get_boolean(self: &MyObject, cpp: &MyObjectQt) -> bool;
        #[cxx_name = "setBoolean"]
        fn set_boolean(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: bool);

        #[cxx_name = "getFloat32"]
        fn get_float_32(self: &MyObject, cpp: &MyObjectQt) -> f32;
        #[cxx_name = "setFloat32"]
        fn set_float_32(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: f32);

        #[cxx_name = "getFloat64"]
        fn get_float_64(self: &MyObject, cpp: &MyObjectQt) -> f64;
        #[cxx_name = "setFloat64"]
        fn set_float_64(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: f64);

        #[cxx_name = "getInt8"]
        fn get_int_8(self: &MyObject, cpp: &MyObjectQt) -> i8;
        #[cxx_name = "setInt8"]
        fn set_int_8(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i8);

        #[cxx_name = "getInt16"]
        fn get_int_16(self: &MyObject, cpp: &MyObjectQt) -> i16;
        #[cxx_name = "setInt16"]
        fn set_int_16(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i16);

        #[cxx_name = "getInt32"]
        fn get_int_32(self: &MyObject, cpp: &MyObjectQt) -> i32;
        #[cxx_name = "setInt32"]
        fn set_int_32(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: i32);

        #[cxx_name = "getUint8"]
        fn get_uint_8(self: &MyObject, cpp: &MyObjectQt) -> u8;
        #[cxx_name = "setUint8"]
        fn set_uint_8(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: u8);

        #[cxx_name = "getUint16"]
        fn get_uint_16(self: &MyObject, cpp: &MyObjectQt) -> u16;
        #[cxx_name = "setUint16"]
        fn set_uint_16(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: u16);

        #[cxx_name = "getUint32"]
        fn get_uint_32(self: &MyObject, cpp: &MyObjectQt) -> u32;
        #[cxx_name = "setUint32"]
        fn set_uint_32(self: &mut MyObject, cpp: Pin<&mut MyObjectQt>, value: u32);
    }

    unsafe extern "C++" {
        include ! (< QtCore / QObject >);
        include!("cxx-qt-lib/include/convert.h");
        include!("cxx-qt-lib/include/cxxqt_thread.h");

        type MyObjectCxxQtThread;

        #[cxx_name = "unsafeRust"]
        fn rust(self: &MyObjectQt) -> &MyObject;

        #[cxx_name = "qtThread"]
        fn qt_thread(self: &MyObjectQt) -> UniquePtr<MyObjectCxxQtThread>;
        fn queue(self: &MyObjectCxxQtThread, func: fn(ctx: Pin<&mut MyObjectQt>)) -> Result<()>;

        #[rust_name = "new_cpp_object"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn newCppObject() -> UniquePtr<MyObjectQt>;
    }

    extern "C++" {
        #[cxx_name = "unsafeRustMut"]
        unsafe fn rust_mut(self: Pin<&mut MyObjectQt>) -> Pin<&mut MyObject>;
    }

    extern "Rust" {
        #[cxx_name = "createRs"]
        #[namespace = "cxx_qt::my_object::cxx_qt_my_object"]
        fn create_rs() -> Box<MyObject>;
    }
}

pub use self::cxx_qt_ffi::*;
mod cxx_qt_ffi {
    use super::ffi::*;

    pub type FFICppObj = super::ffi::MyObjectQt;
    type UniquePtr<T> = cxx::UniquePtr<T>;

    unsafe impl Send for MyObjectCxxQtThread {}

    use std::pin::Pin;

    #[derive(Default)]
    pub struct MyObject {
        boolean: bool,
        float_32: f32,
        float_64: f64,
        int_8: i8,
        int_16: i16,
        int_32: i32,
        uint_8: u8,
        uint_16: u16,
        uint_32: u32,
    }

    impl MyObject {
        pub fn get_boolean(&self, cpp: &MyObjectQt) -> bool {
            cpp.get_boolean()
        }

        pub fn set_boolean(&mut self, cpp: Pin<&mut MyObjectQt>, value: bool) {
            cpp.set_boolean(value);
        }

        pub fn get_float_32(&self, cpp: &MyObjectQt) -> f32 {
            cpp.get_float_32()
        }

        pub fn set_float_32(&mut self, cpp: Pin<&mut MyObjectQt>, value: f32) {
            cpp.set_float_32(value);
        }

        pub fn get_float_64(&self, cpp: &MyObjectQt) -> f64 {
            cpp.get_float_64()
        }

        pub fn set_float_64(&mut self, cpp: Pin<&mut MyObjectQt>, value: f64) {
            cpp.set_float_64(value);
        }

        pub fn get_int_8(&self, cpp: &MyObjectQt) -> i8 {
            cpp.get_int_8()
        }

        pub fn set_int_8(&mut self, cpp: Pin<&mut MyObjectQt>, value: i8) {
            cpp.set_int_8(value);
        }

        pub fn get_int_16(&self, cpp: &MyObjectQt) -> i16 {
            cpp.get_int_16()
        }

        pub fn set_int_16(&mut self, cpp: Pin<&mut MyObjectQt>, value: i16) {
            cpp.set_int_16(value);
        }

        pub fn get_int_32(&self, cpp: &MyObjectQt) -> i32 {
            cpp.get_int_32()
        }

        pub fn set_int_32(&mut self, cpp: Pin<&mut MyObjectQt>, value: i32) {
            cpp.set_int_32(value);
        }

        pub fn get_uint_8(&self, cpp: &MyObjectQt) -> u8 {
            cpp.get_uint_8()
        }

        pub fn set_uint_8(&mut self, cpp: Pin<&mut MyObjectQt>, value: u8) {
            cpp.set_uint_8(value);
        }

        pub fn get_uint_16(&self, cpp: &MyObjectQt) -> u16 {
            cpp.get_uint_16()
        }

        pub fn set_uint_16(&mut self, cpp: Pin<&mut MyObjectQt>, value: u16) {
            cpp.set_uint_16(value);
        }

        pub fn get_uint_32(&self, cpp: &MyObjectQt) -> u32 {
            cpp.get_uint_32()
        }

        pub fn set_uint_32(&mut self, cpp: Pin<&mut MyObjectQt>, value: u32) {
            cpp.set_uint_32(value);
        }
    }

    impl MyObjectQt {
        pub fn get_boolean(&self) -> bool {
            self.rust().boolean
        }

        pub fn set_boolean(mut self: Pin<&mut Self>, value: bool) {
            unsafe {
                self.as_mut().rust_mut().boolean = value;
            }
            self.as_mut().emit_boolean_changed();
        }

        pub fn get_float_32(&self) -> f32 {
            self.rust().float_32
        }

        pub fn set_float_32(mut self: Pin<&mut Self>, value: f32) {
            unsafe {
                self.as_mut().rust_mut().float_32 = value;
            }
            self.as_mut().emit_float_32_changed();
        }

        pub fn get_float_64(&self) -> f64 {
            self.rust().float_64
        }

        pub fn set_float_64(mut self: Pin<&mut Self>, value: f64) {
            unsafe {
                self.as_mut().rust_mut().float_64 = value;
            }
            self.as_mut().emit_float_64_changed();
        }

        pub fn get_int_8(&self) -> i8 {
            self.rust().int_8
        }

        pub fn set_int_8(mut self: Pin<&mut Self>, value: i8) {
            unsafe {
                self.as_mut().rust_mut().int_8 = value;
            }
            self.as_mut().emit_int_8_changed();
        }

        pub fn get_int_16(&self) -> i16 {
            self.rust().int_16
        }

        pub fn set_int_16(mut self: Pin<&mut Self>, value: i16) {
            unsafe {
                self.as_mut().rust_mut().int_16 = value;
            }
            self.as_mut().emit_int_16_changed();
        }

        pub fn get_int_32(&self) -> i32 {
            self.rust().int_32
        }

        pub fn set_int_32(mut self: Pin<&mut Self>, value: i32) {
            unsafe {
                self.as_mut().rust_mut().int_32 = value;
            }
            self.as_mut().emit_int_32_changed();
        }

        pub fn get_uint_8(&self) -> u8 {
            self.rust().uint_8
        }

        pub fn set_uint_8(mut self: Pin<&mut Self>, value: u8) {
            unsafe {
                self.as_mut().rust_mut().uint_8 = value;
            }
            self.as_mut().emit_uint_8_changed();
        }

        pub fn get_uint_16(&self) -> u16 {
            self.rust().uint_16
        }

        pub fn set_uint_16(mut self: Pin<&mut Self>, value: u16) {
            unsafe {
                self.as_mut().rust_mut().uint_16 = value;
            }
            self.as_mut().emit_uint_16_changed();
        }

        pub fn get_uint_32(&self) -> u32 {
            self.rust().uint_32
        }

        pub fn set_uint_32(mut self: Pin<&mut Self>, value: u32) {
            unsafe {
                self.as_mut().rust_mut().uint_32 = value;
            }
            self.as_mut().emit_uint_32_changed();
        }
    }

    pub fn create_rs() -> std::boxed::Box<MyObject> {
        std::default::Default::default()
    }
}
