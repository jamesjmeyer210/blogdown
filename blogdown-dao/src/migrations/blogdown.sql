create table if not exists groups
(
    id integer primary key autoincrement not null,
    name text not null,
    created_on_utc text not null
);

create table if not exists users
(
    id integer primary key autoincrement not null,
    name text not null,
    email text not null,
    password blob not null,
    created_on_utc text not null,
    updated_on_utc text not null
);

create table if not exists users_to_groups
(
    user_id integer,
    group_id integer,
    foreign key (user_id) references users(id),
    foreign key (group_id) references groups(id)
    );

create table if not exists blogs
(
    id integer primary key autoincrement not null,
    title text not null,
    content text not null
);

create table if not exists users_to_blogs
(
    user_id integer,
    blog_id integer,
    foreign key (user_id) references users(id),
    foreign key (blog_id) references blogs(id)
    );

create table if not exists comments
(
    id integer primary key autoincrement not null,
    content text not null,
    reply_to_id integer,
    blog_id integer not null,
    user_id integer not null,
    foreign key (reply_to_id) references comments(id),
    foreign key (blog_id) references blogs(id),
    foreign key (user_id) references users(id)
);