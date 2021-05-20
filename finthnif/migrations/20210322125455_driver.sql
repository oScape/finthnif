create table driver (
    lastname text not null,
    firstname text not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    primary key (lastname)
);