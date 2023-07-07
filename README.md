
# clean-local

A simple CLI to remove all the files and folders included in the `.gitignore` file. 

- Written in RUST
- Supports `glob` and `wildcard` paths in `.gitignore`
- Pretty fast as its native binary
- Config support (TODO)
- winget support (TODO)

Build by [Vilva](https://twitter.com/vilvaathibanpb)



## Installation

### MacOS/Linux

Install clean-local with `homebrew`

```bash
  brew tap vilvaathibanpb/tap         
  brew install clean-local
```

For `arm64` architecture use the below command

```bash
  arch -arm64 brew install clean-local
```

### Windows

For Windows, directly download from the [releases](https://github.com/vilvaathibanpb/clean-local/releases).

Winget support will be there soon.

## Usage

- Run the command `clean-local` in the root of the project. 

**Note**: Make sure `.gitignore` file is present on the path where the command is executed.

