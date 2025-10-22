CREATE TABLE wiki_pages_history (
    id serial NOT NULL,
    wiki_page_id integer NOT NULL,
    updater_id integer NOT NULL,
    updater_ip inet NOT NULL,
    title text NOT NULL,
    body text NOT NULL,
    locked boolean NOT NULL,
    revision integer DEFAULT 1 NOT NULL,
    updated_at timestamp with time zone NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted boolean DEFAULT false NOT NULL,
    reason text,

    PRIMARY KEY (id),
    FOREIGN KEY (wiki_page_id)
        REFERENCES wiki_pages (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID,
    FOREIGN KEY (updater_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE RESTRICT
        NOT VALID
);