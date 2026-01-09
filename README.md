🧠 What is Mnemo?
------------------
`mnemo` is a lightweight, fast command-line utility written in Rust that reminds you of installed executables both via standalone executable and via shell-hooks.

💡 Why does this even exists?
------------------------
I created Mnemo both as a Rust learning project and for personal usage as i find myself to be a very forgetful person. Then i thought it may come in handy to other absent-minded people out there so here it is for everyone to play with :)

💻 Examples
----------

```bash
$ mnemo --summary

📁 /usr/local/bin
   textarea (1)         - Browser based text editor

📁 /opt
   witr (1)             - Why Is This Running? This program provides you the info you need

```

```bash
$ mnemo --hint "ls | grep witr"
  🤖 : Were you looking for one of these executable(s)?
     - grep witr     ->    ⚙️  witr
```

(The last example is best to be used in combination with shell-hooks) 

🛠️ Installation
---------------
**Pre-built binaries (Recommended):**

1. Go to the project's Releases and grab the latest precompiled release
2. Move the `mnemo` binary into a directory on your `$PATH`

This is the suggested method because releases are built and packaged automatically by CI.

**Self-compile:**

Prerequisites:
- Rust toolchain (`rustup`, `cargo`, `rustc`) — https://rustup.rs
- Standard build tools for your distro (make, build-essential, etc.)

Build steps:

```bash
git clone <repo-url>
cd mnemo
cargo build --release
mkdir -p <your-destination-dir>/assets/hooks
cp target/release/mnemo <your-destination-dir>
cp default_config.yaml <your-destination-dir>/config.yaml
cp -r src/hooks/assets/* <your-destination-dir>/assets/hooks/
export PATH="$PATH:<your-destination-dir>"
```

🚀 Usage
--------
Mnemo supports two main modes of operation: as a standalone executable and as a shell hook integrated into your interactive shell (_**currently supported shells are bash and zsh**_).

- Standalone usage
  - Run `mnemo <command> [args]` from any terminal. This is ideal for scripts, CI, or one-off invocations.

- Shell hook usage (interactive)
  - Make sure `mnemo` is installed and available in your `$PATH`.
  - Set the needed shell hook (which you can find under `src/hooks/assets/`) using the built-in helper command `--set-shell-hook`.
  - After installing the hook, your shell will automatically call `mnemo -H $command` (where command is the last command in your history) after every input. This may sound bad but don't worry: `mnemo` has a built-in confidence threshold calculation that prevent it to clutter your output. You won't hear of him unless there's an effective match to what you typed and an executable in the config directories. Anyway if you don't feel comfortable with this usage you can just remove or comment out the line right under '# Mnemo Hook' in your shell `.*rc` config file after setting the hook and Mnemo will stop suggesting things automatically.
  
  Another way to use Mnemo is to call it explicitly. The hooks enable tighter integration and contextual behavior which is the way this tool is mainly intended to be used but eventually the choice is left to you!

🔧 Configuration
----------------
Upon `mnemo`'s first manual run (launching `mnemo --help` is enough) it will create its own folder under `~/.config` and will place its config files grabbed from repository's folders there. these files are the following:
- **A Default configuration**: the file `default_config.yaml` in the repository root will be copied over and renamed `config.yaml`. Any edit to the configuration should be done on `~/.config/mnemo/config.yaml`.
- **Default Shell hooks**: `~/.config/mnemo/assets/hooks` will be created and files from `src/hooks/assets` will be copied over here trimming the "default_" prefix. Again if you want to customize how the hooks work you can do it but please make sure to alter files in the destination directory.

🤝 Contributing
---------------
Mnemo is happy to see some love!

Feel free to open issues for bugs or feature requests,
fork, implement, and send PRs.
