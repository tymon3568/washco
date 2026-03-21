#!/bin/bash
# WashCo Database Backup Script
# Usage: ./infra/scripts/backup-db.sh [backup_dir]
# Cron example: 0 2 * * * /path/to/washco/infra/scripts/backup-db.sh /backups/washco

set -euo pipefail

BACKUP_DIR="${1:-./backups}"
CONTAINER_NAME="washco-postgres"
DB_NAME="washco"
DB_USER="washco"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="${BACKUP_DIR}/washco_${TIMESTAMP}.sql.gz"
RETENTION_DAYS=30

mkdir -p "$BACKUP_DIR"

echo "[$(date)] Starting database backup..."

podman exec "$CONTAINER_NAME" pg_dump -U "$DB_USER" -d "$DB_NAME" --format=plain --no-owner | gzip > "$BACKUP_FILE"

FILESIZE=$(du -h "$BACKUP_FILE" | cut -f1)
echo "[$(date)] Backup completed: $BACKUP_FILE ($FILESIZE)"

# Clean old backups
DELETED=$(find "$BACKUP_DIR" -name "washco_*.sql.gz" -mtime +"$RETENTION_DAYS" -delete -print | wc -l)
if [ "$DELETED" -gt 0 ]; then
    echo "[$(date)] Cleaned $DELETED backups older than $RETENTION_DAYS days"
fi

echo "[$(date)] Backup finished successfully"
