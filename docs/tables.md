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
- isbn: Int
- title: String
- author: String
- embeddings: String

### BookListing
- user: User
- book: Book
