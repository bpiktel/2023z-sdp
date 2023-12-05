CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    salt UUID NOT NULL,
    username varchar(255) UNIQUE NOT NULL,
    password_hash char(64) NOT NULL
);

CREATE TABLE documents (
    document_id UUID PRIMARY KEY,
    document_name varchar(255) NOT NULL UNIQUE
);

CREATE TABLE document_versions (
    document_id UUID NOT NULL,
    version_id UUID NOT NULL,
    version_name varchar(255) NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    content varchar(2047) DEFAULT '',
    PRIMARY KEY(document_id, version_id),
    UNIQUE(document_id, version_name),
    CONSTRAINT fk__document_versions__documents FOREIGN KEY(document_id) REFERENCES documents(document_id)
);
