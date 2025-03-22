-- This file should undo anything in `up.sql`
ALTER TABLE "booklistings" DROP COLUMN "book_id";
ALTER TABLE "booklistings" ADD COLUMN "book_id" INT8 NOT NULL;

ALTER TABLE "books" DROP COLUMN "isbn";
ALTER TABLE "books" ADD COLUMN "isbn" INT8 NOT NULL;



