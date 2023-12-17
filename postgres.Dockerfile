FROM postgres:16.1-alpine3.19

COPY ./migrations/sql/*.sql /docker-entrypoint-initdb.d/

ENV POSTGRES_DB=gogo
ENV POSTGRES_USER=gogo
ENV POSTGRES_PASSWORD=gogo

EXPOSE 5432