# o

o is a small, UNIX-friendly file finder and opener.

It works with [fzf](https://github.com/junegunn/fzf) (in the terminal) and
[dmenu](https://tools.suckless.org/dmenu/) (elsewhere) to provide easy access to
commonly-used files.

## Usage

With o, you get to choose how your files are displayed and opened. All
subcommands are user-defined in `$XDG_CONFIG_HOME/config.yml`.

To find and open an image using the below configuration, simply run:

```shell
$ o images
```

If you're currently in a terminal or tty, this will pull up fzf with all of the
files in all of the directories listed in the `images` category. If running
outside of a tty (eg. directly from dmenu), you'll see the same list but piped
through dmenu instead.

## Configuration

Configuration is fairly straightforward:

```yaml
# ~/.config/o/config.yml

# categories are user-defined subcommands
categories:
  # usage:
  #   dirs: look in which directories?
  #   filetypes: which filetypes should be included?
  #   include: limit the search to any specific patterns?
  #   ignored: which patterns should be ignored (if any)?
  #   command: open files with which command? (default: xdg-open)
  #   full-path: select using the full path, or just the file name?
  #   terminal: does the command run in the terminal? (default: false)

  # basic example (view images using xdg-open)
  images:
    dirs: ["~/Images/"]
    filetypes: [".jpg", ".png"]
    ignored: ["*/screenshots/*"]

  # gui example (view pdfs using zathura)
  docs:
    dirs: ["~/Documents/", "~/Downloads"]
    filetypes: [".pdf"]
    command: "zathura"

  # tty example (edit local files using nvim)
  edit:
    dirs: ["."]
    command: "nvim"
    full-path: true
    terminal: true
```

## Development

o is in very early development, and there are bound to be many bugs and changes
over the next few months.

Planned features are:

- More flags
    - To specify alternate config files.
    - To override category options.
- Better integration with environment variables.
- Better help (show user-defined subcommands in the menu).
- `o` will use itself to open itself with a category.
