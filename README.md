# Mythra

<!-- markdownlint-disable-next-line -->
<p align="center"><img src="assets/muse.jpeg" alt="mythra" height="100px"></p>

<!-- ![Mythra](assets/muse.jpeg) -->

| Build Type     | Status                                                                                                                                                                       |
| -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Web API        | [![Deploy status](https://github.com/deven96/mythra/workflows/Deploy%20to%20Heroku/badge.svg)](https://github.com/deven96/mythra/actions/)                                   |
| Build and Test | [![Test status](https://github.com/deven96/mythra/workflows/Build%20and%20Test/badge.svg)](https://github.com/deven96/mythra/actions/)                                       |
| Documentation  | [![Docs status](https://github.com/deven96/mythra/workflows/Deploy%20docs%20to%20Stoplight/badge.svg)](https://bisoncorps.stoplight.io/docs/mythra/reference/Mythra.v1.yaml) |
| Releases       | [![Release status](https://github.com/deven96/mythra/workflows/Release%20to%20GitHub/badge.svg)](https://github.com/deven96/mythra/releases)                                 |

In my bid to learn rust I am trying to make a music web scraper

In order to use either the `ncurses` or the `pancurses` backend of a dependent library `cursive`, you will need the ncurses library installed on your system.

## Archlinux

```sh
pacman -S ncurses
```

## Ubuntu

```sh
apt-get install libncursesw5-dev libssl-dev
```

## Fedora

```sh
yum install ncurses-devel openssl-devel
```

## macOS

```sh
brew install ncurses
```

### Engines

- FreeMP3Cloud
- MP3S

## Installation

With Rust installed

You must have [chromedriver](https://chromedriver.chromium.org/) available on path and running on port 4444

```bash
git clone https://github.com/deven96/mythra.git
chromedriver --port=4444 --headless

# develop

cargo update
cargo run

#run the pakage
./target/debug/mythra search --query "Justin Timberlake Mirrors"

```

Or download from Github [Releases](https://github.com/deven96/mythra/releases)

## Example

![Mythra example](assets/example.gif)
<!-- <p align="center"><img src="assets/example.gif" alt="mythra example"></p> -->

## Deployment

The deployed API version from `mythra api` is available. Please read the [API documentation](https://bisoncorps.stoplight.io/docs/mythra/reference/Mythra.v1.yaml) for usage

## Roadmap

View [Roadmap](https://github.com/deven96/mythra/pull/3#issue-537670800) discussion

## License

This project is opened under the [GNU AGPLv3](./LICENSE) which allows very broad use for both academic and commercial purposes.

## Credits

| Library/Resource                                    | Use                                                      |
| --------------------------------------------------- | -------------------------------------------------------- |
| [Stoplight](https://stoplight.io)                   | Generating API docs                                      |
| [Fantoccini](https://github.com/jonhoo/fantoccini/) | Scraping javascript sites using chromedriver/geckodriver |
