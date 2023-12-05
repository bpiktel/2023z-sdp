CREATE TABLE document_version_comments (
    comment_id UUID NOT NULL,
    user_id UUID NOT NULL,
    document_id UUID NOT NULL,
    version_id UUID NOT NULL,
    content varchar(2047) NOT NULL,
    created_at timestamp with time zone DEFAULT now(),
    PRIMARY KEY(user_id, document_id, version_id, comment_id),
    CONSTRAINT fk__document_version_comment__users FOREIGN KEY(user_id) REFERENCES users(user_id),
    CONSTRAINT fk__document_version_comment__document_versions FOREIGN KEY(document_id, version_id) REFERENCES document_versions(document_id, version_id)
);
