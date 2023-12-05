/* Initial users */
INSERT INTO users (user_id, salt, username, password_hash) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', 'q', 'ca5dcb79740c085bf5426b8cd1ef53ddd1380e4d7a1bc722143fddab8ff8a18f'),
    ('65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', 'w', '2dc87ed5a5b711d5402222645e0d05d43c7fa935653b5bc557bfcb592100874f'),
    ('65a45428-f418-11ed-a05b-0242ac120003', '65a45428-f418-11ed-a05b-0242ac120003', 'e', 'cc7acefb11f4f277094f37c06b9625eae71002a4883365f3ea06f59d8c27ec8a'),
    ('65a4581a-f418-11ed-a05b-0242ac120003', '65a4581a-f418-11ed-a05b-0242ac120003', 'r', 'c660adae63b394836173d804dc6ff32b30cab6ddd8167d49a5c590a973bf9cdb');

/* Documents */
INSERT INTO documents (document_id, document_name, updated_at) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', 'alpha document', now()),
    ('65a452ca-f418-11ed-a05b-0242ac120003', 'beta document', now()),
    ('65a45428-f418-11ed-a05b-0242ac120003', 'gamma document', now()),
    ('65a4581a-f418-11ed-a05b-0242ac120003', 'delta document', now());

/* Initial versions */
INSERT INTO document_versions (document_id, version_id, version_name, created_at, content, updated_at, version_state) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '1', now(), 'initial alpha', now(), 0),
    ('65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '1', now(), 'initial beta', now(), 1),
    ('65a45428-f418-11ed-a05b-0242ac120003', '65a45428-f418-11ed-a05b-0242ac120003', '1', now(), 'initial gamma', now(), 2),
    ('65a4581a-f418-11ed-a05b-0242ac120003', '65a4581a-f418-11ed-a05b-0242ac120003', '1', now(), 'initial delta', now(), 3);

/* Owners */
INSERT INTO user_document_version_roles (user_id, document_id, version_id, role_id) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', 0),
    ('65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', 0),
    ('65a45428-f418-11ed-a05b-0242ac120003', '65a45428-f418-11ed-a05b-0242ac120003', '65a45428-f418-11ed-a05b-0242ac120003', 0),
    ('65a4581a-f418-11ed-a05b-0242ac120003', '65a4581a-f418-11ed-a05b-0242ac120003', '65a4581a-f418-11ed-a05b-0242ac120003', 0);

/* Versions of alpha */
INSERT INTO document_versions (document_id, version_id, version_name, created_at, content, updated_at, version_state) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', '2', now(), 'alpha 1', now(), 0),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', '3', now(), 'alpha 2', now(), 1),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003', '4', now(), 'alpha 3', now(), 2);

/* Alpha version roles */
INSERT INTO user_document_version_roles (user_id, document_id, version_id, role_id) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', 0),
    ('65a452ca-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', 2),
    ('65a4581a-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', 3),
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', 1),
    ('65a452ca-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', 0),
    ('65a4581a-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', 3),
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003', 2),
    ('65a45428-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003', 0),
    ('65a4581a-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003', 3);

/* Version dependencies */
INSERT INTO documents_dependencies (document_id, parent_version_id, child_version_id) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003');
