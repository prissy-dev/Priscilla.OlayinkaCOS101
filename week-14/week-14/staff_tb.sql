--
-- PostgreSQL database dump
--

\restrict AIkdfHZvVdXen5X32sDuUdOjbhV7wz5ERSdoPeyAmadjmnJ83Q8UjvZiBinx4SX

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
-- Name: staff employees_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.staff
    ADD CONSTRAINT employees_pkey PRIMARY KEY (staff_id);


--
-- PostgreSQL database dump complete
--

\unrestrict AIkdfHZvVdXen5X32sDuUdOjbhV7wz5ERSdoPeyAmadjmnJ83Q8UjvZiBinx4SX

