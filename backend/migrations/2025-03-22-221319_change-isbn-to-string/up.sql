-- Your SQL goes here
ALTER TABLE "booklistings" DROP COLUMN "book_id";
ALTER TABLE "booklistings" ADD COLUMN "book_id" TEXT NOT NULL;
ALTER TABLE "books" DROP COLUMN "isbn";
ALTER TABLE "books" ADD COLUMN "isbn" TEXT NOT NULL;
ALTER TABLE "booklistings" DROP CONSTRAINT IF EXISTS booklistings_pkey;
ALTER TABLE "booklistings" ADD PRIMARY KEY ("user_id", "book_id");
ALTER TABLE "books" ADD PRIMARY KEY ("isbn");
