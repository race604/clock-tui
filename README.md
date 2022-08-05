# clock-tui (tclock)

A clock app in terminal. It support the following modes:

## Clock

![clock](/assets/demo-clock-mode.gif)

## Timer

![timer](/assets/demo-timer-mode.gif)

## Stopwatch

![stopwatch](/assets/demo-stopwatch-mode.gif)

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

```shell
$ tclock --help
```

## Clock mode, this it the default mode

```shell
$ tclock clock

# Or just run
$ tclock
```

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

## Run stopwatch

```shell
$ tclock stopwatch
```

You can press `Space` key to _pause_ and _resume_ the stopwatch.

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

MIT License.
