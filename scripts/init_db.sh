#!/usr/bin/env bash
set -x
set -eo pipefail

# تحقق من وجود psql
if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

# تحقق من وجود sqlx
if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "  cargo install --version='~0.7' sqlx-cli \\"
  echo >&2 "    --no-default-features --features rustls,postgres"
  exit 1
fi

# إعداد المتغيرات
DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"

# اسم الحاوية
CONTAINER_NAME="postgres"

# شغل Postgres في Docker إلا إذا كان SKIP_DOCKER مفعّل
if [[ -z "${SKIP_DOCKER}" ]]
then
  if [ "$(docker ps -q -f name=$CONTAINER_NAME)" ]; then
    echo "Postgres container already running."
  else
    if [ "$(docker ps -aq -f status=exited -f name=$CONTAINER_NAME)" ]; then
      # امسح الحاوية القديمة
      docker rm $CONTAINER_NAME
    fi

    echo "Starting a new Postgres container..."
    docker run \
      --name $CONTAINER_NAME \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      -d postgres \
      postgres -N 1000
  fi
fi

# استمر في محاولة الاتصال حتى يصبح Postgres جاهز
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"

# اضبط DATABASE_URL
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}

# أنشئ قاعدة البيانات وطبّق migrations
sqlx database create
sqlx migrate run

>&2 echo "✅Postgres has been migrated, ready to go!"
