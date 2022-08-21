-- noinspection SqlDialectInspectionForFile

-- Add migration script here
CREATE TABLE events
(
    aggregate_type text,
    aggregate_id   uuid,
    sequence       SERIAL,
    event_type     text,
    event_version  text,
    payload        json,
    metadata       json,
    timestamp       timestamptz,
    PRIMARY KEY (aggregate_type, aggregate_id, sequence)
);

CREATE TABLE snapshots
(
    aggregate_type   text,
    aggregate_id     uuid,
    last_sequence    bigint,
    current_snapshot bigint,
    payload          json,
    timestamp       timestamptz,
    PRIMARY KEY (aggregate_type, aggregate_id, last_sequence)
);
