create table users
(
    user_id smallint primary key generated by default as identity,
    email   text not null
)
