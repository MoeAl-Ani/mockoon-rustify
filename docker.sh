docker build -t mockoon-rust .
data_volume=$PWD/data
echo $data_volume
docker run --volume $data_volume:/data -it --rm mockoon-rust mockoon-rust --data /data/api-config.json