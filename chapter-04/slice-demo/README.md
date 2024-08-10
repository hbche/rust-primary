### 字符串切片

#### 背景

写一个从由空格分隔的字符串语句中找出第一个单词的索引：

``` rust
fn main() {
    let mut s = String::from("Hello world");
    let first_index = first_word(&s);

    // 清空字符串
    // s.clear();

    println!("{}", first_index);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return index
        }
    }

    s.len()
}
```

上述例子存在的问题：无法将 first_index 与 s 字符串关联，如果字符串被清空，first_index 仍然为之前的值，这是有问题的。


#### 定义

字符串切片是指向字符串中一部分内容的引用

``` rust
fn main() {
    let s = String::from("Hello word");
    let hello = &s[0..5];
    // let hello = &s[..5];
    let word = &s[6..10];
    // let word = &s[6..];
    let whold_word = &s[0..s.len()];
    // let whold_word = &s[..];
    println!("{}", hello);
    println!("{}", word);
    println!("{}", whold_word);
}
```

#### 语法

`[开始索引..结束索引]`

- 开始索引：切片起始位置的索引值
- 结束索引：切片终止位置的下一个索引值
- 如果切片开始位置是字符串引用的第一个索引值，则可以省略开始索引 `[..结束索引]`
- 如果切片结束位置是字符串引用的最后一个索引值，则可以省略结束索引，`[开始索引..]`
- 如果切片引用的是整个字符串引用，那么可以同时省略开始索引和结束索引，`[..]`

#### 注意事项

- 字符串切片的范围索引必须发生在有效的UTF-8字符边界内
- 如果尝试从一个多字节的字符中创建字符串切片，程序会报错并推出

#### 重写背景中的例子

``` rust
fn main() {
    let mut s = String::from("Hello world");
    let first_index = first_word(&s);

    s.clear();

    println!("{}", first_index);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}
```

此时 `s.clear()` 会报错: 不可以将 s 作为可变引用，因为它已经作为不可变引用了。
```
cannot borrow `s` as mutable because it is also borrowed as immutable
mutable borrow occurs hererustcClick for full compiler diagnostic
main.rs(3, 34): immutable borrow occurs here
main.rs(7, 20): immutable borrow later used here
```

#### 字符串字面值是字符串切片

字符串字面值是被直接存储在二进制程序中的

``` rust
fn main() {
    let s = "Hello, word";
}
```

变量 s 的类型是 `&str`，它是一个指向二进制特定程序的切片。`&str` 是不可变引用，所以字符串字面量是不可变的

#### 将字符串切片作为参数传递

- fn first_word(s: &String) -> &str {}
- 有经验的 Rust 开发者会采用 &str 作为参数类型，因为这样就可以接收 String 和 &str 类型的参数；
- fn first_word(s: &str) -> &str
  - 索引字符串切片直接调用该函数
  - 使用 String，可以创建一个完整的 String 切片来调用该函数
- 定义函数时使用字符串切片代替字符串引用会使我们的 API 更加通用，且不会损失任何功能。

``` rust
fn main() {
    let s1 = String::from("Hello world");
    let first_word_s1 = first_word(&s1);
    let s2 = "Helo world";
    let first_word_s2 = first_word(&s2);

    println!("{}", first_word_s1);
    println!("{}", first_word_s2);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (index, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..index];
        }
    }

    &s[..]
}
```

#### 其他类型的切片

``` rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let a1 = &a[1..3];
    println!("{:?}", a1);
}
```