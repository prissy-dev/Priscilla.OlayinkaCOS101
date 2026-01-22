--
-- PostgreSQL database dump
--

\restrict Kb8VOguainfvQrMdbgJPzRe1teqvbFc402OeWPXt1uOiq7TWxSfm1JrYCeceinX

-- Dumped from database version 17.7
-- Dumped by pg_dump version 17.7

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET transaction_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: department; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.department (
    dept_managerid integer NOT NULL,
    dno integer NOT NULL,
    dname text NOT NULL,
    dlocation character(50),
    pno text
);


ALTER TABLE public.department OWNER TO postgres;

--
-- Name: project; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.project (
    pno text NOT NULL,
    pname text NOT NULL,
    pduration interval NOT NULL,
    project_managerid integer NOT NULL
);


ALTER TABLE public.project OWNER TO postgres;

--
-- Name: staff; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.staff (
    staff_id integer NOT NULL,
    staff_name text NOT NULL,
    dno integer NOT NULL,
    staff_sal real NOT NULL,
    age integer NOT NULL,
    mobile character varying(15) NOT NULL
);


ALTER TABLE public.staff OWNER TO postgres;

--
-- Data for Name: department; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.department (dept_managerid, dno, dname, dlocation, pno) FROM stdin;
108	1	administration	ikeja                                             	44
101	2	account	egboda                                            	11
100	3	packaging	ajah                                              	44
120	4	research	v.i                                               	33
97	5	account	magodo                                            	22
122	6	operations	mile 2                                            	44
107	7	packaging	ketu                                              	55
\.


--
-- Data for Name: project; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.project (pno, pname, pduration, project_managerid) FROM stdin;
11	a	9 mons	102
22	b	1 year 2 mons	97
33	c	1 year 4 mons	120
44	d	2 years 1 mon	106
55	e	9 mons	107
\.


--
-- Data for Name: staff; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.staff (staff_id, staff_name, dno, staff_sal, age, mobile) FROM stdin;
100	mustapha ali	3	175000	32	08063285831
107	dankade aminat	7	380000	48	07090082812
108	josiah joshua	1	120000	30	08053189131
97	alokwe martin	5	550000	40	09023688832
102	mankinde mary	2	450000	55	09023487830
120	adeleke jane	4	200000	38	07061045862
122	osahon mark	6	320000	44	08022289842
104	kuti lawal	1	750000	35	09145689842
117	suleman ajayi	3	800000	50	7030089981
\.


--
-- Name: department department_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.department
    ADD CONSTRAINT department_pkey PRIMARY KEY (dept_managerid);


--
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_managerid);


--
-- PostgreSQL database dump complete
--

\unrestrict Kb8VOguainfvQrMdbgJPzRe1teqvbFc402OeWPXt1uOiq7TWxSfm1JrYCeceinX

