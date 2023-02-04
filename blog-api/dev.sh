export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/blog
docker run -p 5432:5432 --rm -e POSTGRES_PASSWORD=password -e POSTGRES_DB=blog postgres:12