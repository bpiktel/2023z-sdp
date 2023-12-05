CREATE TABLE files (
    file_id UUID PRIMARY KEY,
    file_name varchar(255) NOT NULL,
    file_mime_type varchar(255) NOT NULL,
    file_hash char(64) NOT NULL
);

CREATE TABLE file_attachments (
    document_id UUID NOT NULL,
    version_id UUID NOT NULL,
    file_id UUID NOT NULL,
    PRIMARY KEY(document_id, version_id, file_id),
    CONSTRAINT fk__file_attachments__document_versions FOREIGN KEY(document_id, version_id) REFERENCES document_versions(document_id, version_id),
    CONSTRAINT fk__file_attachments__files FOREIGN KEY(file_id) REFERENCES files(file_id)
);
