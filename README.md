# gyro
A new version of [giwyn-rs](https://github.com/k0pernicus/giwyn-rs)

I rewritten the project entirely (in Rust, again) to obtain better system performances, as the interactive mode.

## What is gyro ?
This project is a console-based git projects monitor.  
With gyro, you can keep an eye on your local git repositories, get easily some labels, statuses, last commit id, etc...

## How it works ?

* `cargo install --git https://github.com/k0pernicus/gyro` ;
* `gyro scan --save` to scan your hard drive in order to find new git repositories, and save some informations about them in `~/.gyro` ;
* `gyro status` to get the status of those repositories ;
* `gyro help` to take a look at the documentation.

### Commands

```
gyro 0.1.0
A. Carette <antonin@carette.xyz>
Your Git Project Monitor

USAGE:
    gyro [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -r, --reset      Reset the configuration file
    -V, --version    Prints version information

SUBCOMMANDS:
    help        Prints this message or the help of the given subcommand(s)
    override    Override default settings from your configuration file
    scan        Scan your hard disk to find git repositories
    status      Get the status of watched git repositories
```

## License

MIT
