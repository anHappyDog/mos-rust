# MOS_WITH_RUST

为什么会有 RUST,就像为什么 RUST 会是 RUST 一样。这是使用RUST编写的MIPS内核，具有简单的内存管理和异常处理，可以调度并执行C用户态程序，最后通过Shell与用户交互。
## 编译

```shell
cargo build
```

## 运行

```shell
cargo run
```

## 清除输出文件

```shell
cargo clean
```

## C 兼容

事实上要兼顾的只是 C 中对`Page`和`Env`结构体的使用：

- `Page`:用户态中只是用了其成员变量`pp_ref`，并且通过数组元素的方式访问，故需要保持其 Page 大小不变，`pp_ref`成员内存布局位置不变。

- `Env`:与`Page`结构体类似，同样通过数组元素访问，访问的成员变量有`env_id`,`env_runs`与`env_user_tlb_mod_entry`。

shell的一些bug懒得改了。
