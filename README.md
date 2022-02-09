# o

`o` is a small, UNIX-friendly file finder and opener.

It works with `fzf` (in the terminal) and `dmenu` (elsewhere) to provide
easy-access to commonly used files.

## Usage

With `o`, you get to choose how your files are displayed and opened. All
subcommands are user-defined in `config.yml`.

To find and open an image using the below configuration, simply run:

```bash
$ o images
```

If you're currently in a terminal or tty, this will pull up `fzf` with all files
defined by the `images` category. If running outside of a tty (eg. directly from
`dmenu`), you'll see the same interface but using `dmenu` instead.

## Configuration

Configuration is very straightforward:

```yaml
# ~/.config/o/config.yml

# categories are user-defined subcommands
categories:
  # usage:
  #   dirs: where should o look for files?
  #   filetypes: which filetypes should be included?
  #   command: open files with which command? (default: xdg-open)
  #   wait: wait for the command to finish? (default: false)

  # basic example (view images using xdg-open)
  images:
    dirs: ["~/Images/"]
    filetypes: [".jpg", ".png"]

  # gui example (view pdfs using zathura)
  docs:
    dirs: ["~/Documents/", "~/Downloads"]
    filetypes: [".jpg", ".png"]
    command: "zathura"

  # tty example (edit local .rs and .toml files using nvim)
  scripts:
    dirs: ["."]
    filetypes: [".rs", ".toml"]
    command: "nvim"
    wait: true # nvim runs in the terminal, so we need to wait
```

## Development

`o` is in very early development, and there are bound to be many bugs and
changes over the next few months.

Planned features are:

- Flags to specify alternate config files.
- Flags to override category options.
- A `guicommand` option, for when the user isn't in a terminal.
