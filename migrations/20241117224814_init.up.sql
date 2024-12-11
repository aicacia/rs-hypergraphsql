CREATE TABLE "nodes" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "uri" TEXT NOT NULL,
  "data" TEXT NOT NULL,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;
CREATE UNIQUE INDEX "nodes_id_unique_idx" ON "nodes" ("id");
CREATE INDEX "nodes_uri_idx" ON "nodes" ("uri");

CREATE TABLE "edges" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "from_node_id" INTEGER NOT NULL,
  "to_node_id" INTEGER NOT NULL,
  "uri" TEXT NOT NULL,
  "data" TEXT,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("from_node_id") REFERENCES "nodes" ("id") ON DELETE CASCADE,
  FOREIGN KEY ("to_node_id") REFERENCES "nodes" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "edges_id_unique_idx" ON "edges" ("id");
CREATE UNIQUE INDEX "edges_from_node_id_to_node_id_unique_idx" ON "edges" ("from_node_id", "to_node_id");
CREATE INDEX "edges_uri_idx" ON "edges" ("uri");