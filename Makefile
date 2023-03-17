test:
	docker run --rm -v $$PWD:/brok foundry "forge test --root /brok/contracts"

build:
	docker run --rm -v $$PWD:/brok foundry "forge build --root /brok/contracts"

install:
	docker run --rm -v $$PWD:/brok foundry "forge install --root /brok/contracts"