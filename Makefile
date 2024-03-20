default: help

help:
		@echo "Please use \033[32mmake \033[32m<target>\033[39m where <target> is one of"
		@echo "  \033[32m help \033[39m               Shows this help"
		@echo "  \033[32m start \033[39m              Start the setup with node server"
		@echo "  \033[32m stop \033[39m               Stops the setup"
		@echo "  \033[32m build \033[39m              Rebuilds the setup"
		@echo "  \033[32m volume_prune \033[39m       Removing all volumes and downing the project"
		@echo "  \033[32m migration_up \033[39m       Running diesel migrations"
		@echo "  \033[32m migration_redo \033[39m     Running diesel migrations down + up"
		@echo "  \033[32m readme \033[39m             Shows some help"

list:
	$(MAKE) help

start:
	docker-compose up -d

stop:
	docker-compose down

build:
	docker-compose up -d --build

volume_prune:
	docker-compose down -v

migration_up:
	diesel migration run

migration_redo:
	diesel migration redo

readme:
	@echo "\033[32m cargo run\033[39m to run the (rust) server"
	@echo "\033[32m docker-compose exec node sh\033[39m and than \033[32mnpm run dev\033[39m to start the vite web server"