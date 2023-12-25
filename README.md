# Book Logger

Features

- Scan Barcode / ISBN per Camera
- Track Reading-time per book (+ pages read => reading-speed calc)
- Start/End-Date of reading a book
- Take notes on books
    - more research needed
    - [How I Remember Everything I Read](https://www.youtube.com/watch?v=AjoxkxM_I5g)
- App for PlayStore
- Webapplication
- Hosting via AWS
- Multi-User
- Recommend a book via link “?recommend&book=XXXXX&sender=Felix
- Some Book-API needed + cover images
- Book Categories / Folders
- Also add papers
- Add eBook Storage
- Online-Reader for PDFs

Todo Steps

1. Create Frontend with local dummy-DB
    1. Create Basic Design
    2. Implement in VueJS
2. Create Server with API and DB + Multiuser
    1. Simple API with dummy DB
    2. Add DB

Tiers:

1. Add Books; Add Start+End-Date; Book-Covers
2. Taking Notes as simple Markdown
3. Track Reading-Time per Book + pages read
4. Multi-User
5. App
6. Research on how to take good notes and implement schema
7. Add Research Papers

Google Books API

- Suche:
    - [https://www.googleapis.com/books/v1/volumes?q=search+term](https://www.googleapis.com/books/v1/volumes?q=search+term)
- [https://www.googleapis.com/books/v1/volumes/ID](https://www.googleapis.com/books/v1/volumes/ID)
- Cover: volumeInfo > imageLinks > large