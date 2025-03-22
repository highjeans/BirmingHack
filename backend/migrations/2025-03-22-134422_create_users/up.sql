-- Your SQL goes here
CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"username" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"fullname" TEXT NOT NULL
);

