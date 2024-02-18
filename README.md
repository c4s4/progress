# Progress

This tool prints text progress for character count, start and due date.

## Installation

### Unix users (Linux and MacOSX)

Unix users may download and install latest *progress* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/progress/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/progress/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *progress* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/progress/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *progress*.

## Usage

Let's say you must produce a text of *20.000* characters for *2024-03-13* starting on *2024-02-14*, your article is in file *article.md*. To print your progress on 2024-02-18, you would run following command:

```
$ progress article.md 20000 2024-02-14 2024-03-13
text size: 1624, due size: 20000, due for: 2024-03-13, days left: 23
progress: 8%, should be: 14%, delta: -6%
```

*Enjoy!*
