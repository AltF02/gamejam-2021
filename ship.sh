cargo build --release --no-default-features
pwd
rm -rf build/gamejam-2021-linux.zip
mkdir build/linux
cp ./release/gamejam-2021 build/linux
cp -avr ./assets ./build/linux
zip -r build/gamejam-2021-linux.zip ./build/linux/*
rm -rf ./build/linux