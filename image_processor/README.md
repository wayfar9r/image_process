Examples

#RUN blur effect#

RUST_LOG=info cargo run -- --input ../img/snow_leopard1.png --output output.png --plugin libblur_plugin --params params_blur.txt --plugin-path ../target/debug

#RUN mirror effect#

RUST_LOG=info cargo run -- --input ../img/snow_leopard1.png --output output.png --plugin libmirror_plugin --params params_mirror.txt --plugin-path ../target/debug
