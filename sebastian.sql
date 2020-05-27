create table users(
   id text primary key,
   name text not null
);

create table totals(
   user_id text,
   word text,
   total_count integer

   foreign key (user_id) references users(id)
)


create table words(
   user_id text,
   before_word text,
   after_word text,
   count integer,

   FOREIGN key (user_id) REFERENCES users(id)
);
