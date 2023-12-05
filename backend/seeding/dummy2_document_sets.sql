/* Document sets */
INSERT INTO document_sets (document_set_id, document_set_name) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', 'alpha document set'),
    ('65a452ca-f418-11ed-a05b-0242ac120003', 'beta document set');

/* Initial set versions */
INSERT INTO document_set_versions (document_set_id, set_version_id, set_version_name, created_at) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '1', now()),
    ('65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '1', now());

/* Version set elements */
INSERT INTO document_set_versions_elements (document_set_id, set_version_id, document_id, version_id) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003'),
    ('65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003');

/* Version sets of alpha */
INSERT INTO document_set_versions (document_set_id, set_version_id, set_version_name, created_at) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', '2', now()),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', '3', now()),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003', '4', now());

/* Version set elements */
INSERT INTO document_set_versions_elements (document_set_id, set_version_id, document_id, version_id) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003', '65a452ca-f418-11ed-a05b-0242ac120003');

/* Version set dependencies */
INSERT INTO document_sets_dependencies (document_set_id, parent_version_id, child_version_id) VALUES
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e4be-f419-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003'),
    ('65a45040-f418-11ed-a05b-0242ac120003', '88c2e78e-f419-11ed-a05b-0242ac120003', '88c2e8d8-f419-11ed-a05b-0242ac120003');
