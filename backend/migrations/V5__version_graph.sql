CREATE TABLE documents_dependencies (
    document_id UUID NOT NULL,
    parent_version_id UUID NOT NULL,
    child_version_id UUID NOT NULL,
    PRIMARY KEY(document_id, parent_version_id, child_version_id),
    CONSTRAINT fk__parent_document FOREIGN KEY(document_id, parent_version_id) REFERENCES document_versions(document_id, version_id),
    CONSTRAINT fk__child_document FOREIGN KEY(document_id, child_version_id) REFERENCES document_versions(document_id, version_id)
);

CREATE INDEX idx__documents_dependencies__child_version_id ON documents_dependencies (document_id, child_version_id);
