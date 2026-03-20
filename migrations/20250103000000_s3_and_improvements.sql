-- Add image support to locations
ALTER TABLE locations ADD COLUMN IF NOT EXISTS image_url TEXT;

-- Add S3 file metadata table for tracking uploaded files
CREATE TABLE IF NOT EXISTS uploaded_files (
    id              UUID PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id),
    object_key      TEXT NOT NULL,
    filename        TEXT NOT NULL,
    content_type    TEXT NOT NULL,
    uploaded_by     UUID NOT NULL REFERENCES users(id),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_uploaded_files_tenant ON uploaded_files (tenant_id);
CREATE INDEX idx_uploaded_files_key ON uploaded_files (object_key);
