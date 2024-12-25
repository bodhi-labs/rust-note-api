
## Implement CRUD Functionality

- Now we can create handler for all CRUD functionality 
- and later we can attach them to our API.

Create handler.rs file
```shell
# create handler.rs
touch src/handler.rs
```

## Query with and/or without Macro

 - With `Macro`
 ```rust
 sqlx::query_as!(
    NoteModel,
    r#"SELECT * FROM notes ORDER by id LIMIT ? OFFSET ?"#,
    limit as i32,
    offset as i32
)
```
- If database is in wrong schema or offline, it can given an intellisense `error`

- using `query` without `macro`

```rust
sqlx::query_as::<_, NoteModel>(
    r#"SELECT * FROM notes ORDER by id LIMIT ? OFFSET ?"#
)
```


### API will be plan like this:

- `List` will have pagination page and limit parameter and use `FilterOptions` e.g., `GET` `http://127.0.0.1:8080/api/notes?page=0&limit=10`
= Create and Update will use `CreateNoteSchema` and `UpdateNoteSchema`
- `Read` and `Delete` will have `id` param and directly put in handler function


## Implement handler to Fetch All Records
function to handle fetching all Notes records ->
`note_list_handler` 

- Validate `limit` & `offset` from param given.
- Query with validated param and fetch all data and put into `notes`
- Convert query result `notes:NoteModel` to Response Model `NoteModelResponse` and put into `note_responses` .
- Create `json_response` and return it.

## Implement handler to insert a record (Create/Insert)
Create a function to handle inserting a record -> `create_note_handler`


- Note is using UUID as ID. Therefore we need to create ID first with this 
`let id = uuid::Uuid::new_v4().to_string();` .
- Then, we do Insert Query with payload (JSON Request already transform automatically into `CreateNoteScheme`).
- Then, we check error (e.g., `Duplicate Record`).
Get Inserted notes by Select data using ID we create first (optional).
- Return data with JSON format response using `serde_json::json!()` macro.

## Implement handler to get a record (Read)

Create a function to handle getting a single record by ID -> `get_note_handler`

- `ID` will automatically transformed from path route and passed to variable `id` .
- Don’t forget to checking the `error`. e.g., Database `error` or `No record` found. Otherwise, return the `Note data` with `JSON` format.

## Implement handler to update a record (Update)

create a function to handle updating a record by ID -> `edit_note_handler`


- We do Select `Query` get single Note by `ID`.
- `Parsing` data (if needed), in our case we need to parsing `is_published` from `Boolean` to `i8` and vice versa.
- Do `Query` update and check if there is an `error`.
- `Get` updated note data and `response` it as `JSON` format.

## Implement handler to delete a record (Delete)

create a function to handle deleting a record By ID -> `delete_note_handler`

- Quite straightforward, using `id` from route `path`, we do delete query by `ID`.
- Then we can check the return, `OK` as success or `NOT_FOUND` when `ID` not found or no data deleted.

## Register all handler to Axum Router

Our handler `handler.rs` should be looks like this, 
(it including moving the other handlers like health_check_handler)

## handler into route

Now we can create route.rs to implement our handler into route

## add CORS layer 

Then, update our main.rs to connect them all (and add CORS layer on it):

## Finally! 

You can try to test them out using REST Client or Postman or else:

- Create: POST http://localhost/8080/api/notes it will create a note with JSON Payload
- List: GET http://localhost/8080/api/notes listing all notes
- Get: GET http://localhost/8080/api/{ID} get a note by ID
- Update: PATCH http://localhost/8080/api/notes/{ID} update note by ID with JSON Payload
- Delete: DELETE http://localhost/8080/api/notes/{ID} delete a note by ID

- Bonus! You can use REST Client, a plugins on VSCode to test HTTP, create test.http and test them out

```http
### Health Check
GET http://localhost:8080/api/healthcheck

### List
GET http://localhost:8080/api/notes

### Create
POST http://localhost:8080/api/notes
content-type: application/json

{
    "title": "a note",
    "content": "here some reminder, mention @raditzlawliet",
    "is_published": true
}

### Read
GET http://localhost:8080/api/notes/05406abb-187e-4f00-9399-07872a6677f6

### Update
PATCH http://localhost:8080/api/notes/05406abb-187e-4f00-9399-07872a6677f6
content-type: application/json

{
    "content": "here some reminder, mention @raditzlawliet share and like"
}

### Delete
DELETE http://localhost:8080/api/notes/05406abb-187e-4f00-9399-07872a6677f6

```