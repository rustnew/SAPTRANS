ALTER TABLE admin_users
    ADD COLUMN IF NOT EXISTS display_name VARCHAR(120) NOT NULL DEFAULT '',
    ADD COLUMN IF NOT EXISTS role VARCHAR(24) NOT NULL DEFAULT 'viewer',
    ADD COLUMN IF NOT EXISTS center VARCHAR(40),
    ADD COLUMN IF NOT EXISTS active BOOLEAN NOT NULL DEFAULT TRUE;

ALTER TABLE admin_users
    ADD CONSTRAINT admin_users_role_check
    CHECK (role IN ('admin', 'commercial', 'transport', 'equipment', 'inspection', 'finance', 'viewer'));

CREATE TABLE work_items (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source_contact_id BIGINT UNIQUE REFERENCES contact_messages(id) ON DELETE SET NULL,
    kind VARCHAR(24) NOT NULL CHECK (kind IN ('transport', 'engins', 'autorelax', 'immobilier')),
    status VARCHAR(24) NOT NULL DEFAULT 'nouveau'
        CHECK (status IN ('nouveau', 'qualifie', 'planifie', 'en_cours', 'en_revue', 'termine', 'annule')),
    title VARCHAR(180) NOT NULL,
    customer_name VARCHAR(120) NOT NULL,
    customer_email VARCHAR(320),
    customer_phone VARCHAR(32),
    description TEXT NOT NULL DEFAULT '',
    details JSONB NOT NULL DEFAULT '{}'::jsonb,
    center VARCHAR(40),
    assigned_to BIGINT REFERENCES admin_users(id) ON DELETE SET NULL,
    created_by BIGINT REFERENCES admin_users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    closed_at TIMESTAMPTZ
);

CREATE INDEX work_items_kind_status_idx ON work_items(kind, status, created_at DESC);
CREATE INDEX work_items_assigned_idx ON work_items(assigned_to, status, created_at DESC);
CREATE INDEX work_items_center_idx ON work_items(center, kind, created_at DESC);

CREATE TABLE work_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    work_item_id BIGINT NOT NULL REFERENCES work_items(id) ON DELETE CASCADE,
    actor_id BIGINT REFERENCES admin_users(id) ON DELETE SET NULL,
    action VARCHAR(32) NOT NULL,
    note TEXT NOT NULL DEFAULT '',
    old_status VARCHAR(24),
    new_status VARCHAR(24),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX work_events_item_idx ON work_events(work_item_id, recorded_at, id);

INSERT INTO work_items (source_contact_id, kind, title, customer_name, customer_email, description, details, created_at, updated_at)
SELECT id, service, 'Demande ' || service || ' SAP-' || lpad(id::text, 6, '0'), name, email, message,
       jsonb_build_object('contact_details', details), created_at, created_at
FROM contact_messages
WHERE service IN ('transport', 'engins', 'autorelax', 'immobilier')
ON CONFLICT (source_contact_id) DO NOTHING;

INSERT INTO work_events (work_item_id, action, note, new_status, recorded_at)
SELECT id, 'demande_recue', 'Demande publique importée', 'nouveau', created_at
FROM work_items WHERE source_contact_id IS NOT NULL;
