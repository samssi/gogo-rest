#!/bin/bash
curl -X POST -H "Content-Type: application/json" -d "{\"message\": \"${1}\"}" http://localhost:3000/v1/messages/