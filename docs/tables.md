### User
- username: String
- password: String
- name: String
- socials: Social[]
- books: BookListing[]

### Social
- platform: String
- username: String

### Book
- isbn: String
- title: String
- author: String
- blurb: String
- embeddings: String

### BookListing
- user: User
- book: Book
