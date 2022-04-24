# Setup Cleaner

## Installing

1. Install pg_cron: [Instructions](https://github.com/citusdata/pg_cron)
2. Create function:
```sql
CREATE OR REPLACE FUNCTION cron.cross_schedule(job_name name, schedule text, command text, database_name text) RETURNS bigint
AS
$$
DECLARE
    job_id bigint;
BEGIN
    SELECT cron.schedule(job_name, schedule, command) INTO job_id;
    UPDATE cron.job SET database = database_name WHERE jobid = job_id;
    RETURN job_id;
END;
$$ LANGUAGE plpgsql;
```
3. Schedule:
```sql
SELECT cron.cross_schedule('clean_account_db', '*/5 * * * *', 'SELECT clean_account_db();', 'account');
```

<br>

## Removing
1. Unschedule:
```sql
SELECT cron.unschedule('clean_account_db');
```
2. Remove function:
```sql
DROP FUNCTION cron.cross_schedule(job_name name, schedule text, command text, database_name text);
```