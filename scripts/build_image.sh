#!/bin/bash
CLIENT_IMAGE=${CLIENT_IMAGE:-simple-portal}
docker build . -t $CLIENT_IMAGE
