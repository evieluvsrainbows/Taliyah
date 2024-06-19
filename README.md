# Taliyah

A feature-packed bot for Discord servers, written in Rust with Serenity and various other libraries.

[![Invite Taliyah][invite-badge]][invite-link]
[![License][license-badge]][license-link]
[![Dependency Status][dependency-badge]][dependency-link]
[![GitHub Actions Build Status][github-actions-badge]][github-actions-link]

> [!IMPORTANT]
> 🚧 **Pardon The Dust** 🚧
>
> Taliyah is currently undergoing a significant rewrite based around the (relatively) new Poise command framework. Therefore,
> the contents of the README as shown below are no longer appropriate, however are retained for posterity. Please do not use
> the contents of this README as a guide on how to setup Taliyah until more of the rewrite has finished. Thank you, and again
> pardon the dust while this work is ongoing. 😁

Welcome to the official GitHub / GitLab repository for Taliyah, a bot for the Discord chat platform written in Rust with
the serenity library, as well as various other libraries. It should be noted that this project is still in a heavy WIP state,
however there are still a pretty robust set of commands implemented so far, including a near-complete suite of voice
commands, which I am very happy with. This project will be continulously improved and updated with more commands and features,
so please keep an eye on this repository for any new features and updates, as well as the changelog.

## Installation

### Prerequisites

> [!CAUTION]
> These prerequisites are currently out of date while the bot transitions to being based around poise and Discord's new
> slash command system instead of serenity's old command framework; therefore these instructions should not be followed.
> I am also moving to a macOS-based workflow during the week of June 24, so when these instructions are updated,
> they will be primarily based around macOS and the Homebrew package manager.

Alright, before we can get Taliyah up and running, we'll need to install a couple pieces of software in order for Taliyah
to actually build and run. This will depend on your operating system, be it either Windows, macOS or Linux. On Windows,
this means you'll need Visual Studio 2022 installed, be it either the full IDE (any edition works fine) or just the Visual
Studio 2019 Build Tools, and Rust itself. On macOS, you will need the Xcode Developer Tools, as it includes the system
compiler (`clang`) necessary to build Rust programs and libraries, or you could also go with simply installing Rust through
the `homebrew` tool. On Linux, you don't need to install anything in most cases, as most Linux distributions such as Ubuntu
and Fedora already have the `gcc` toolchain installed, however if desired this can be switched to the same `clang` compiler
as macOS by installing it through your respective package manager, or through `homebrew` as well.

Across *all* operating systems, however, you will need to install the PostgreSQL database server, version 13 or later, as
that is required for the database. For the voice functionalities provided by Taliyah, you will need Opus, FFmpeg, `youtube-dl`,
as well as a Lavalink-compatible voice server, e.g. Lavalink or Andesite, meaning you will also need Python and Java. Version
15 of the AdoptOpenJDK distribution is recommended, with the OpenJ9 runtime being a good option. For Python, version 3.2
or better work fine, however Python 3.9.1 or later is recommended as newer versions of Python perform faster. Version 2.6
or 2.7 are NOT, and I mean ARE NOT, supported. They are supported by the `youtube-dl` tool, but I will provide absolutely
ZERO support for these, as they are EOL and completely unsupported, even by the Python Software Foundation.

All in all, you will need the following prerequisites for Taliyah to build and run:

* Visual Studio 2022 / Visual Studio 2022 Build Tools (*Windows (non-WSL) only*)
* PostgreSQL, version 13 or later
* Opus 1.3.1 or later
* FFmpeg 3.4.8 or later
* youtube-dl
* Lavalink / Andesite
* AdoptOpenJDK 15 or later (OpenJ9 runtime)
* Python, version 3.2 or later
* Rust, preferably version 1.48 or later

#### Windows

> **TODO**: Add instructions for Java, Python, and other dependencies (for both Windows and WSL)

To install Visual Studio 2019, or the Visual Studio 2019 build tools, please visit the website for Visual Studio, which can
be accessed by [clicking here](https://visualstudio.microsoft.com/), hover over the Download Visual Studio button on the
tile for Visual Studio, and selecting any given edition. If you have a license for either Professional or Enterprise, select
either of those, but if you do not, the Community works fine too. Or, if you would just like to install the Build Tools instead
of installing the entire IDE, you can visit [this URL](https://visualstudio.microsoft.com/downloads/), scroll down to the
All Downloads section, expand the "Tools for Visual Studio 2019" section, and click the Download button next to Build Tools
for Visual Studio 2019.

Next, we will need to install the `rustup` tool, which allows us to very easily manage Rust toolchain installations as well
as easily update Rust when new versions are available. To download the tool, visit the website for the Rust programming language,
located [here](https://www.rust-lang.org/learn/get-started), or the Rustup website, located [here](https://rustup.rs/), and
select the 64-bit executable file to begin the process of initializing the Rustup utility.

#### With Windows Subsystem for Linux (WSL) 2

Installing Rust in the Windows Subsystem for Linux is even easier, and doesn't require Visual Studio 2019, or the Build Tools,
as the GNU Compiler Collection (gcc) is more than likely already installed for you. To install Rust, just run the following
command in a WSL terminal window and follow any instructions that are provided to you:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Rust may also be provided in the respective Windows Subsystem for Linux distribution you are using, however this is not recommended,
as the version of Rust available in the distribution's package repositories may be significantly outdated, due to the nature
of Ubuntu, Debian, and other non-rolling Linux distributions preferring to wait until new distribution versions to update
their packages to new major versions. For example, Ubuntu still has Rust 1.43.0 in their package repositories, a version
that was released in April of 2020, despite Rust 1.48.0 being the current stable version available, and installing Rust
through your system's package manager also removes the ability to have multiple Rust toolchains installed, which `rustup`
provides, among other features.

##### Advanced Notes

To install `rustup`, `rustc`, and `cargo` to a different folder than the default, create both the `RUSTUP_HOME` and the `CARGO_HOME`
system environment variables under the System Properties window in Windows, under Advanced. The `rustup` tool does not currently
offer a user-friendly way of changing the instal location, but this is an option if you would like to install Rust to e.g.,
a different drive.

> **TODO**: Add macOS and Linux install instructions

### Installing the Bot

Now, we can actually download Taliyah and set her up. This step 100% requires Git, as that is how we will
be downloading her.

```bash
git clone https://github.com/evelynharthbrooke/Taliyah.git
```

If you'd like to use GitLab for the cloning process instead of GitHub, you can do that too. Just use
the following command instead to clone from Taliyah's GitLab mirror.

```bash
git clone https://gitlab.com/evelynharthbrooke/Taliyah.git
```

Alright, now let's `cd` into the download directory where we downloaded Taliyah to. This works across any and all operating
systems, including Windows.

```bash
cd Taliyah
```

Now we can install Taliyah's dependencies. On Windows, you will need to install the `windows-build-tools`
package using npm, as Windows does not natively include build tools like Linux does. For macOS, just
install Xcode and the commandline tools.

#### Non-release variant (unoptimized, with debug symbols)

```bash
cargo build
```

#### Release variant (optimized, without debug symbols)

```bash
cargo build --release
```

Just be patient while this process completes. It may take a while to complete, depending on your Internet
speed as well as the speed of your system's SSD and/or hard drive.

### Configuring the Bot

> This section is currently out of date. This section will be updated soon.

Now we can set up Taliyah. You will need to go to the developers site for Discord, and create a new application.
You can do this by going [here](https://discordapp.com/developers/applications/), logging in, and selecting
"Create an application" on the main page, and filling in the neccessary information. Once you have
successfully created an application, click on your application's card. Now, we'll have to create a
"Bot user" for the application. You can do this by selecting "Bot" on the left hand column, under
OAuth2, and clicking "Add Bot". This will add a bot user to your application.

Now, for the fun part! Let's grab the bot's token. You can do this by clicking the "Click to reveal token"
button underneath the Username field on the bot page. Copy the token given to you. Now, in the bot's root
directory, rename `config.sample.toml` to `config.toml`, and open the file. Paste the token into the token
field. While you have the file open, you may want to take this opportunity to enter your Discord user ID
in the "owner" field so you can use any owner-only commands that have been added, as well as any API keys
and usernames and passwords you'd like. I should note though that there is currently no error catching
implemented in any commands right now, so if you forget to add API keys or usernames/passwords, you will
encounter an error when trying to run the respective commands, so that's why I strongly suggest doing so.

Now, we are pretty much done. Now, onto the final step, which is actually running Taliyah.

### Running the Bot

You have reached the final step of the install instructions. You're almost there. You just have to build
the bot and then start her up.

```bash
cargo run # (--release if you want to run the optimized variant)
```

Congratulations! You have (hopefully) successfully installed and set up Taliyah, and you can now add the bot to
any guild you'd like. (if you have the permission to of course)

### Licensing

Taliyah is licensed under the terms of the MIT License, a fairly unrestrictive license that gives you the power to do
mostly anything you want with this project, and is one of two licenses used by the Rust project itself alongside version
2.0 of the Apache License, meaning that this software should be 100% compatible. The full contents of the MIT license are
written in the `LICENSE` file, in the root project directory. Please read it in full to understand your full rights
with regards to this software.

[invite-link]: https://discordapp.com/oauth2/authorize?client_id=483499705108529163&scope=bot
[invite-badge]: https://img.shields.io/badge/invite-to%20your%20Discord%20server-7289da.svg?style=flat-square&logo=discord

[dependency-link]: https://deps.rs/repo/github/evelynharthbrooke/Taliyah
[dependency-badge]: https://deps.rs/repo/github/evelynharthbrooke/Taliyah/status.svg

[license-link]: https://github.com/evelynharthbrooke/Taliyah/blob/rust_rewrite/LICENSE.txt
[license-badge]: https://img.shields.io/github/license/evelynharthbrooke/Taliyah.svg?color=ff1f46&style=flat-square

[github-actions-link]: https://github.com/evelynharthbrooke/Taliyah/actions?query=workflow%3A%22Check+Project%22
[github-actions-badge]: https://github.com/evelynharthbrooke/Taliyah/workflows/Check%20Project/badge.svg
