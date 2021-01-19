FROM python:3.7-alpine

RUN apk add --no-cache bash

WORKDIR /app

ENTRYPOINT ["/bin/bash"]
