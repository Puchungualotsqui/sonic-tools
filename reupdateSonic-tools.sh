#!/bin/bash
set -e

# === CONFIG ===
USER="root"                  # VPS user
HOST="140.82.43.248"               # VPS IP or domain
DEST="/root/soundtools/"  # destination folder on VPS

IMAGE="dabid/rust-audio:latest"
TARFILE="rust-audio.tar.gz"
DOCKERFILE="Services/rust-audio/Dockerfile"

echo "[1/4] Building rust-audio image..."
docker build -t $IMAGE -f $DOCKERFILE .
cd ..

echo "[2/4] Saving image..."
docker save $IMAGE | gzip > $TARFILE

echo "[3/4] Copying to VPS..."
rsync -av --progress -e "ssh -c aes128-ctr -o Compression=no" $TARFILE $USER@$HOST:$DEST

echo "[4/4] Loading + restarting on VPS..."
ssh $USER@$HOST "cd $DEST && docker load < $TARFILE && docker compose up -d rust-audio"

echo "Redeployment Successfully"
