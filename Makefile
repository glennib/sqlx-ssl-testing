.PHONY: help up down init-db init-ca

help: ## Display this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

init-db: up ## Initialize database
	sqlx database reset -y

init-ca: ca/server.key

ca/server.key:
	./init-ca.sh

up: ## bring up database
	docker compose up -d

down:  ## bring down database
	docker compose down


