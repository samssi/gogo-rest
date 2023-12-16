FROM golang:1.21-alpine

WORKDIR /app

COPY gogo-rest-app ./
RUN go mod download

RUN go build -o /gogo-rest-app

EXPOSE 8080

CMD [ "/gogo-rest-app" ]