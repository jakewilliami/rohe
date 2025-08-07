# -*- mode: just -*-

# Define build target
#
# NOTE: this can be changed with an environment variable
target := env("TARGET", "release")
build_option := if target != "debug" { "--" + target } else { "--" }

#  Define binary/target
#
# NOTE: There are a few different ways to approach this [1].
#
# If we were in `build.rs` we could use the environment variable [2]:
#
#     env_var("CARGO_PKG_NAME")
#
# If the project directory is also the name of the project, and the justfile is
# in the root of the project (both quite reasonable assumptions, but not always
# the case), then we can use that directory name [3, 4]:
#
#     file_stem(justfile_dir())
#
# Otherwise, you could use `cargo metadata` [5]:
#
#     if command -v jq 2>&1 >/dev/null:                              # Ensure jq available
#         cargo metadata --format-version=1 --color=never --no-deps  # JSON output
#              | jq --raw-output '.packages[0].name'                 # Get package name
#
# Alternatively, we can parse the package ID [6]:
#
#     cargo pkgid                    # Output like "path+file:///home/gl#1.0.0"
#         | awk -F'/' '{print $NF}'  # Get final component of path ("gl#1.0.0")
#         | cut -d'#' -f1            # Remove version number
#
# This is an attractive alternative because it doesn't require `jq` as a dependency.
#
# Unfortunately, parsing the package ID does not work in CI pipelines because it
# requires parsing from a Cargo.lock file that may not necessarily exist.  Global
# variables in Just seem to be defined before `build-core` is run, so `cargo pkgid`
# silently fails and stores an empty variable.
#
# Rather, we can adapt a version using a similar commant for parsing a `pyproject.toml`
# from a Python project [7].  This is what I ended up doing.  Unfortunately, it
# requires UV, where I did not want to add additional dependencies.  However, I am
# using UV for running pre-commit anyway, so I think this is a reasonable dependency.
#
# Note that this would not necessarily be appropriate if the Rust package was a
# library rather than a binary.
#
# [1]: https://stackoverflow.com/q/75023094
# [2]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
# [3]: https://just.systems/man/en/functions.html#justfile-and-justfile-directory
# [4]: https://just.systems/man/en/functions.html#path-manipulation
# [5]: https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
# [6]: https://doc.rust-lang.org/cargo/reference/pkgid-spec.html
# [7]: https://stackoverflow.com/a/79276213
project_dir := justfile_dir() + "/"
bin_name := `cargo pkgid | awk -F'/' '{print $NF}' | cut -d'#' -f1`
target_bin := "target" / target / bin_name
doc_file := "target" / "doc" / bin_name / "index.html"

# Build the project and copy and strip the resulting binary to the root project
build: build-core
    cp -f {{target_bin}} {{project_dir}}
    strip {{bin_name}}

[private, macos, linux]  # TODO: NOT Windows
cp-to-root:


# Core build recipe using `cargo`, used by main build recipe
[private]
build-core:
    cargo build {{build_option}}

# Build the project for Windows
[macos]
build-win: rust-target-win
    # https://stackoverflow.com/a/62853319
    cargo build --target x86_64-pc-windows-gnu {{build_option}}

# Build the project for Windows
[linux]
build-win: get-mingw-w64
    # https://stackoverflow.com/a/62853319
    cargo build --target x86_64-pc-windows-gnu {{build_option}}

[private, linux]
get-mingw-w64: rust-target-win
    dpkg -l | grep -qw mingw-w64 || sudo apt install -y mingw-w64

# Install required Rust toolchain for cross-compiling to Windows
[private, unix]
rust-target-win:
    # TODO: figure out requirements on BSD/macOS
    rustup target add x86_64-pc-windows-gnu

# Check project formatting and linting
fmt: pre-commit clippy cargo-fmt

[private]
pre-commit:
    uvx pre-commit run --all-files

[private]
cargo-fmt:
    cargo fmt --all -- --check

[private]
clippy:
    cargo clippy --all --all-targets --all-features -- --deny warnings --allow clippy::uninlined-format-args

# Update project dependencies in Cargo.lock
update:
    cargo update --locked --package {{bin_name}}

# Run tests
test:
    cargo test --all

# Generate doc
doc:
    cargo doc --open

