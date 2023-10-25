# Convert images to braille

This is a very simple just-for-fun tool to create unicode braille art given some image as input.

## Usage

```
braille-art <image_path> [width]
```

The `width` specifies the number of characters the output image is allowed to take up in in width.

## Example

The image in `images/cat.jpg` is turned into:

```
$ cargo run -- images/cat.jpg 30
⠀⠀⠀⣠⣶⣶⣤⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⣼⣿⠋⠀⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢻⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡄⠀⠀⠀
⠀⠀⠈⢻⣿⣦⣄⣀⣤⣤⣤⣤⣄⣀⣀⣀⠀⠀⠀⠀⠀⢀⣠⣶⣾⣿⣷⣆⠀⠀
⠀⠀⠀⠀⢹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣶⣶⣿⣿⣿⣿⣿⣿⣿⠇⠀
⠀⠀⠀⢀⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠋⠀⠀⠀⠀
⠀⠀⠀⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀
⠀⢀⣶⣿⣿⣿⢿⣿⣿⣿⠿⠿⠟⠛⠛⠻⣿⣿⣿⠿⠿⣿⣿⣿⣿⠀⠀⠀⠀⠀
⢀⣾⠏⠉⠀⠀⣾⣿⡿⠃⠀⠀⠀⠀⠀⠀⣿⡿⠁⠀⠀⠀⠙⢿⣿⣧⡀⠀⠀⠀
⢸⡏⠀⠀⠀⠀⠻⣿⡀⠀⠀⠀⠀⠀⠀⢸⡿⠁⠀⠀⠀⠀⠀⠀⠉⠻⣷⣄⠀⠀
⠀⠁⠀⠀⠀⠀⠀⠈⠛⠶⠂⠀⠀⠀⠀⠘⠛⠃⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠋
```
