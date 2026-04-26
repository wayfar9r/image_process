# Examples

## RUN blur effect

### Linux

RUST_LOG=info cargo run -- --input ../img/snow_leopard1.png --output output.png --plugin libblur_plugin --params params_blur.txt --plugin-path ../target/debug

### Windows

_Powershell_

$env:RUST_LOG="info"; cargo run -- --input .\img\snow_leopard1.png --output output.png --plugin blur_plugin -- params .\image_processor\params_blur.txt --plugin-path .\target\debug

## RUN mirror effect#

### Linux

RUST_LOG=info cargo run -- --input ../img/snow_leopard1.png --output output.png --plugin libmirror_plugin --params params_mirror.txt --plugin-path ../target/debug

### Windows

_Powershell_

$env:RUST_LOG="info"; cargo run -- --input .\img\snow_leopard1.png --output output.png --plugin mirror_plugin -- params .\image_processor\params_mirror.txt --plugin-path .\target\debug
