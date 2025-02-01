# GaoDaoYi Cross Platform App by Tauri

## Tauri configs: Tauri + Vanilla


## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Cargo commands

- Init tools: ```cargo install create-tauri-app --locked```
- or: ```cargo install tauri-cli --version "^2.0.0" --locked```
- Development: ```cargo tauri dev```
- Build the final bundles: ```cargo tauri build```

## How I get current logo icons

1. Original picture is the assets/images/book-cover.jpg, from the original epub book
2. Use https://www.remove.bg/ to remove the background as a transparent bg png (with a little blur and shadow effects)
3. Then to upload the png to https://icon.kitchen, using "image" as the src, by selecting the mask color and padding size, the bg is using two gradient colors.
4. Will download a zip, copy out the best one "AppIcon~ios-marketing.png", for its res is 1024x1024, which is current src/assets/book-cover.png
5. Using this png as the src input of ```cargo tauri icon``` command, then all icon files will be generated under icons folder.
