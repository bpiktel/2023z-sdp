ALTER TABLE documents ADD updated_at timestamp with time zone DEFAULT now();
ALTER TABLE document_versions ADD updated_at timestamp with time zone DEFAULT now();
