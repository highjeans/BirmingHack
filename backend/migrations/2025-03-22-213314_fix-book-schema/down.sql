-- This file should undo anything in `up.sql`

ALTER TABLE "books" ADD COLUMN "user_id" UUID NOT NULL;



