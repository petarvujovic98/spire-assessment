# SPVM local verifier

Initial mock implementation of a SPVM that can verify and execute `Mint` and `Transfer` transactions.

Could be further extended to be a `RPC` server that can accept transactions and respond to balance inquiries.

The state could also be extracted away to a trait which could have multiple implementors (in-memory, sqlite or full out distributed hosted PostgreSQL).
