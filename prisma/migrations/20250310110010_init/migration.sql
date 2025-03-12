-- CreateTable
CREATE TABLE "user" (
    "id" BIGSERIAL NOT NULL,
    "created_at" TIMESTAMPTZ(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "username" VARCHAR,
    "first_name" VARCHAR,

    CONSTRAINT "user_pkey" PRIMARY KEY ("id")
);
