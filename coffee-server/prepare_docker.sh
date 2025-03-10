#!/bin/sh

cd ../frontend/
npm run build
cd ../coffee-server/
cp -r ../frontend/dist/ ./routes
