# Xbot: A chatbot built on Rust

This chatbot is specifically made to cater to the XboxEra Discord server. If you want to fork this and use it for your own servers, go right ahead. 

How to Build
------------

Make sure you have the [latest Rust version](https://www.rust-lang.org/) installed on your machine. 

This bot uses [Serenity](https://github.com/serenity-rs/serenity) in order to connect to the Discord REST API. Please make sure you install its dependencies in order to compile.

The following dependencies are required in order to compile:
```
openssl

(For Voice only)
libsodium
opus
ffmpeg
youtube-dl
```

If you're running on Windows, please follow [Serenity's guide](https://github.com/serenity-rs/serenity/wiki/Voice-on-Windows) in order to get the voice dependencies working.

Contributing
------------

If you wish to contribute to this project, please message me. I will then send you a Discord invite link to the testing server. 
