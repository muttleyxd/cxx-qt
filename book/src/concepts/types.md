<!--
SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>

SPDX-License-Identifier: MIT OR Apache-2.0
-->

# Types

CXX-Qt supports most types supported by CXX. These can be used in properties, invokables, and signals.

Note that there is currently missing support for a few CXX types in CXX-Qt, which is tracked in [this issue](https://github.com/KDAB/cxx-qt/issues/328).

## `cxx-qt-lib` Types

The `cxx-qt-lib` crate provides CXX bindings for common Qt types.

Use the [`cxx-qt-lib` Docs](https://docs.rs/cxx-qt-lib/latest/cxx_qt_lib/) to explore the available types.

## Defining a Custom Type

Any types that are valid CXX types should be usable with CXX-Qt as well.

Note that the same rules apply as CXX, so a type must be [trivial](https://cxx.rs/extern-c++.html?highlight=trivial#integrating-with-bindgen-generated-or-handwritten-unsafe-bindings) to pass by value.
If they are opaque, references or pointers must be used.

For examples of how to wrap Qt objects, explore the [`cxx-qt-lib` source code](https://github.com/KDAB/cxx-qt/tree/main/crates/cxx-qt-lib).

## Opaque Type Conversions

If your type can't be marked as trivial for CXX, but the Qt API needs to take the type by value (eg `T` rather than `const T&` or `::std::unique_ptr<T>`),
then you can define [type conversions](type-conversions.md) in C++ that convert between pointers / references and values.
Then when using properties, invokables, or signals an attribute in the macro can be used.

### Properties

In the following example using `cxx_type` means that `OpaqueExampleType` will be the type of the `Q_PROPERTY`, getter, and setter in C++.
On the Rust side `cxx::UniquePtr<OpaqueExampleType>` will be the type and CXX-Qt will perform the conversions between.

```rust,ignore
#[cxx_qt::qobject]
struct MyStruct {
  #[qproperty(cxx_type = "OpaqueExampleType")]
  my_property: cxx::UniquePtr<OpaqueExampleType>
}
```

### Invokables

In the following example using `return_cxx_type` means that `OpaqueExampleType` will be the return type of the `Q_INVOKABLE` in C++.
On the Rust side `cxx::UniquePtr<OpaqueExampleType>` will be the return type and CXX-Qt will perform the conversion between.

```rust,ignore
impl qobject::MyStruct {
  #[qinvokable(return_cxx_type = "OpaqueExampleType")]
  pub fn invokable(&self) -> cxx::UniquePtr<OpaqueExampleType> {
    ...
  }
}
```

### Signals

In the following example using `cxx_type` means that `OpaqueExampleType` will be the parameter type of the `Q_SIGNAL` in C++.
On the Rust side `cxx::UniquePtr<OpaqueExampleType>` will be the type in the enum and CXX-Qt will perform the conversion between.

```rust,ignore
#[cxx_qt::qsignals(MyStruct)]
pub enum Signals {
  #[cxx_type = "OpaqueExampleType"]
  NewData { value: cxx::UniquePtr<OpaqueExampleType> },
}
```
