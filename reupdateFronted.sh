#!/bin/bash
set -e

# === CONFIG ===
USER="root"                  # VPS user
HOST="140.82.43.248"               # VPS IP or domain
DEST="/root/soundtools/"  # destination folder on VPS

IMAGE="dabid/frontend:latest"
TARFILE="frontend.tar.gz"

echo "[1/4] Building frontend image..."
cd Frontend
docker build -t $IMAGE .
cd ..

echo "[2/4] Saving image..."
docker save $IMAGE | gzip > $TARFILE

echo "[3/4] Copying to VPS..."
rsync -avz --progress $TARFILE $USER@$HOST:$DEST

echo "[4/4] Loading + restarting on VPS..."
ssh $USER@$HOST "cd $DEST && docker load < $TARFILE && docker compose up -d frontend"

echo "Redeployment Successfully"
