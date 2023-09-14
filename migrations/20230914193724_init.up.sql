CREATE TABLE links (
    id uuid PRIMARY KEY,
    url character varying(255) NOT NULL,
    hash character varying(255) NOT NULL,
    create_at timestamp(0) without time zone NOT NULL,
    updated_at timestamp(0) without time zone NOT NULL
);

CREATE UNIQUE INDEX links_id_unique_index ON links(id uuid_ops);
CREATE UNIQUE INDEX links_hash_unique_index ON links(hash text_ops);

CREATE TABLE visits (
    id uuid PRIMARY KEY,
    link_id uuid REFERENCES links(id) NOT NULL,
    create_at timestamp(0) without time zone NOT NULL
);

-- CREATE UNIQUE INDEX refunds_pkey ON refunds(id uuid_ops);
CREATE UNIQUE INDEX visits_id_unique_index ON visits(id uuid_ops);
CREATE INDEX visits_link_id_index ON visits(link_id uuid_ops);