<p align="center"><img src="assets/muse.jpeg" alt="mythra" height="100px"></p>

<div align="center">
  <table>
  <tr>
  <td>Web API</td>
    <td>
    <a href="https://github.com/deven96/mythra/workflows/">
      <img src="https://github.com/deven96/mythra/workflows/Deploy%20to%20Heroku/badge.svg" alt="Deploy Status">
    </a>
    </td>
  </tr>
  <tr>
  <td>Build and Test</td>
    <td>
    <a href="https://github.com/deven96/mythra/workflows/">
      <img src="https://github.com/deven96/mythra/workflows/Build%20and%20Test/badge.svg" alt="Test Status">
    </a>
    </td>
  </tr>
  <tr>
  <td>Documentation</td>
    <td>
    <a href="https://bisoncorps.stoplight.io/docs/mythra/reference/Mythra.v1.yaml/">
      <img src="https://github.com/deven96/mythra/workflows/Deploy%20docs%20to%20Stoplight/badge.svg" alt="Docs Status">
    </a>
    </td>
  </tr>
  </table>
</div>

# Mythra

In my bid to learn rust I am trying to make a music web scraper



In order to use either the `ncurses` or the `pancurses` backend of a dependent library `cursive`, you will need the ncurses library installed on your system.

## Archlinux

```
pacman -S ncurses
```

## Ubuntu

```
apt-get install libncursesw5-dev
```

## Fedora

```
yum install ncurses-devel
```

## macOS

```
brew install ncurses
```

### Engines

- FreeMP3Cloud


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

<p align="center"><img src="assets/example.gif" alt="mythra example"></p>

## Deployment

The deployed API version from `mythra api` is available on [Heroku]([200~https://bisoncorps.stoplight.io/docs/mythra/reference/Mythra.v1.yaml). Please read the API documentation for usage

## License

This project is opened under the [GNU AGPLv3](https://github.com/deven96/mythra/blob/master/LICENSE) which allows very broad use for both academic and commercial purposes.


## Credits
Library/Resource | Use
------- | -----
[Stoplight](https://stoplight.io) | Generating API docs
[Fantoccini](https://github.com/jonhoo/fantoccini/) | Scraping javascript sites using chromedriver/geckodriver
