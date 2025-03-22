-- Your SQL goes here



CREATE TABLE "booklistings"(
	"user_id" UUID REFERENCES users,
	"book_id" INT8 REFERENCES books,
	PRIMARY KEY("user_id", "book_id")
);

