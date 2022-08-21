# Rust Template

![build](https://github.com/bwte/template/workflows/Rust/badge.svg)
![release](https://github.com/bwte/template/actions/workflows/release.yml/badge.svg)

A project template for Rust, helping to structure your projects blazingly fast ⚡.

## Features 🔥

- Code-ready for binary projects.
- Add amazing features yourself 😋.

## Download

Available releases can be downloaded for your platform of choice on the [Releases](https://github.com/bwte/template/releases) page.

## Usage

☑️ Steps to set up your own personal project:

- Use this repository as a template but make sure to copy the branches too.
- 🔧 Update the `Cargo.toml` with your desired information for your project.
- ⚙️ Update the `release.yml` to change the release branch to your preferred branch and make sure to change the following fields to your likings.
```yml
# FILE EXPORT CONFIGURATION.
release_version: 0.1.0 # Change your version here.
bin_name: template # You might want to change it to your preferred export name.
# END OF CONFIGURATION.
...
# RELEASE TAB CONFIGURATION.
tag_name: 0.1.0 # Change your version here.
release_name: 📦 Template 0.1.0 # You might want to change it to your preferred release name.
prerelease: true # Set to false to create a "stable release".
draft: false # Set to true to create a draft release. This will keep your release private, and you would need to manually publish it.
# END OF CONFIGURATION.
```
- 🔖 Change the name inside the `LICENSE` file, or replace it with a new license.
- 📜 Change the email inside the `CODE_OF_CONDUCT.md` file, or replace it with one fit to your likings.
- 💰 Update the `FUNDING.yml` file with your username or just delete it.
- 📄 Update this `README.md`.
- 📰 Update the `CHANGELOG` and add your amazing features!

## Releasing ✨

- Push your changes to your preferred releases branch, you have supplied in the `release.yml` file.
- It will use the title of the merge commit as the release name and the description of the merge commit as the release description.
- 🤖 The release will be created automatically!

## Building 🔨

If desired, you can build this project yourself. You will need a working `Rust` and `Cargo` setup. [Rustup](https://rustup.rs/) is the simplest way to set this up on either Windows, Mac or Linux.

Once the prerequisites have been installed, compilation on your native platform is as simple as running the following in a terminal:

```
cargo build --release
```

## Compatibility 💻

This template supports running on every platform.

## Contribution 🚩

Found a problem or have a suggestion? Feel free to open an issue.

## License

This template itself is licensed under the [The Unlicense](LICENSE) and includes this as the default project license.
