DELETE FROM trust
WHERE truster_id = $1 AND trustee_id = $2 RETURNING *;