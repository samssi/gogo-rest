#!/bin/bash
curl -X GET -w "HTTP status: %{http_code}\n" -H "Content-Type: application/json" http://localhost:3000/v1/messages/