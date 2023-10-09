#!/bin/bash

curl https://raw.githubusercontent.com/brreg/brok/v6/docker-compose.yaml -o docker-compose.yaml
docker-compose -p brreg-brok-localhost -f docker-compose.yaml pull

# Bring up the project
docker-compose -p brreg-brok-localhost -f docker-compose.yaml up
