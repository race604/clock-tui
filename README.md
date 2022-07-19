# clock-tui

A clock app in terminal. It support the following modes:

## Clock

![clock](/assets/demo-clock-mode.gif)

## Timer

![timer](/assets/demo-timer-mode.gif)

## Stopwatch

![stopwatch](/assets/demo-stopwatch-mode.gif)

# Usage

## Help inforamtion

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

## Run stopwatch

```shell
# Start timer for 5 minutes
$ tclock stopwatch
```

You can press `Space` key to _pause_ and _resume_ the stopwatch.

# License

MIT License.
