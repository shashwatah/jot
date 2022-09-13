# jot

<a href="LICENSE"><img alt="MIT License" src="https://img.shields.io/apm/l/atomic-design-ui.svg?"></a>
<a href="https://github.com/araekiel/jot/releases/tag/v0.1.0"><img alt="Release" src="https://img.shields.io/badge/release-v0.1.0-red"></a>
<a href=""><img alt="Cargo" src="https://img.shields.io/badge/cargo-jot-blue"></a>

<p>
  <a href="#highlights">Highlights</a> •
  <a href="#notes">Notes</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#build-from-source">Build from Source</a> •
  <a href="#authors">Authors</a> •
  <a href="#license">License</a>
</p>


Jot is a CLI alternative for obsidian built with rust.
<br>
While Obsidian is an excellent knowledge management tool, it proves to be overkill when all I want to do is jot down (pun intended) some notes and manage them locally, plus the GUI can often slow things down. Since Obsidian doesn't have a CLI client or an API that can enable the creation of one, I decided to make a fast and lightweight CLI alternative.
<br>

<img alt="Screenshot" src="assets/imgs/jot.png"/>

## Highlights
- Jot is under active development.
- Jot uses files and folders for notes and vaults just like Obsidian.
- Jot is aimed at reducing the time taken to perform tasks, so, each command is a two letter abbreviation of the word(s) that describe it.  

## Notes
- App data is stored in config and data files in locations generated by the [directories](https://crates.io/crates/directories) crate. It is advised that these files not be messed with, since atm there's no way to automatically fix them.
- App config has two fields: *editor* & *conflict*.
    - *editor* by default is set to *nvim* and conflict to *true*.
    - *conflict* field tells the app if the editor conflicts with it for control over the terminal. Set it to *true* for editors like *nvim* and *false* for editors like *notepad*.

## Installation

### With cargo

Run the following command to install jot with cargo:

```bash
$ cargo install jot
```

### v0.1.0 Executable Download

[Jot v0.1.0](https://github.com/araekiel/jot/releases/download/v0.1.0/jot.exe) (.exe)

## Commands

### `vl`

Use this command to create a vault.
<br>
This command needs an absolute path that already exists or it will fail.

<img alt="jot-vl" src="assets/gifs/jot-vl-cr.gif">

Pass in no arguments to get a list of vaults.

<img alt="jot-vl" src="assets/gifs/jot-vl.gif">

Use the *-l* flag to get vaults' locations.

<img alt="jot-vl" src="assets/gifs/jot-vl-l.gif">

<hr>

### `nt`

This command will create a note in current folder.

<img alt="jot-vl" src="assets/gifs/jot-nt.gif">

<hr>

### `fd`

This command will create a folder in current folder

<img alt="jot-vl" src="assets/gifs/jot-fd.gif">

<hr>

### `en`

Use this command to enter a vault.

<img alt="jot-vl" src="assets/gifs/jot-en.gif">

<hr>

### `op`

Use this command to open a note.

<img alt="jot-vl" src="assets/gifs/jot-op.gif">

<hr>

### `cd`

Use this command to change current folder with a path from the current folder.

<img alt="jot-vl" src="assets/gifs/jot-cd.gif">

<hr>

### `ls`

Use this command to print dir tree of the current folder

<img alt="jot-vl" src="assets/gifs/jot-ls.gif">

<hr>

### `rm`

Remove an item (*vl*, *nt* or *fd*).

<img alt="jot-vl" src="assets/gifs/jot-rm.gif">

<hr>

### `rn` 

Rename an item (*vl*, *nt* or *fd*).

<img alt="jot-vl" src="assets/gifs/jot-rn.gif">

### `mv`

Move an item (*vl*, *nt* or *fd*).

<img alt="jot-vl" src="assets/gifs/jot-mv.gif">

### `vm` 

Move an item (*nt* or *fd*) to a different vault.

<img alt="jot-vl" src="assets/gifs/jot-vm.gif">

<hr>

### `cf`

Use this command to get the value of a config field.

<img alt="jot-vl" src="assets/gifs/jot-cf.gif">

Pass in a value along with the above command to set the value of the config field.

<img alt="jot-vl" src="assets/gifs/jot-cf-set.gif">

## Build from Source

### Prerequisites

- Git is need to clone the repository on your machine.
- Cargo is needed to compile the app.

### Installation & Configuration

Clone the repo and cd into the directory: 

```bash
$ git clone https://github.com/araekiel/jot.git
$ cd jot
```

Run the following command to install dependencies and build/compile the app. 

```bash
$ cargo build 
```

Then run the executable created in *target/debug/*.

Or, run the app directly:

```bash
$ cargo run -- *args*
```

Pass in commands and arguments after *'--'*.

## Authors

- **araekiel** - [Github](https://github.com/araekiel)

## License

[MIT License](https://github.com/araekiel/jot/blob/master/LICENSE) | Copyright (c) 2022 Kumar Shashwat
