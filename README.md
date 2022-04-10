# o

o is a small, UNIX-friendly file finder and opener.

It works with [fzf](https://github.com/junegunn/fzf) (in the terminal) and
[dmenu](https://tools.suckless.org/dmenu/) (elsewhere) to provide a configurable
way to... well... open things.

## Usage

Every system is different, so there's no functionality by default. Rather,
subcommands are defined manually in `$XDG_CONFIG_HOME/o/config.yml`.

For example: To find and open an image using the below configuration, simply
run:

```shell
$ o images
```

If you're currently in a terminal or tty, this will pull up fzf with all of the
files in all of the directories listed in the `images` category. If running
outside of a tty, you'll see the same list but piped through dmenu instead.

## Configuration

Configuration is fairly straightforward. The following example contains all of
the options currently supported.

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

Be aware that o is in very early development. Not all planned features have been
implemented yet, and there are bound to be many bugs and changes over the next
few months. I'm trying to maintain backward compatibility, but things might
break every now and then.
