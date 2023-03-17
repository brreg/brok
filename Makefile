docker:
	docker pull ghcr.io/foundry-rs/foundry:latest
	docker tag ghcr.io/foundry-rs/foundry:latest foundry:latest

test:
	docker run --rm -v $$PWD:/brok foundry "forge test --root /brok/contracts"

build:
	docker run --rm -v $$PWD:/brok foundry "forge build --root /brok/contracts"

install:
	docker run --rm -v $$PWD:/brok foundry "forge install --root /brok/contracts"

help:
	docker run --rm -v $$PWD:/brok foundry "forge -h"