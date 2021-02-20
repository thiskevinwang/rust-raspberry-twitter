# Rust Raspberry Pi Twitter Bot

This project generates a Docker container that is meant to run on a Raspberry PI 4 Model B (**arm 7, 32 bit**).
This Tweets values detected by an attached [SenseHat](https://www.raspberrypi.org/products/sense-hat/)

## Annoying tweets!

<blockquote class="twitter-tweet" data-dnt="true" data-theme="dark"><p lang="en" dir="ltr">👋 Hello from 🍒 <a href="https://twitter.com/hashtag/RaspberryPi?src=hash&amp;ref_src=twsrc%5Etfw">#RaspberryPi</a> ⚙️ <a href="https://twitter.com/hashtag/Rust?src=hash&amp;ref_src=twsrc%5Etfw">#Rust</a> 🐳 <a href="https://twitter.com/hashtag/Docker?src=hash&amp;ref_src=twsrc%5Etfw">#Docker</a><br>---<br>🕐 01:44:16 PM<br>📅 Tuesday November 17, 2020<br>🌡 CPU: 49.173°C (max: 51.121°C)<br>💧 Humidity: 24.5%<br>Temp 1: 98.70 ºF<br>Temp 2: 95.15 ºF</p>&mdash; Kevin Wang (@thekevinwang) <a href="https://twitter.com/thekevinwang/status/1328755852170420225?ref_src=twsrc%5Etfw">November 17, 2020</a></blockquote>

![](imgs/tweet.png)

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- A Raspberry Pi 4 Model B (**arm 7, 32 bit**), w/ internet

## Running

```sh
cp .env.example .env
# Manually update env vars
```

```sh
# Build the image
docker build --rm -t rust-bot .
# Run the container
docker run --rm -it -id --name do-stuff rust-bot:latest
```

## Possible Gotchas

- The code won't run on a desktop machine.
- It'll need to be run with Docker on a Rasperry Pi.
