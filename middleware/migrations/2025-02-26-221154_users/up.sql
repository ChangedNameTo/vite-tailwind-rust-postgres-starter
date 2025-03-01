create table
  "public"."users" (
    "id" serial not null,
    "name" varchar(255) not null,
    "address" varchar(255),
    "city" varchar(255),
    "state" varchar(255),
    "zip" varchar(255),
    "email" varchar(255) not null,
    "phone" varchar(255),
    "photo" varchar(255),
    "verified" BOOLEAN default false not null,
    "provider" varchar(255) not null,
    "provider_id" varchar(255),
    "created_at" timestamp not null default NOW(),
    "updated_at" timestamp not null default NOW(),
    constraint "user_pkey" primary key ("id")
  )