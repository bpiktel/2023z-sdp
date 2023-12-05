CREATE TABLE roles (
    role_id smallint PRIMARY KEY,
    role_name varchar(255) NOT NULL UNIQUE
);

INSERT INTO roles VALUES (0, 'Admin');

CREATE TABLE user_roles (
    user_id UUID NOT NULL,
    role_id smallint NOT NULL,
    PRIMARY KEY(user_id, role_id),
    CONSTRAINT fk__user_roles__user FOREIGN KEY(user_id) REFERENCES users(user_id),
    CONSTRAINT fk__user_roles__role FOREIGN KEY(role_id) REFERENCES roles(role_id)
);

CREATE TABLE document_version_roles (
    role_id smallint PRIMARY KEY,
    role_name varchar(255) NOT NULL UNIQUE
);

INSERT INTO document_version_roles VALUES (0, 'Owner'), (1, 'Viewer'), (2, 'Editor'), (3, 'Reviewer');

CREATE TABLE user_document_version_roles (
    user_id UUID NOT NULL,
    document_id UUID NOT NULL,
    version_id UUID NOT NULL,
    role_id smallint NOT NULL,
    PRIMARY KEY(user_id, document_id, version_id, role_id),
    CONSTRAINT fk__user_document_version_roles__users FOREIGN KEY(user_id) REFERENCES users(user_id),
    CONSTRAINT fk__user_document_version_roles__document_versions FOREIGN KEY(document_id, version_id) REFERENCES document_versions(document_id, version_id),
    CONSTRAINT fk__user_document_version_roles__document_version_roles FOREIGN KEY(role_id) REFERENCES document_version_roles(role_id)
);
