reset-postgres:
	docker rm -f some-postgres
	docker run --name some-postgres --rm -p 5432:5432 -e POSTGRES_HOST_AUTH_METHOD=trust -d postgres
	sleep 2
	psql postgres -h localhost -d postgres -f ./migrations/01.sql