DELETE FROM autoreplies
WHERE id NOT IN (
    SELECT MIN(id) FROM autoreplies
    GROUP BY trigger, reply
);

CREATE UNIQUE INDEX idx_autoreplies_pair ON autoreplies(trigger, reply);
