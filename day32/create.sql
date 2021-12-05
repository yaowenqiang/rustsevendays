create table users (
    id serial primary key,
    username varchar(255) not null,
    avatar varchar(255) not null
);

insert into users (username) values ("zbyszek");

create table photos (
    id serial primary key,
    user_id integer not null references users (id),
    url varchar(255) not null
);

insert into photos (user_id, url) values (1, "http://lorempixel.com/output/cats-q-c-640-480-10.jpg");

