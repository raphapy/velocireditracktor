create extension "uuid-ossp";
-- Drop table

-- DROP TABLE public.redirects

CREATE TABLE public.redirects (
	redirect_id uuid NOT NULL DEFAULT uuid_generate_v4(),
	redirect_ts timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
	caller_ip varchar(300) NOT NULL,
	called_url varchar(2000) NULL,
	request_metadata jsonb NULL,
	CONSTRAINT redirects_pk PRIMARY KEY (redirect_id)
);