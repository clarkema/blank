# blank

blank is a tiny program that blanks your terminal.  That’s it.

Sometimes I just want to run a simple terminal or Vim instance without a ton of
multitasking or other windows.  On modern wide and ultrawide monitors, running
a terminal full-screen results in the text being piled up against the left side
of the screen, which looks weird and is uncomfortable.

What I'd like is something like Emacs’ [Olivetti
mode](https://github.com/rnkn/olivetti) to balance the text in the middle of
the screen.  Some terminal emulators allow you to split the window; I generally
end up using `tmux` and creating a three column layout.  This leaves you with
spare shells with their prompts hanging out on either side of your workspace
though... `blank` to the rescue!

Run `blank` in your side columns and it will leave you with perfectly blank
terminals, allowing you to focus on the centre area.

There are various other ways of achieving this, including a variety of tmux
tricks and support scripts that do things like create panes running `sleep`.

However, `blank` has the advantage that the side panes are still just running
normal terminals; if you need to you can quit `blank` to refer to something or
do some temporary multitasking, and then run it again to hide distractions.
Rather than actually clearing the terminal, `blank` works by switching over
to the alternate screen and back, so your shell history is preserved.

## Installation

Currently there are no pre-built binaries available.  You’ll need a [Rust
toolchain](https://rustup.rs/) installed, then from the repository root you can
run:

```
cargo build --release
```

Then just copy `target/release/blank` to somewhere in your path.

Quit `blank` either by hitting Enter or with Ctrl-C.

## zsh integration

If you find yourself hopping in and out of blank mode frequently and you use
zsh, you can bind a key to run blank for you:

```zsh
bindkey -s '^x^b' 'blank^M'
```
