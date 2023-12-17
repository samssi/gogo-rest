SET SESSION search_path TO 'gogo';

create table public.message (
    message_id SERIAL PRIMARY KEY,
    message TEXT NOT NULL
);
