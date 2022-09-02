# clock-tui (tclock)

A clock app in terminal. It support the following modes:

## Clock

![clock](/assets/demo-clock-mode.gif)

## Timer

![timer](/assets/demo-timer-mode.gif)

## Stopwatch

![stopwatch](/assets/demo-stopwatch-mode.gif)

## Countdown

![countdown](/assets/demo-countdown-mode.gif)

# Usage

## Install

Install excutable by `cargo`:

```shell
$ cargo install clock-tui
```

## Basic usage

```shell
$ tclock
```
Run this command to start a clock, and press `q` to exit.

You can always use `-h` or `--help` to show help message, for exmaple

```shell
$ tclock --help

# or
$ tclock clock -h
```

## Clock mode, this it the default mode

```shell
$ tclock clock

# Or just run
$ tclock
```

For more details, run `tclock clock -h` to show usage.

## Run timer

```shell
# Start timer for 5 minutes
$ tclock timer -d 5m
```

The option `-d` or `--duration` to set time, for example `100s`, `5m`, `1h`, etc.

You can press `Space` key to _pause_ and _resume_ the timer.

The timer mode also accept additional command to run when the timer ends, for example:

```
tclock timer -d 25m -e terminal-notifier -title tclock -message "'Time is up!'"
```

Here we use [terminal-notifier](https://github.com/julienXX/terminal-notifier) to fire a notification when time is up.

For more details, run `tclock timer -h` to show usage.

## Run stopwatch

```shell
$ tclock stopwatch
```

For more details, run `tclock stopwatch -h` to show usage.

## Run countdown

```shell
$ tclock countdown --time 2023-01-01 --title 'New Year 2023'`
```

You can use `-t` or `--time` to specify time, for example: `2023-01-01`, `20:00`, `'2022-12-25 20:00:00'` or `2022-12-25T20:00:00-04:00`.

You can use `-r` or `--reverse` to run in count-up mode, it counts up duration since the specific time.

For more details, run `tclock countdown -h` to show usage.

## Customize style

You can customize the styles.

### Size

You can use `-s` or `--size` option to custome clock size, for example:

```shell
$ tclock -s 2
```

### Color

You can use `-c` or `--color` to set clock forground color, for exmaple:

```shell
# color name, any one of: 
# Black, Red, Green, Yellow, Blue, Magenta, Cyan, Gray, DarkGray, LightRed,
# LightGreen, LightYellow, LightBlue, LightMagenta, LightCyan, White
$ tclock -c yellow

# or hex color
$ tclock -c '#e63946'
```

# License

MIT License, refer to [LICENSE](./LICENSE) for detail.
