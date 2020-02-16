# Linux Modules

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat)](https://github.com/RichardLitt/standard-readme)

[![linux_modules crates.io version and link](https://img.shields.io/crates/v/linux_modules.svg)](https://crates.io/crates/linux_modules)
![linux_modules Crates.io license](https://img.shields.io/crates/l/linux_modules)
[![linux_modules docs.rs badge](https://docs.rs/linux_modules/badge.svg)](https://docs.rs/linux_modules)

Tool To Manage Linux Kernel Modules

This is a tool to manage your Linux Kernel Modules.
It is an alternative to `modprobe` and supports listing, adding, removing modules,
as well as getting information on them.

## Install

```shell
cargo install linux_modules
```

## Usage

### CLI

```shell
$ linux_modules info loop
+-------------------+----------------------------------------------------------+
| File              | loop.ko                                                  |
+==============================================================================+
| Authors           |                                                          |
|-------------------+----------------------------------------------------------|
| License           | GPL                                                      |
|-------------------+----------------------------------------------------------|
| Description       |                                                          |
|-------------------+----------------------------------------------------------|
| Version           |                                                          |
|-------------------+----------------------------------------------------------|
| Firmware          |                                                          |
|-------------------+----------------------------------------------------------|
| Alias             | devname:loop-control                                     |
|                   | char-major-10-237                                        |
|                   | block-major-7-*                                          |
|-------------------+----------------------------------------------------------|
| Dependencies      |                                                          |
|-------------------+----------------------------------------------------------|
| Soft Dependencies |                                                          |
|-------------------+----------------------------------------------------------|
| In Tree           | true                                                     |
|-------------------+----------------------------------------------------------|
| Retpoline         | true                                                     |
|-------------------+----------------------------------------------------------|
| Staging           | false                                                    |
|-------------------+----------------------------------------------------------|
| Version Magic     | 5.5.3-arch1-1 SMP preempt mod_unload                     |
|-------------------+----------------------------------------------------------|
| Source Checksum   | 7EB1D1BC035F5C6FD6EE3FD                                  |
|-------------------+----------------------------------------------------------|
| Parameters        | +----------+------------------------------------+------+ |
|                   | | Name     | Desc                               | Type | |
|                   | +======================================================+ |
|                   | | max_part | Maximum number of partitions per   | int  | |
|                   | |          | loop device                        |      | |
|                   | |----------+------------------------------------+------| |
|                   | | max_loop | Maximum number of loop devices     | int  | |
|                   | +----------+------------------------------------+------+ |
|-------------------+----------------------------------------------------------|
| Signature         | true                                                     |
+-------------------+----------------------------------------------------------+
```

Note that the table size will adapt to your terminal.
