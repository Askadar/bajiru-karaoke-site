-- Users
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(256) NOT NULL UNIQUE,
    twitch_id BIGINT UNIQUE,
    discord_id BIGINT UNIQUE
);

-- Capabilities
CREATE TABLE capabilities (
    id SERIAL PRIMARY KEY,
    title VARCHAR(64) NOT NULL UNIQUE
);

-- User <-> Capability (M2M)
CREATE TABLE user_capabilities (
    user_id INT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    capability_id INT NOT NULL REFERENCES capabilities (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, capability_id)
);

-- Genres
CREATE TABLE genres (
    id SERIAL PRIMARY KEY,
    title VARCHAR(64) NOT NULL UNIQUE
);

-- Tags
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL UNIQUE
);

-- Artists
CREATE TABLE artists (
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) NOT NULL,
    description VARCHAR(2048)
);

-- Cover art
CREATE TABLE cover_art (
    id SERIAL PRIMARY KEY,
    file_path VARCHAR(512) NOT NULL,
    thumbnail VARCHAR(512),
    -- Credit URL or names for the artwork.
    credits VARCHAR(512)
);

-- Audio files
CREATE TABLE audio (
    id SERIAL PRIMARY KEY,
    file_path VARCHAR(512) NOT NULL
);

-- Video links
CREATE TABLE video (
    id SERIAL PRIMARY KEY,
    external_url VARCHAR(512) NOT NULL
);

-- Songs
CREATE TABLE songs (
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) NOT NULL,
    art_id INT REFERENCES cover_art (id) ON DELETE SET NULL,
    audio_id INT UNIQUE REFERENCES audio (id) ON DELETE SET NULL,
    video_id INT REFERENCES video (id) ON DELETE SET NULL,
    uploader_id INT REFERENCES users (id) ON DELETE SET NULL,
    lyrics TEXT,
    stream_date TIMESTAMPTZ,
    play_count INT NOT NULL DEFAULT 0,
    -- Duration in milliseconds.
    duration INT NOT NULL DEFAULT 0,
    date_added TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Admin-visible note shown on the queue page.
    memo VARCHAR(1024)
);

-- Song <-> Singer (M2M)
CREATE TABLE song_singers (
    song_id INT NOT NULL REFERENCES songs (id) ON DELETE CASCADE,
    artist_id INT NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
    PRIMARY KEY (song_id, artist_id)
);

-- Song <-> OriginalArtist (M2M)
CREATE TABLE song_original_artists (
    song_id INT NOT NULL REFERENCES songs (id) ON DELETE CASCADE,
    artist_id INT NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
    PRIMARY KEY (song_id, artist_id)
);

-- Song <-> Genre (M2M)
CREATE TABLE song_genres (
    song_id INT NOT NULL REFERENCES songs (id) ON DELETE CASCADE,
    genre_id INT NOT NULL REFERENCES genres (id) ON DELETE CASCADE,
    PRIMARY KEY (song_id, genre_id)
);

-- Song <-> Tag (M2M)
CREATE TABLE song_tags (
    song_id INT NOT NULL REFERENCES songs (id) ON DELETE CASCADE,
    tag_id INT NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    PRIMARY KEY (song_id, tag_id)
);

-- Playlists
CREATE TABLE playlists (
    id SERIAL PRIMARY KEY,
    title VARCHAR(256) NOT NULL,
    description VARCHAR(2048),
    created_by INT REFERENCES users (id) ON DELETE SET NULL
);

-- Playlist <-> Song (M2M, ordered)
CREATE TABLE playlist_songs (
    playlist_id INT NOT NULL REFERENCES playlists (id) ON DELETE CASCADE,
    song_id INT NOT NULL REFERENCES songs (id) ON DELETE CASCADE,
    -- Zero-based display order within the playlist.
    sort_order INT NOT NULL DEFAULT 0,
    PRIMARY KEY (playlist_id, song_id)
);

-- User <-> FavoriteSong (M2M)
CREATE TABLE user_favorite_songs (
    user_id INT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    song_id INT NOT NULL REFERENCES songs (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, song_id)
);

-- Indexes
CREATE INDEX CONCURRENTLY idx_songs_date_added ON songs (date_added);
CREATE INDEX CONCURRENTLY idx_artists_title ON artists (title);
