-- dept_table
CREATE TABLE public.dept_table (
	deptid smallserial NOT NULL,
	pid int2 NOT NULL,
	deptname varchar(64) NOT NULL,
	CONSTRAINT dept_table_pk PRIMARY KEY (deptid)
);

-- role_table
CREATE TABLE public.role_table (
	rolex int8 NOT NULL,
	roley int8 NOT NULL,
	rolepath varchar(128) NOT NULL,
	CONSTRAINT role_table_pk PRIMARY KEY (rolex, roley)
);


-- user_table
CREATE TABLE public.user_table (
	displayname varchar(64) NOT NULL,
	loginid bpchar(16) NOT NULL,
	rolex int8 NOT NULL,
	roley int8 NOT NULL,
	deptid int2 NOT NULL,
	passwd bpchar(16) NOT NULL,
	email varchar(64) NOT NULL,
	CONSTRAINT user_table_pk PRIMARY KEY (loginid),
	CONSTRAINT user_table_dept_table_fk FOREIGN KEY (deptid) REFERENCES dept_table(deptid),
	CONSTRAINT user_table_fk FOREIGN KEY (rolex, roley) REFERENCES role_table(rolex, roley)
);
