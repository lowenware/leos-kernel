# LeOS Kernel

Kernel project for operating system for ARM 64bit (AArch64) written in Rust.

## Developer's notes

> Logging is done using uart0, but future implementation will be different. Boot log will be done
> using memory buffer with futher output to file. Boot log will be shown only in case of panic.

> At this moment there is no SoC neither board abstraction layer, but it will be done as soon
> as its model will be defined.

## Compilation

```shell
make
```

## Execution in QEMU

```shell
make run
```

Run in debug mode

```shell
make run debug=1
```

## Attach GDB

```shell
make debug
```

## Cleanup

```shell
make clean
```

