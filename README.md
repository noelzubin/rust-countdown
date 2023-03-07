# Countdown
Rust port of https://github.com/antonmedv/countdown/blob

<p align="center"><img src="https://user-images.githubusercontent.com/141232/54696023-9ed03e00-4b5d-11e9-9c7b-d6f67691e70c.gif" width="450" alt="Screen shot"></p>

## Install
```
cargo install --path .
```

## Usage

Specify duration in format `1h2m3s`.

``` bash
countdown 25s
```

Or specify target time: `02:15PM` or `14:15`. For instance, if the current time 
would be 11:30, the next example would trigger a 2-minute countdown. 

```bash
countdown 11:32
```

Add a command with `&&` to run after the countdown.

```bash
countdown 1m30s && say "Hello, world"
```

Count from up from the zero.

```bash
countdown -up 30s
```


## Key binding

- `p`: To pause the countdown.
- `c`: To resume the countdown.
- `q` or `Ctrl+C`: To stop the countdown without running next command.
