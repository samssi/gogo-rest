FROM flyway/flyway

ENV FLYWAY_LOCATIONS=filesystem:/flyway/sql
ENV FLYWAY_CONFIG_FILES=conf/flyway.conf
ENV FLYWAY_SCHEMAS=flyway

RUN rm -rf /flyway/sql/*

COPY migrations/sql/* /flyway/sql/
COPY migrations/conf/* /flyway/conf/
