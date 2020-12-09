# Linux Modules

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat)](https://github.com/RichardLitt/standard-readme)
[![linux_modules crates.io version and link](https://img.shields.io/crates/v/linux_modules.svg)](https://crates.io/crates/linux_modules)
![linux_modules Crates.io license](https://img.shields.io/crates/l/linux_modules)
[![linux_modules docs.rs badge](https://docs.rs/linux_modules/badge.svg)](https://docs.rs/linux_modules)

Tool To Manage Linux Kernel Modules

This is a tool to manage your Linux Kernel Modules.
It is an alternative to `modprobe`, and supports listing, adding, and removing modules,
as well as displaying information on them.

It does not yet support modprobe style dependencies or aliases,
and is not capable of displaying module signature data, except whether one exists or not.

## Install

```shell
cargo install linux_modules
```

## Usage

### CLI

```shell
$ nms info loop
╭───────────────────┬──────────────────────────────────────╮
│ File              ┆ /lib/modules/5.5.13-arch1-1/kernel/d │
│                   ┆ rivers/block/loop.ko.xz              │
╞═══════════════════╪══════════════════════════════════════╡
│ Authors           ┆                                      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ License           ┆ GPL                                  │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Description       ┆                                      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Version           ┆                                      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Firmware          ┆                                      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Alias             ┆ devname:loop-control                 │
│                   ┆ char-major-10-237                    │
│                   ┆ block-major-7-*                      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Dependencies      ┆                                      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Soft Dependencies ┆                                      │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ In Tree           ┆ true                                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Retpoline         ┆ true                                 │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Staging           ┆ false                                │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Version Magic     ┆ 5.5.13-arch1-1 SMP preempt           │
│                   ┆ mod_unload                           │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Source Checksum   ┆ 7EB1D1BC035F5C6FD6EE3FD              │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Parameters        ┆ ╭──────────┬────────────────┬──────╮ │
│                   ┆ │ Name     ┆ Desc           ┆ Type │ │
│                   ┆ ╞══════════╪════════════════╪══════╡ │
│                   ┆ │ max_loop ┆ Maximum number ┆ int  │ │
│                   ┆ │          ┆ of loop        ┆      │ │
│                   ┆ │          ┆ devices        ┆      │ │
│                   ┆ ├╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌┤ │
│                   ┆ │ max_part ┆ Maximum number ┆ int  │ │
│                   ┆ │          ┆ of partitions  ┆      │ │
│                   ┆ │          ┆ per loop       ┆      │ │
│                   ┆ │          ┆ device         ┆      │ │
│                   ┆ ╰──────────┴────────────────┴──────╯ │
├╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Signature         ┆ true                                 │
╰───────────────────┴──────────────────────────────────────╯
```

Note that the table size will adapt to your terminal.
This example uses size `60`.

### Pager

`nms info` and `nms list` automatically pipe output to a pager,
defaulting to `less`, but this may be customized through environment variables.

See the [`pager`](https://crates.io/crates/pager) crate for details on
how to customize what pager is used.

## Changelog

Please see [CHANGELOG](CHANGELOG.md) for version history

## Contributing

This crate is not looking for contributors at this time.

However, feel free to ask questions and request bindings using github issues,
or suggest/discuss API improvements.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0)>
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT)>

at your option.
