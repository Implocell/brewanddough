-- Add up migration script here
create table "brew_instruction" (
    instruction_id uuid primary key default uuid_generate_v1mc(),
    brew_id uuid not null,
    content text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    CONSTRAINT FK_brew FOREIGN KEY (brew_id) REFERENCES brew (brew_id) ON DELETE CASCADE
);

SELECT
    trigger_updated_at('"brew_instruction"');