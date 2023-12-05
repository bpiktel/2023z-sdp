CREATE TABLE document_version_states (
    state_id smallint PRIMARY KEY,
    state_name varchar(255) NOT NULL UNIQUE
);

INSERT INTO document_version_states VALUES (0, 'InProgress'), (1, 'ReadyForReview'), (2, 'Reviewed'), (3, 'Published');

ALTER TABLE document_versions ADD version_state smallint DEFAULT 0;
ALTER TABLE document_versions ADD CONSTRAINT fk__version_state FOREIGN KEY (version_state) REFERENCES document_version_states (state_id);
