# Taliyah

A feature-packed bot for Discord servers, written in Rust with Serenity and various other libraries.

[![Invite Taliyah][invite-badge]][invite-link]
[![License][license-badge]][license-link]
[![Dependency Status][dependency-badge]][dependency-link]
[![GitHub Actions Build Status][github-actions-badge]][github-actions-link]

> [!IMPORTANT]
>
> **November 2024 update**: Development on Taliyah has been paused for the forseeable future due to a lack of desire and
> drive to continue maintaining the project. However, the codebase will still receive updates to dependencies to keep the
> project as free of security vulnerabilities as possible. Taliyah will however stay on Serenity 0.12.x and Poise 0.6.x,
> even if they get new versions, during this pause in development. It should additionally be said and reiterated that
> development on the project is not ceasing for good, but I am taking a break for at least the short term to try and get
> my drive and desire back to maintain this project. There is more information about the pause in development located
> [here](https://github.com/evieluvsrainbows/Taliyah/commit/698537bcb185bcba92e9a9adb1f950a6b8ad26e5), going further into
> detail on why I am pausing the development of Taliyah, and I encourage anyone who ends up seeing this to read that.
>
> 🚧 **Pardon The Dust** 🚧
>
> Taliyah is currently undergoing a significant rewrite based around the (relatively) new Poise command framework. Therefore,
> the contents of the README as shown below are no longer appropriate, however are retained for posterity. Please do not
> use the contents of this README as a guide on how to setup Taliyah until more of the rewrite has finished. Thank you, and
> again pardon the dust while this work is ongoing. 😁

Welcome to the official GitHub / GitLab repository for Taliyah, a bot for the Discord chat platform written in Rust with
the serenity library, as well as various other libraries. It should be noted that this project is still in a heavy WIP state,
however there are still a pretty robust set of commands implemented so far, including a near-complete suite of voice
commands, which I am very happy with. This project will be continulously improved and updated with more commands and features,
so please keep an eye on this repository for any new features and updates, as well as the changelog.

## Installation

> [!NOTE]
> Installation instructions do not exist for macOS and Linux yet. They will be added soon.

### Prerequisites

Before we can get Taliyah up and running, we'll need to install a couple pieces of software in order for Taliyah to actually
build and run. This will depend on your operating system, be it either Windows, macOS or Linux. On Windows, this means you'll
need to have Visual Studio 2022 installed, be it the full IDE or the Build Tools, and Rust itself. On macOS, you will need
the Xcode Developer Tools, as it includes the system compiler (`clang`) necessary to build Rust programs and libraries, or
you could also go with simply installing Rust with `homebrew` or MacPorts. On Linux, you don't need to install anything in
most cases, as most Linux distributions such as Ubuntu and Fedora already have the `gcc` toolchain installed, however if
desired this can be switched to the same `clang` compiler as macOS by installing it through your respective package manager,
or through `homebrew` or MacPorts as previously mentioned.

You will also need Git, in order to be able to clone this repository to your system. Git can be downloaded either from the
Git website (located [here](https://git-scm.com/download/win) if you're on Windows), the package manager provided by your
distribution of choice, or, on macOS, with `homebrew` or MacPorts.

All in all, you will need the following prerequisites for Taliyah to build and run:

* Visual Studio 2022 / Visual Studio 2022 Build Tools (*Windows (non-WSL) only*)
* Xcode and the Command Line Tools (macOS)
* Git, preferably latest stable
* Rust, preferably latest nightly

#### Windows

To install Visual Studio 2022, or the Visual Studio 2022 Build Tools, please visit the Visual Studio website, which can be
accessed by [clicking here](https://visualstudio.microsoft.com/), and click on the Download Visual Studio button at the top
of the page to download the Visual Studio installer. When in the installer, choose any edition you prefer; the Community
edition works fine. Or, if you would just like to install the Build Tools instead of installing the IDE, you can visit
[this URL](https://visualstudio.microsoft.com/downloads/), scroll down to All Downloads section, expand the "Tools for
Visual Studio"section, and click the Download button next to "Build Tools for Visual Studio 2022".

Next, we will need to install the `rustup` tool, which allows easy managemnt of Rust toolchain installations as well as easy
updating of Rust when new versions are available. To download rustup, visit the website located [here](https://rustup.rs/)
and click on `rustup-init.exe` which will download the rustup initialization utility to your system. When it is downloaded,
run the tool and follow the instructions to install Rust on your system.

#### Windows Subsystem for Linux (WSL) 2

> [!WARNING]
> Please do NOT use the version of Rust provided by your distribution's repositories. The version provided by your distribution
> is likely out of date compared to what the current version of Rust actually is (1.80.0 at the time of writing); therefore
> rustup should be used instead.

Installing Rust inside of Windows Subsystem for Linux is even easier and doesn't require Visual Studio 2022 or the Build
Tools. It should be noted as well that these instructions also apply to machines running Linux natively, as WSL is just a
virtual machine that has been tightly integrated into Windows.

First, as not all distributions include GCC by default, you will need to install GCC via your Linux distribution's package
manager. As there are multiple Linux distributions as well as multiple package managers on said distributions, instructions
cannot be provided. I recommend looking up how to install your respective distribution's build tools metapackage, which includes
GCC as well as other tools useful for development.

When GCC and the other build tools are installed, run the following command to install `rustup`:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> [!TIP]
> To install `rustup`, `rustc`, and `cargo` to a different install location, create both the `RUSTUP_HOME` and `CARGO_HOME`
> system environment variables under the System Properties window in Windows, under Advanced. The `rustup` tool does not
> currently offer a user-friendly way of changing the instal location, but this is an option if you would like to install
> Rust to a different drive or folder.

### Installing the Bot

Now, clone the Taliyah repository to your system using git:

```bash
git clone https://github.com/evelynharthbrooke/Taliyah.git
```

If you'd like to use GitLab for the cloning process instead of GitHub, you can do that too. Just use the following command
instead to clone from Taliyah's GitLab mirror.

```bash
git clone https://gitlab.com/evelynharthbrooke/Taliyah.git
```

Then, `cd` into the directory you downloaded Taliyah to:

```bash
cd Taliyah
```

### Configuring the Bot

> [!CAUTION]
> Some parts of these instructions are out of date, mainly the "owner" config field. No commands are present in Taliyah
> yet that require owner-level permissions, therefore, that part can be ignored.

Now we can set up Taliyah. You will need to go to the developers site for Discord, and create a new application. You can
do this by going [here](https://discordapp.com/developers/applications/), logging in, and selecting "Create an application"
on the main page, and filling in the neccessary information. Once you have successfully created an application, click on
your application's card. Now, we'll have to create a "Bot user" for the application. You can do this by selecting "Bot"
on the left hand column, under OAuth2, and clicking "Add Bot". This will add a bot user to your application.

Now, for the fun part! Let's grab the bot's token. You can do this by clicking the "Click to reveal token" button underneath
the Username field on the bot page. Copy the token given to you. Now, in the bot's root directory, rename `config.sample.toml`
to `config.toml`, and open the file. Paste the token into the token field. While you have the file open, you may want to
take this opportunity to enter your Discord user ID in the "owner" field so you can use any owner-only commands that have
been added, as well as any API keys and usernames and passwords you'd like. I should note though that there is currently
no error catching implemented in any commands right now, so if you forget to add API keys or usernames/passwords, you will
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

[license-link]: https://github.com/evelynharthbrooke/Taliyah/blob/main/LICENSE.md
[license-badge]: https://img.shields.io/github/license/evelynharthbrooke/Taliyah.svg?color=ff1f46&style=flat-square

[github-actions-link]: https://github.com/evelynharthbrooke/Taliyah/actions?query=workflow%3A%22Check+Project%22
[github-actions-badge]: https://github.com/evelynharthbrooke/Taliyah/workflows/Check%20Project/badge.svg
