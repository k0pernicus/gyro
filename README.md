# gpm-rs
A new version of [giwyn-rs](https://github.com/k0pernicus/giwyn-rs)

I rewritten the project entirely (in Rust, again) to obtain better system performances, as the interactive mode.

## What is gpm-rs ?
This project is a console-based git projects monitor.  
With gpm-rs, you can keep an eye on your local git repositories, get easily some labels, statuses, last commit id, etc...

## How it works ?

* `cargo install --git https://github.com/k0pernicus/gpm-rs` ;
* `gpm scan --save` to scan your hard drive in order to find new git repositories, and save them in `~/.gpm` ;
* `gpm status` to get the status of those repositories ;
* `gpm help` to take a look at the documentation.

## License

MIT
