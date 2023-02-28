-- Add up migration script here
-- Link between brew and ingredient should also have measurement column
create table "brew_ingredient" (
    brew_id uuid,
    ingredient_id uuid,
    -- divide by 100 to get float
    amount integer,
    CONSTRAINT brew_ing_pk PRIMARY KEY (brew_id, ingredient_id),
    CONSTRAINT FK_brew FOREIGN KEY (brew_id) REFERENCES brew (brew_id) ON DELETE CASCADE,
    CONSTRAINT FK_ingredient FOREIGN KEY (ingredient_id) REFERENCES ingredient (ingredient_id) ON DELETE CASCADE
);

SELECT
    trigger_updated_at('"brew_ingredient"');