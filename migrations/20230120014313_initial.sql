CREATE TABLE indexer_state (
  function_name VARCHAR PRIMARY KEY, 
  current_block_height numeric(21,0) NOT NULL
);

CREATE TABLE indexer_storage (
  function_name VARCHAR NOT NULL, 
  key_name VARCHAR NOT NULL,
  value VARCHAR NOT NULL,
  PRIMARY KEY(function_name, key_name)
);

CREATE TABLE log_entries (
    id SERIAL PRIMARY KEY,
    timestamp TIMESTAMP NOT NULL,
    function_name VARCHAR NOT NULL,
    message TEXT NOT NULL
);
