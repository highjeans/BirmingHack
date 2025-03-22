-- Your SQL goes here

CREATE TABLE "books"(
	"isbn" BIGINT NOT NULL PRIMARY KEY,
	"title" TEXT NOT NULL,
	"author" TEXT NOT NULL,
	"embeddings" TEXT NOT NULL,
	"user_id" UUID NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

CREATE TABLE "socials"(
	"id" UUID NOT NULL PRIMARY KEY,
	"platform" TEXT NOT NULL,
	"username" TEXT NOT NULL,
	"user_id" UUID NOT NULL,
	FOREIGN KEY ("user_id") REFERENCES "users"("id")
);

