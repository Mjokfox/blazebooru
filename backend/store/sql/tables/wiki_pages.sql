CREATE TABLE wiki_pages (
    id serial NOT NULL,
    creator_id integer NOT NULL,
    title text NOT NULL,
    body text NOT NULL,
    locked boolean DEFAULT false NOT NULL,
    created_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    revision integer DEFAULT 1 NOT NULL,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater_id integer NOT NULL,
    updater_ip inet NOT NULL,
    deleted boolean DEFAULT false NOT NULL,
    reason text,

    PRIMARY KEY (id),
    UNIQUE(title),
    FOREIGN KEY (creator_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID,
    FOREIGN KEY (updater_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID
);