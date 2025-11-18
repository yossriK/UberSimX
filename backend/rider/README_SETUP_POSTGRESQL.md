# Database Setup Guide (Postgres + SQLx)

This project uses **Postgres** with **SQLx migrations** for database schema management.
Follow these steps to set up your database locally.

---

## 1. Install Postgres

If you don’t already have Postgres installed:

```bash
sudo apt update
sudo apt install postgresql postgresql-contrib
```

Start Postgres:

```bash
sudo service postgresql start
```

---

## 2. Create a `.env` file

In the project root (`driver_project/`), create a file named **`settings.env`** with the following contents:

```env
DATABASE_URL=postgresql://<username>@localhost:5432/ubersim_driver
```

* Replace `<username>` with your Linux user (e.g., `ubuntu` if that’s what you’re using).
* `ubersim_driver` is the database name we use for this service.

---

## 3. Init DB Script

We use a helper script to make sure the database exists.
The script is located at: **`driver_project/sbin/init_db.sh`**

Make it executable:

```bash
chmod +x sbin/init_db.sh
```

---

## 4. Add Initial Migration

If not already created, you can initialize migrations with:

```bash
cargo sqlx migrate add init_schema

## then you can create separate migration files via
 sqlx migrate add create_riders_table
```

This creates a new SQL file under **`migrations/`**.
Example schema for the `riders` table:

```sql
-- migrations/20230830120000_init_schema.sql
CREATE TABLE riders (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
   created_at      TIMESTAMPTZ DEFAULT NOW()
);
```

---

## 5. Run Setup

From the project root, run:

```bash
./sbin/init_db.sh
```

This will:

* Load `.env`
* Create the database if missing
* Run all migrations

---

## 6. Verify

You can connect to the database manually:

```bash
psql -d ubersim_rider
```

Check that your table exists:

```sql
\d riders;
```

# Creating dedicated Postgres user for local dev



## 1️⃣ Check if the Postgres service is running

```bash
sudo systemctl status postgresql
```

If it’s not running:

```bash
sudo systemctl start postgresql
```

---

## 2️⃣ Connect using `psql` locally

Try:

```bash
sudo -u postgres psql
```

* This logs you in as the default Postgres `postgres` user.
* If this works, we know Postgres itself is fine.

---

## 3️⃣ Create a user for your project

Instead of relying on your Ubuntu username, create a dedicated Postgres user:

```sql
-- inside psql
CREATE USER your_user WITH PASSWORD 'your pass';
CREATE DATABASE your_db OWNER your_user;
GRANT ALL PRIVILEGES ON DATABASE your_db TO your_user;
\q
```
