# thokr-plus
✨ sleek typing tui with visualized results and historical logging

[![License](https://img.shields.io/badge/License-MIT-default.svg)](./LICENSE.md)
[![Crate Version](https://img.shields.io/crates/v/thokr-plus)](https://crates.io/crates/thokr-plus)
[![Github Stars](https://img.shields.io/github/stars/hindriix/thokr)](https://github.com/hindriix/thokr/stargazers)

![demo](https://github.com/thatvegandev/assets/raw/main/thokr/demo.gif)

> **thokr-plus** is an enhanced fork of [thokr](https://github.com/jrnxf/thokr).
> On top of the original it adds **continuous timed mode**, **color themes**, a
> **death mode**, and **punctuation/numbers** — while staying a drop-in `thokr`
> command. See the [Usage](#usage) section for the new flags.

## Installation

### Cargo

```sh
$ cargo install thokr-plus
```

The installed command is `thokr`.

## Usage

For detailed usage run `thokr -h`.

```
sleek typing tui with visualized results and historical logging

Usage: thokr [OPTIONS]

Options:
  -w, --number-of-words <NUMBER_OF_WORDS>
          number of words to use in test (initial buffer only when timed) [default: 15]
  -f, --full-sentences <NUMBER_OF_SENTENCES>
          number of sentences to use in test
  -s, --number-of-secs <NUMBER_OF_SECS>
          run a timed test for this many seconds; words stream continuously until the clock stops (unless -p or -f fixes the text)
  -p, --prompt <PROMPT>
          custom prompt to use
  -l, --supported-language <SUPPORTED_LANGUAGE>
          language to pull words from [default: english] [possible values: english, english1k, english10k]
      --pace <PACE>
          ghost caret pacing at this WPM to race against
      --theme <THEME>
          color theme; a `theme.json` in the config dir can override any color [default: default] [possible values: default, matrix, dracula, ocean, mono]
  -h, --help
          Print help
  -V, --version
          Print version
```


### Examples

| command                     |                                                    test contents |
|:----------------------------|-----------------------------------------------------------------:|
| `thokr`                     |                          50 of the 200 most common english words |
| `thokr -w 100`              |                         100 of the 200 most common English words |
| `thokr -w 100 -l english1k` |                        100 of the 1000 most common English words |
| `thokr -s 30`               |    30-second timed test — words stream continuously until time runs out |
| `thokr -s 60 -l english1k`  |     60-second timed test drawing from the 1000 most common words |
| `thokr -p "$(cat foo.txt)" -s 30` | custom prompt with a 30s cap (fixed text, stops early if you finish it) |
| `thokr -p "$(cat foo.txt)"` |                   custom prompt with the output of `cat foo.txt` |
| `thokr -f 4`                | 4 grammatical sentences with full stops; overrides word settings |
| `thokr --pace 60`           |         15 most common words with a ghost caret racing at 60 wpm |

_During a test you can press ← to start over or → to see a new prompt (assuming
you didn't supply a custom one)_

### Timed (continuous) mode

Passing `-s <seconds>` on its own runs a **continuous** test: fresh words stream
in as you type and the test only ends when the clock hits zero — a long prompt
scrolls a few lines at a time so the layout stays put. `-w` then just sets the
size of the initial on-screen buffer. Combining `-s` with a custom prompt (`-p`)
or sentences (`-f`) keeps the text fixed, so the test still ends when you finish
it or the timer expires, whichever comes first.

### Theming

Pick a built-in palette with `--theme`:

```sh
$ thokr --theme dracula
$ thokr -s 30 --theme matrix
```

Available presets: `default` (the classic look), `matrix`, `dracula`, `ocean`,
and `mono`.

For full control, drop a `theme.json` into thokr's config directory (the same
folder as `log.csv` — see [Logging](#logging)). Any color you set overrides the
selected preset; anything you omit falls back to it. Colors accept either a
named terminal color or a `#rrggbb` hex value:

```json
{
  "preset": "dracula",
  "correct": "#50fa7b",
  "incorrect": "red",
  "pending": "#6272a4",
  "graph": "lightmagenta",
  "timer": "#f1fa8c",
  "text": "white"
}
```

An unrecognized color is ignored in favor of the preset, so a typo never leaves
the interface unreadable.

## Supported Languages

The following languages are available by default:

| name         |                     description |
| :----------- | ------------------------------: |
| `english`    |   200 most common English words |
| `english1k`  |  1000 most common English words |
| `english10k` | 10000 most common English words |

## Logging

Upon completion of a test, a row outlining your results is appended to the
`log.csv` file found in the following platform-specific folders. This way you
can easily track your progress over time.

| platform | value                                             |                                         example |
| :------- | ------------------------------------------------- | ----------------------------------------------: |
| Linux    | `$XDG_CONFIG_HOME/thokr` or `$HOME/.config/thokr` |                       /home/colby/.config/thokr |
| macOS    | `$HOME/Library/Application Support/thokr`         | /Users/colby/Library/Application Support/thokr  |
| Windows  | `{FOLDERID_RoamingAppData}\thokr\config`          |     C:\Users\colby\AppData\Roaming\thokr\config |

## Roadmap

- [ ] ⚡️ Performance
  - Right now there are known performance issues surrounding the rendering of
    the tui at each tick interval and/or key press. Ideally each render uses the
    prior render as a base and only makes the necessary adjustments (possibly
    using
    [StatefulWidget](https://docs.rs/tui/0.10.0/tui/widgets/trait.StatefulWidget.html)),
    but I haven't been able to figure that out yet.
- [ ] 🔠 Multi-language support
  - I decided not to launch thokr with languages besides english because of some
    odd rendering issues I was experiencing when trying to input characters with
    accents. It's as though I'm not able to properly input the character in [raw
    mode](https://docs.rs/crossterm/0.3.0/crossterm/raw/index.html). I'd love to
    have that figure out before shipping other languages because I personally
    felt the experience was a bit jarring. I'll open an bug report for it with
    more details and replication steps -- would love more eyes on that problem!
- [ ] 🧪 Tests
  - I've only written a small amount of tests at this point. I haven't sat down
    to really think through what tests look like when the output is dependent on
    the users terminal size, font size, etc. If you have any ideas for this please
    open up an issue and start the discussion!

## Contributing

All contributions are **greatly appreciated**.

If you have a suggestion that would make thokr better, please fork the repo and
create a [pull request](https://github.com/thatvegandev/thokr/pulls). You can
also simply open an issue and select `Feature Request`

1. Fork the repo
2. Create your feature branch (`git checkout -b [your_username]/xyz`)
3. Commit your changes (`git commit -m 'add some xyz'`)
4. Rebase off main (`git fetch --all && git rebase origin/main`)
5. Push to your branch (`git push origin [your_username]/xyz`)
6. Fill out pull request template

See the [open issues](https://github.com/thatvegandev/thokr/issues) for a full
list of proposed features (and known issues).

## License

Distributed under the MIT License. See [LICENSE.md](./LICENSE.md) for more
information.

## Acknowledgments

Check out these amazing projects that inspired thokr!

- [monkeytype](https://github.com/Miodec/monkeytype)
- [tui-rs](https://github.com/fdehau/tui-rs)
- [ttyper](https://github.com/max-niederman/ttyper)

## Follow

[![github](https://img.shields.io/github/followers/thatvegandev?style=social)](https://github.com/thatvegandev)
[![twitter](https://img.shields.io/twitter/follow/thatvegandev?color=white&style=social)](https://twitter.com/thatvegandev)
[![youtube](https://img.shields.io/youtube/channel/subscribers/UCEDfokz6igeN4bX7Whq49-g?style=social)](https://youtube.com/user/thatvegandev)
