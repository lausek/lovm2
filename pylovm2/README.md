# pylovm2

## Building

**NOTE:** manylinux wheels are required for distribution

``` bash
sudo docker build -t pylovm2-build .
sudo docker run -it -v $(pwd):/io pylovm2-build
$ ./build-wheels
```

copy wheels out of container:

``` bash
sudo docker ps
sudo docker cp <container_id>:/app/dist/ ./dist
```
