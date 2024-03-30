CREATE OR REPLACE FUNCTION update_modified_users()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.modification_date = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_modified_at
    BEFORE UPDATE ON users
    FOR EACH ROW
EXECUTE FUNCTION update_modified_users();