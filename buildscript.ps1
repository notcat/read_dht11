$TARGET_HOST = "pi@192.168.0.16"
$TARGET_PATH = "/home/pi/read_dht11"
$SOURCE_PATH = "./target/armv7-unknown-linux-gnueabihf/release/read_dht11"
$TARGET_ARCH = "armv7-unknown-linux-gnueabihf"

cargo build --release --target=$TARGET_ARCH
scp $SOURCE_PATH $TARGET_HOST`:$TARGET_PATH
ssh -t $TARGET_HOST $TARGET_PATH