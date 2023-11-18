create table transactions
(
    "hash"          text not null constraint transactions_pk primary key,
    "chain"         text,
    "aaType"        text,
    "status"        text,
    "timestamp"     timestamp
);