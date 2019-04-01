# SocLocker Client

SocLocker Client implements the client facing side of the SocLocker reference
implementation.

Architecturally, it is built using [Vue.js](https://vuejs.org/) as an 
underlying web framework, with [TypeScript](https://www.typescriptlang.org/) as
it's core implementation language: transpiled into ECMAScript with high
compatability for older web browsers.

The client is designed to be served from the same host as the server, with the
client being interacted with on `https://host.name/` and server being
interacted with on `https://host.name/api/`.

## Building

```
npm install
npm run build
```

This will create the `./dist` folder which contains all of the files that need
to be served from the root of the host.