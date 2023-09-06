DEFINE TABLE account SCHEMAFULL;

DEFINE FIELD username ON TABLE account TYPE string;
DEFINE FIELD email ON TABLE account TYPE string;
DEFINE FIELD password ON TABLE account TYPE string;
DEFINE FIELD created ON TABLE account TYPE datetime;
DEFINE FIELD last_login ON TABLE account TYPE option<datetime>;
DEFINE FIELD banned ON TABLE account TYPE bool;
DEFINE FIELD ban_reason ON TABLE account TYPE option<string>;
DEFINE FIELD active_session ON TABLE account TYPE option<record<session>>;

DEFINE INDEX usernameIndex ON TABLE account COLUMNS username UNIQUE;
DEFINE INDEX emailIndex ON TABLE account COLUMNS email UNIQUE;
