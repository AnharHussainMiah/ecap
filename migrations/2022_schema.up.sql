create table if not exists emails (
    id serial primary key,
    email text not null,
    date_added timestamp not null
);