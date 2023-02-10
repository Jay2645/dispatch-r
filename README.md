# Dispatch-R

Dispatch-R is a model railroad automation software, similar to JMRI.

Note that this software is currently unfinished! There is no guarantee that things will work at all. For the time being - [stick with JMRI](https://www.jmri.org/).

## What it does

Dispatch-R is planned to be able to manage several aspects of your model railroad:

* Locomotive rosters, including indicating which locomotives are consisted together. (**Not made yet**)

* Programming decoders, similar to JMRI. (**Not made yet**)

* Layout control, including throttles, switches, etc. (**Not made yet**)

* CTC control built-in. (**Not made yet**)

* Automatic train manifest generation, with support for add-ons such as RFID chips inside the train cars. (**Not made yet**)

* WiThrottle support. (**Not made yet**)

* Connections to your smart home and IOT devices via software like [Home Assistant](https://www.home-assistant.io/) or protocols like MQTT. (**Not made yet**)

## Building from Source [![contributions welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg?style=flat)](https://github.com/Jay2645/dispatch-r/issues)

Contributions to the project are always welcome. This program is broken into 2 parts:

1. The frontend is a web application using Yew. This serves content on a webpage.

2. The backend is a Tauri application, which starts a webserver and renders it inside the window.

To set up your dev environment, [follow the Tauri documentation to make sure you can develop a Tauri app.](https://tauri.app/v1/guides/getting-started/prerequisites) This will also walk you through installing Rust.

[You should also set up Yew](https://yew.rs/docs/getting-started/introduction) and [Node.js](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) This is necessary for running the frontend. Once both are installed, go to `src-frontend` and run `npm install` to install all required Node.js modules.

In debug mode, Yew will start a server on localhost:8080, which Tauri connects to. Yew supports "live editing", so you can make changes to the frontend, save, and your changes will be compiled/automatically pushed to what you see in the window.

In release mode, the frontend is "baked" into the backend - you cannot do live updates. However, you can still connect to Tauri itself and see the output of the program on other machines.

## License

This program is released under the terms of the GNU General Public License (GPL) v3.
