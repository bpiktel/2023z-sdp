CREATE TABLE document_sets (
    document_set_id UUID PRIMARY KEY,
    document_set_name varchar(255) NOT NULL UNIQUE
);

CREATE TABLE document_set_versions (
    document_set_id UUID NOT NULL,
    set_version_id UUID NOT NULL,
    set_version_name varchar(255) NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    PRIMARY KEY(document_set_id, set_version_id),
    UNIQUE(document_set_id, set_version_name),
    CONSTRAINT fk__document_set_versions__document_sets FOREIGN KEY(document_set_id) REFERENCES document_sets(document_set_id)
);

CREATE TABLE document_set_versions_elements (
    document_set_id UUID NOT NULL,
    set_version_id UUID NOT NULL,
    document_id UUID NOT NULL,
    version_id UUID NOT NULL,
    PRIMARY KEY(document_set_id, set_version_id, document_id, version_id),
    UNIQUE(document_set_id, set_version_id, document_id),
    CONSTRAINT fk__document_set_versions_elements__document_set_versions FOREIGN KEY(document_set_id, set_version_id) REFERENCES document_set_versions(document_set_id, set_version_id),
    CONSTRAINT fk__document_set_versions_elements__document_versions FOREIGN KEY(document_id, version_id) REFERENCES document_versions(document_id, version_id)
);

CREATE TABLE document_sets_dependencies (
    document_set_id UUID NOT NULL,
    parent_version_id UUID NOT NULL,
    child_version_id UUID NOT NULL,
    PRIMARY KEY(document_set_id, parent_version_id, child_version_id),
    CONSTRAINT fk__parent_document FOREIGN KEY(document_set_id, parent_version_id) REFERENCES document_set_versions(document_set_id, set_version_id),
    CONSTRAINT fk__child_document FOREIGN KEY(document_set_id, child_version_id) REFERENCES document_set_versions(document_set_id, set_version_id)
);

CREATE INDEX idx__document_sets_dependencies__child_version_id ON document_sets_dependencies (document_set_id, child_version_id);
