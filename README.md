# GraphX

Application for graph visualization and analysis.

The app will use the user's [data directory](https://tauri.app/v1/api/js/path/#datadir) to store the database file

Platform-specific
* Linux: Resolves to $XDG_DATA_HOME or $HOME/.local/share/graphx.db
* macOS: Resolves to $HOME/Library/Application Support/graphx.db
* Windows: Resolves to {FOLDERID_RoamingAppData}/graphx.db

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

## Build
Create the build
```
$ cargo tauri build
```

## Version History

* 0.1
    * initial development

## Resources

* [indradb](https://github.com/indradb/indradb)
* [regraph](https://github.com/Izhaki/regraph)
* [tauri](https://github.com/tauri-apps/tauri)