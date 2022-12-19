cargo clean
cargo build
docker kill $(docker ps -aq)
docker rm $(docker ps -aq)
docker build -t mockoon-rust .
data_volume=$PWD/data
echo $data_volume
docker run --volume $data_volume:/data -p8080:8080 -it --rm mockoon-rust mockoon-rust --data /data/api-config.json