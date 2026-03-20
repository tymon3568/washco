#!/usr/bin/env bash
set -euo pipefail

PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"
PIDFILE_API="$PROJECT_DIR/.dev-api.pid"
PIDFILE_FE="$PROJECT_DIR/.dev-fe.pid"
PIDFILE_DRIVER="$PROJECT_DIR/.dev-driver.pid"
LOGFILE_API="$PROJECT_DIR/.dev-api.log"
LOGFILE_FE="$PROJECT_DIR/.dev-fe.log"
LOGFILE_DRIVER="$PROJECT_DIR/.dev-driver.log"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log()  { echo -e "${GREEN}[washco]${NC} $1"; }
warn() { echo -e "${YELLOW}[washco]${NC} $1"; }
err()  { echo -e "${RED}[washco]${NC} $1"; }

is_running() {
    [ -f "$1" ] && kill -0 "$(cat "$1")" 2>/dev/null
}

cmd_start() {
    log "Starting WashCo dev environment..."

    # 1. Podman socket
    if ! podman info &>/dev/null; then
        log "Starting podman socket..."
        systemctl --user start podman.socket
        sleep 1
    fi

    # 2. Infrastructure (postgres + keydb)
    log "Starting postgres + keydb..."
    cd "$PROJECT_DIR"
    podman compose up -d postgres keydb 2>&1 | grep -E 'Started|Running|Created' || true

    # Wait for postgres
    log "Waiting for postgres..."
    for i in $(seq 1 30); do
        if podman exec washco-postgres pg_isready -U washco &>/dev/null; then
            break
        fi
        sleep 1
    done
    podman exec washco-postgres pg_isready -U washco &>/dev/null || { err "Postgres failed to start"; exit 1; }
    log "Postgres ready"

    # 3. Build API
    if is_running "$PIDFILE_API"; then
        warn "API already running (pid $(cat "$PIDFILE_API"))"
    else
        log "Building API..."
        cargo build -p washco-server 2>&1 | tail -1
        log "Starting API on :8080..."
        cd "$PROJECT_DIR"
        ./target/debug/washco-server > "$LOGFILE_API" 2>&1 &
        echo $! > "$PIDFILE_API"
        sleep 2
        if is_running "$PIDFILE_API"; then
            log "API running (pid $(cat "$PIDFILE_API"))"
        else
            err "API failed to start. Check $LOGFILE_API"
            exit 1
        fi
    fi

    # 4. Frontend
    if is_running "$PIDFILE_FE"; then
        warn "Frontend already running (pid $(cat "$PIDFILE_FE"))"
    else
        log "Starting frontend on :5173..."
        cd "$PROJECT_DIR/frontend/owner"
        bun run dev > "$LOGFILE_FE" 2>&1 &
        echo $! > "$PIDFILE_FE"
        sleep 3
        if is_running "$PIDFILE_FE"; then
            log "Frontend running (pid $(cat "$PIDFILE_FE"))"
        else
            err "Frontend failed to start. Check $LOGFILE_FE"
            exit 1
        fi
    fi

    # 5. Driver frontend
    if is_running "$PIDFILE_DRIVER"; then
        warn "Driver app already running (pid $(cat "$PIDFILE_DRIVER"))"
    else
        log "Starting driver app on :5174..."
        cd "$PROJECT_DIR/frontend/driver"
        bun run dev > "$LOGFILE_DRIVER" 2>&1 &
        echo $! > "$PIDFILE_DRIVER"
        sleep 3
        if is_running "$PIDFILE_DRIVER"; then
            log "Driver app running (pid $(cat "$PIDFILE_DRIVER"))"
        else
            err "Driver app failed to start. Check $LOGFILE_DRIVER"
            exit 1
        fi
    fi

    echo ""
    log "All services running:"
    echo "  Postgres       localhost:5432"
    echo "  KeyDB          localhost:6379"
    echo "  API            http://localhost:8080"
    echo "  Owner App      http://localhost:5173"
    echo "  Driver App     http://localhost:5174"
    echo ""
    log "Logs: $LOGFILE_API | $LOGFILE_FE | $LOGFILE_DRIVER"
    log "Stop: ./dev.sh stop"
}

cmd_full() {
    log "Starting WashCo with APISIX gateway..."

    if ! podman info &>/dev/null; then
        log "Starting podman socket..."
        systemctl --user start podman.socket
        sleep 1
    fi

    cd "$PROJECT_DIR"

    # 1. Start infra + APISIX via dev compose
    log "Starting APISIX + infra..."
    podman compose -f compose.dev.yaml up -d 2>&1 | tail -5

    # Wait for postgres
    log "Waiting for postgres..."
    for i in $(seq 1 30); do
        if podman exec washco-postgres pg_isready -U washco &>/dev/null; then
            break
        fi
        sleep 1
    done

    # 2. Build & start API
    if is_running "$PIDFILE_API"; then
        warn "API already running"
    else
        log "Building API..."
        cargo build -p washco-server 2>&1 | tail -1
        log "Starting API on :8080..."
        ./target/debug/washco-server > "$LOGFILE_API" 2>&1 &
        echo $! > "$PIDFILE_API"
        sleep 2
        is_running "$PIDFILE_API" && log "API running" || { err "API failed"; exit 1; }
    fi

    # 3. Owner frontend
    if is_running "$PIDFILE_FE"; then
        warn "Owner app already running"
    else
        log "Starting Owner app on :5173..."
        cd "$PROJECT_DIR/frontend/owner"
        bun run dev > "$LOGFILE_FE" 2>&1 &
        echo $! > "$PIDFILE_FE"
        sleep 3
        is_running "$PIDFILE_FE" && log "Owner app running" || { err "Owner app failed"; exit 1; }
    fi

    # 4. Driver frontend
    if is_running "$PIDFILE_DRIVER"; then
        warn "Driver app already running"
    else
        log "Starting Driver app on :5174..."
        cd "$PROJECT_DIR/frontend/driver"
        bun run dev > "$LOGFILE_DRIVER" 2>&1 &
        echo $! > "$PIDFILE_DRIVER"
        sleep 3
        is_running "$PIDFILE_DRIVER" && log "Driver app running" || { err "Driver app failed"; exit 1; }
    fi

    # Wait for APISIX
    log "Waiting for APISIX..."
    for i in $(seq 1 30); do
        if curl -s -o /dev/null "http://localhost/api/v1/locations/nearby?lat=0&lng=0&radius=1" 2>/dev/null; then
            break
        fi
        sleep 2
    done

    echo ""
    log "Full stack running (via APISIX gateway):"
    echo "  APISIX Gateway   http://localhost (port 80)"
    echo "    → /api/v1/*    → API :8080"
    echo "    → /*           → Owner App :5173"
    echo "    → /driver/*    → Driver App :5174"
    echo "  API (direct)     http://localhost:8080"
    echo "  Owner (direct)   http://localhost:5173"
    echo "  Driver (direct)  http://localhost:5174"
    echo "  Postgres         localhost:5432"
    echo "  KeyDB            localhost:6379"
    echo ""
    log "Stop: ./dev.sh full-stop"
}

cmd_full_stop() {
    log "Stopping full stack..."

    # Stop local processes
    for pidfile in "$PIDFILE_DRIVER" "$PIDFILE_FE" "$PIDFILE_API"; do
        if is_running "$pidfile"; then
            kill "$(cat "$pidfile")" 2>/dev/null
            rm -f "$pidfile"
        fi
    done
    log "Local processes stopped"

    # Stop containers
    cd "$PROJECT_DIR"
    podman compose -f compose.dev.yaml down 2>&1 | tail -5
    log "All stopped"
}

cmd_stop() {
    log "Stopping WashCo dev environment..."

    # Driver frontend
    if is_running "$PIDFILE_DRIVER"; then
        kill "$(cat "$PIDFILE_DRIVER")" 2>/dev/null && log "Driver app stopped"
        rm -f "$PIDFILE_DRIVER"
    else
        rm -f "$PIDFILE_DRIVER"
    fi

    # Owner frontend
    if is_running "$PIDFILE_FE"; then
        kill "$(cat "$PIDFILE_FE")" 2>/dev/null && log "Owner app stopped"
        rm -f "$PIDFILE_FE"
    else
        rm -f "$PIDFILE_FE"
    fi

    # API
    if is_running "$PIDFILE_API"; then
        kill "$(cat "$PIDFILE_API")" 2>/dev/null && log "API stopped"
        rm -f "$PIDFILE_API"
    else
        rm -f "$PIDFILE_API"
    fi

    # Infrastructure
    cd "$PROJECT_DIR"
    log "Stopping postgres + keydb..."
    podman compose stop postgres keydb 2>&1 | grep -E 'Stopped|stopped' || true

    log "All stopped"
}

cmd_restart() {
    cmd_stop
    sleep 1
    cmd_start
}

cmd_status() {
    echo "Service        Status"
    echo "-------------- ------"

    if podman exec washco-postgres pg_isready -U washco &>/dev/null 2>&1; then
        echo -e "Postgres       ${GREEN}running${NC}"
    else
        echo -e "Postgres       ${RED}stopped${NC}"
    fi

    if podman exec washco-keydb keydb-cli ping &>/dev/null 2>&1; then
        echo -e "KeyDB          ${GREEN}running${NC}"
    else
        echo -e "KeyDB          ${RED}stopped${NC}"
    fi

    if is_running "$PIDFILE_API"; then
        echo -e "API (:8080)    ${GREEN}running${NC} (pid $(cat "$PIDFILE_API"))"
    else
        echo -e "API (:8080)    ${RED}stopped${NC}"
    fi

    if is_running "$PIDFILE_FE"; then
        echo -e "Owner App      ${GREEN}running${NC} (pid $(cat "$PIDFILE_FE"))"
    else
        echo -e "Owner App      ${RED}stopped${NC}"
    fi

    if is_running "$PIDFILE_DRIVER"; then
        echo -e "Driver App     ${GREEN}running${NC} (pid $(cat "$PIDFILE_DRIVER"))"
    else
        echo -e "Driver App     ${RED}stopped${NC}"
    fi
}

cmd_logs() {
    local svc="${1:-api}"
    case "$svc" in
        api)         tail -f "$LOGFILE_API" ;;
        fe|owner)    tail -f "$LOGFILE_FE" ;;
        driver)      tail -f "$LOGFILE_DRIVER" ;;
        pg|postgres) podman logs -f washco-postgres ;;
        apisix)      podman logs -f washco-apisix ;;
        *)           err "Unknown: $svc (api|owner|driver|pg|apisix)" ;;
    esac
}

cmd_db() {
    local sub="${1:-shell}"
    case "$sub" in
        shell) PGPASSWORD=washco /usr/bin/psql -h localhost -U washco -d washco ;;
        reset)
            warn "Dropping and recreating database..."
            PGPASSWORD=washco /usr/bin/psql -h localhost -U washco -d postgres \
                -c "DROP DATABASE IF EXISTS washco;" \
                -c "CREATE DATABASE washco;"
            log "Database reset. Restart API to re-apply migrations: ./dev.sh restart"
            ;;
        *) err "Unknown: $sub (shell|reset)" ;;
    esac
}

case "${1:-help}" in
    start)      cmd_start ;;
    stop)       cmd_stop ;;
    restart)    cmd_restart ;;
    status)     cmd_status ;;
    logs)       cmd_logs "${2:-api}" ;;
    db)         cmd_db "${2:-shell}" ;;
    full)       cmd_full ;;
    full-stop)  cmd_full_stop ;;
    help|*)
        echo "Usage: ./dev.sh <command>"
        echo ""
        echo "Commands (dev mode - local processes):"
        echo "  start     Start all services (postgres, keydb, api, owner app, driver app)"
        echo "  stop      Stop all services"
        echo "  restart   Stop then start"
        echo "  status    Show service status"
        echo "  logs      Tail logs (api|owner|driver|pg|apisix)"
        echo "  db        Database (shell|reset)"
        echo ""
        echo "Commands (full stack - APISIX gateway):"
        echo "  full       Start entire stack via podman compose (APISIX + all services)"
        echo "  full-stop  Stop entire stack"
        ;;
esac
