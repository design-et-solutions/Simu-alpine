export PGDATA=$PWD/.pgdata
export PGPORT=5432
export PGDATABASE=pictureperfect
export PGUSER=dev
export PGHOST=localhost

if [ ! -d "$PGDATA" ]; then
  echo "🔧 Initializing local PostgreSQL database..."
  initdb -U $PGUSER
fi

if [ ! -d "$PGDATA/run" ]; then
  mkdir -p "$PGDATA/run"
fi

echo "🚀 Starting PostgreSQL on port $PGPORT..."
pg_ctl -D "$PGDATA" -o "-p $PGPORT -F -k $PGDATA/run" -l "$PGDATA/logfile" start

until pg_isready -q -h "$PGHOST" -p $PGPORT; do
  sleep 0.2
done

if ! psql -tAc "SELECT 1 FROM pg_database WHERE datname = '$PGDATABASE'" | grep -q 1; then
  echo "📦 Creating database '$PGDATABASE'..."
  createdb $PGDATABASE
fi

echo "✅ PostgreSQL is running."
echo "🔸 Press Ctrl+C to stop."

trap "pg_ctl -D '$PGDATA' stop" EXIT

while true; do sleep 1; done
