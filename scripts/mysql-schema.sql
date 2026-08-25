-- MySQL 8 schema for personal_ai_workbench.
-- Safe to run repeatedly. Does not migrate or delete existing data.
CREATE DATABASE IF NOT EXISTS personal_ai_workbench
  CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci;
USE personal_ai_workbench;

CREATE TABLE IF NOT EXISTS cards (
  id VARCHAR(128) PRIMARY KEY,
  title VARCHAR(512) NOT NULL,
  summary TEXT NOT NULL,
  card_type VARCHAR(64) NOT NULL,
  tags JSON NOT NULL,
  source TEXT NULL,
  source_path TEXT NULL,
  visibility VARCHAR(64) NOT NULL,
  status VARCHAR(32) NOT NULL,
  favorite BOOLEAN NOT NULL DEFAULT FALSE,
  created_at DATETIME(6) NOT NULL,
  updated_at DATETIME(6) NOT NULL,
  deleted_at DATETIME(6) NULL,
  INDEX idx_cards_type (card_type),
  INDEX idx_cards_status (status),
  INDEX idx_cards_updated (updated_at),
  INDEX idx_cards_deleted (deleted_at)
);

CREATE TABLE IF NOT EXISTS card_contents (
  card_id VARCHAR(128) PRIMARY KEY,
  content LONGTEXT NOT NULL,
  content_sha256 CHAR(64) NOT NULL,
  updated_at DATETIME(6) NOT NULL,
  CONSTRAINT fk_contents_card FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS card_versions (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  card_id VARCHAR(128) NOT NULL,
  title VARCHAR(512) NOT NULL,
  summary TEXT NOT NULL,
  tags JSON NOT NULL,
  status VARCHAR(32) NOT NULL,
  content LONGTEXT NOT NULL,
  content_sha256 CHAR(64) NOT NULL,
  created_at DATETIME(6) NOT NULL,
  INDEX idx_versions_card (card_id, id),
  CONSTRAINT fk_versions_card FOREIGN KEY (card_id) REFERENCES cards(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS card_relations (
  from_card_id VARCHAR(128) NOT NULL,
  to_card_id VARCHAR(128) NOT NULL,
  relation_type VARCHAR(64) NOT NULL,
  created_at DATETIME(6) NOT NULL,
  PRIMARY KEY (from_card_id, to_card_id, relation_type),
  CONSTRAINT fk_rel_from FOREIGN KEY (from_card_id) REFERENCES cards(id) ON DELETE CASCADE,
  CONSTRAINT fk_rel_to FOREIGN KEY (to_card_id) REFERENCES cards(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS audit_log (
  id BIGINT AUTO_INCREMENT PRIMARY KEY,
  actor VARCHAR(128) NOT NULL,
  action VARCHAR(128) NOT NULL,
  target_id VARCHAR(128) NULL,
  detail TEXT NULL,
  created_at DATETIME(6) NOT NULL,
  INDEX idx_audit_created (created_at),
  INDEX idx_audit_target (target_id)
);

CREATE TABLE IF NOT EXISTS file_objects (
  id VARCHAR(128) PRIMARY KEY,
  object_kind ENUM('original','attachment','backup') NOT NULL,
  file_name VARCHAR(512) NOT NULL,
  mime_type VARCHAR(255) NULL,
  size_bytes BIGINT NOT NULL,
  sha256 CHAR(64) NOT NULL,
  content LONGBLOB NOT NULL,
  created_at DATETIME(6) NOT NULL,
  INDEX idx_file_kind (object_kind),
  INDEX idx_file_sha (sha256)
);
