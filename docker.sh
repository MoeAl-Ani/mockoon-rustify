docker build -t mockoon-rust .
data_volume=$PWD/data
echo $data_volume
docker run --volume $data_volume:/data -p8080:8080 -it --rm mockoon-rust mockoon-rust --data /data/api-config.json