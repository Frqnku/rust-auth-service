# 🔁 Commande par défaut
all: up

# 🔼 Démarrer les services avec Docker Compose
up:
	@echo "🔼 Démarrage des services avec Docker Compose..."
	docker-compose up --build

# 🔽 Arrêter les services
down:
	@echo "🔽 Arrêt des services..."
	docker-compose down

# ♻️ Redémarrer les services
restart:
	@echo "♻️ Redémarrage des services..."
	docker-compose down
	docker-compose up --build

# 🧪 Lancer localement Postgres + Cargo
local:
	@if ! docker ps --format '{{.Names}}' | grep -q '^pg$$'; then \
		if docker ps -a --format '{{.Names}}' | grep -q '^pg$$'; then \
			echo "🔄 Conteneur PostgreSQL trouvé mais stoppé. Démarrage..."; \
			docker start pg; \
		else \
			echo "🚀 Création et lancement du conteneur PostgreSQL..."; \
			docker run --name pg \
				-e POSTGRES_USER=postgres \
				-e POSTGRES_PASSWORD=password \
				-e POSTGRES_DB=mydb \
				-e LANG=en_US.UTF-8 \
				-e LC_ALL=en_US.UTF-8 \
				-e LC_MESSAGES=en_US.UTF-8 \
				-p 5432:5432 \
				-d postgres:17; \
		fi \
	else \
		echo "✅ Conteneur PostgreSQL déjà en cours d'exécution."; \
	fi

	@echo "⏳ Attente que PostgreSQL soit prêt..."
	@until docker exec pg pg_isready -U postgres > /dev/null 2>&1; do \
		sleep 0.5; \
	done
	@echo "🎉 PostgreSQL est prêt à l'emploi !"

	@echo "🦀 Lancement de l'application Rust..."
	cargo run

# 📜 Afficher les logs en live
logs:
	docker-compose logs -f

# 🐘 Connexion rapide à psql
psql:
	docker exec -it pg psql -U postgres -d mydb

.PHONY: all up down restart local logs psql
