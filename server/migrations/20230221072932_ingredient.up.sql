-- Add up migration script here
create table "ingredient" (
    ingredient_id uuid primary key default uuid_generate_v1mc(),
    name text collate "case_insensitive" not null,
    unit text not null,
    ingredient_type text not null
);

SELECT
    trigger_updated_at('"ingredient"');