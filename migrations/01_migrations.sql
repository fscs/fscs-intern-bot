CREATE TABLE IF NOT EXISTS users (
    discord_id INTEGER NOT NULL, 
    name VARCHAR(250) NOT NULL
);

CREATE TABLE IF NOT EXISTS antragsthreads (
    antrag_id TEXT  NOT NULL, 
    thread_id INTEGER NOT NULL
);
