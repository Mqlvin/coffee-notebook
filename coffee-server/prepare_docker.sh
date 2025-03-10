#!/bin/sh

cd ../frontend/
npm i
npm run build
cd ../coffee-server/
cp -r ../frontend/dist/ ./routes
