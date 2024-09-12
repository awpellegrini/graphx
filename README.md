# GraphX

Application for graph visualization and analysis.

The app will use the user's [data directory](https://docs.rs/tauri/1.7.2/tauri/api/path/fn.data_dir.html) to store the database file

Platform-specific
* Linux: Resolves to $XDG_DATA_HOME/graphx.db or $HOME/.local/share/graphx.db
* macOS: Resolves to $HOME/Library/Application Support/graphx.db
* Windows: Resolves to {FOLDERID_RoamingAppData}/graphx.db

## Requirements

* [node](https://nodejs.org/en/download/package-manager)
* [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Dev mode

### Installation

Install the client dependencies
```
$ cd ./graphx-client && npm install
```

### Running the app

Launch the app in development mode
```
$ cargo tauri dev
```

## Building the application

Create the build
```
$ cargo tauri build
```

## Version History

* 0.1
    * initial development

## Resources

* [indradb](https://github.com/indradb/indradb)
* [regraph](https://github.com/Izhaki/regraph) / [react](https://react.dev/)
* [tauri](https://github.com/tauri-apps/tauri)