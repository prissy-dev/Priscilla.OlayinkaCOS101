--
-- PostgreSQL database dump
--

\restrict T7As8XNG0E9Q89x7INQyTvcA4eQJPXdZNSyZdfnoXmbazcWHO1zl84S6nbLByze

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
-- Name: project project_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.project
    ADD CONSTRAINT project_pkey PRIMARY KEY (project_managerid);


--
-- PostgreSQL database dump complete
--

\unrestrict T7As8XNG0E9Q89x7INQyTvcA4eQJPXdZNSyZdfnoXmbazcWHO1zl84S6nbLByze

