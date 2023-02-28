-- create extension if not exists "uuid-ossp";
-- create
-- or replace function set_updated_at() returns trigger as $ $ begin NEW.updated_at = now();
-- return NEW;
-- end;
-- $ $ language plpgsql;
-- create
-- or replace function trigger_updated_at(tablename regclass) returns void as $ $ begin execute format(
--     'CREATE TRIGGER set_updated_at
--         BEFORE UPDATE
--         ON %s
--         FOR EACH ROW
--         WHEN (OLD is distinct from NEW)
--     EXECUTE FUNCTION set_updated_at();',
--     tablename
-- );
-- end;
-- $ $ language plpgsql;
-- -- Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
-- -- over things like usernames and emails, without needing to remember to do case-conversion.
-- create collation case_insensitive (
--     provider = icu,
--     locale = 'und-u-ks-level2',
--     deterministic = false
-- );
-- Add up migration script here
create table "brew" (
    brew_id uuid primary key default uuid_generate_v1mc(),
    name text collate "case_insensitive" not null,
    original_gravity integer,
    final_gravity integer,
    abv integer,
    date_start bigint,
    date_end bigint,
    brew_type integer not null,
    dry_hops boolean not null default false,
    carbonation integer,
    carbonation_ingredient integer,
    fermentation_ingredient integer,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

SELECT
    trigger_updated_at('"brew"');