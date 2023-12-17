#!/bin/bash
curl -X POST -H "Content-Type: application/json" -d '{"message": "bar"}' http://localhost:8080/v1/messages/