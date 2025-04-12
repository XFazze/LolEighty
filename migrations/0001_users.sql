create type tier as ENUM (
    'CHALLENGER',
    'GRANDMASTER',
    'MASTER',
    'DIAMOND',
    'EMERALD',
    'PLATINUM',
    'GOLD',
    'SILVER',
    'BRONZE',
    'IRON'
);

create type division as ENUM (
    'I',
    'II',
    'III',
    'IV'
);


create table if not exists users (
  puuid varchar(255) primary key,
  created_at timestamp default CURRENT_TIMESTAMP,
  updated_at TIMESTAMP  update CURRENT_TIMESTAMP,
  game_name varchar(255) not null,
  tag_line varchar(255) not null
);

