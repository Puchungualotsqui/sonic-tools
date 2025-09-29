#!/bin/bash
set -e

# === CONFIG ===
USER="root"                  # VPS user
HOST="140.82.43.248"               # VPS IP or domain
DEST="/root/soundtools/"  # destination folder on VPS

IMAGE="dabid/backend:latest"
TARFILE="backend.tar.gz"

echo "[1/4] Building backend image..."
cd Backend
docker build -t $IMAGE .
cd ..

echo "[2/4] Saving image..."
docker save $IMAGE | gzip > $TARFILE

echo "[3/4] Copying to VPS..."
rsync -avz --progress $TARFILE $USER@$HOST:$DEST

echo "[4/4] Loading + restarting on VPS..."
ssh $USER@$HOST "cd $DEST && docker load < $TARFILE && docker compose up -d backend"

echo "Redeployment Successfully"
