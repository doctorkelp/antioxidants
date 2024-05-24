# Antioxidants
A collection of Rust tools to keep files organized and minimal.

## Tools and Context

These tools were more or less created with the downloads folder in mind. For me, this is my "shit drawer" (a place for everyhting without a place). I suspect the desktop may be yours as it is for many others.

### `dedupe`
`dedupe` only exists to minimize space usage. It isn't idempotent at present, so expect issues if you run it more than once in a directory. I'll try to make it idempotent in a future update.

### `directorycleanup`
`directorycleanup` is just a decluttering application. It's only goal is to organize a directory on your behalf. This application should be idempotent.