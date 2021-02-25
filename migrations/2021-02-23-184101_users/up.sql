create table "user"
(
    id   serial  not null
        constraint user_pk
            primary key,
    name varchar not null,
    age  integer not null
);

create unique index user_name_uindex
    on "user" (name);

