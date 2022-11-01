# To-do list app made in rust

#### Setup

Before `server` can be ran, a `.env` file must be created in the `server` directory.<br>

The following environment variables must also be set:<br>

- `DATABASE_URL`

#### Run Database

```sh
# Start PostgreSQL
docker run --rm -p 5432:5432 -e "POSTGRES_PASSWORD=postgres" --name pg postgres:14

# Start psql terminal
docker exec -it -u postgres pg psql
```

#### Ideas

- sorting & priority system based on
  - which assignments will most easily raise an okay grade
  - which assignments will most effectively raise the lowest grade (independent of how high the grade will be raised)
- grade criteria field (grade letter to percentage ratio)
- user-defined priority
- integrate Todoist API so I can easily move all my lists over 

#### To-Do

- [x] figure out rust structs and making instances of them
- [x] make task groups
- [ ] make the functions for interfacing and indexing with objects
- [ ] make function to index objects prettily
- [ ] track assignment priority
- [ ] track class priority
- [ ] postgres integration
- [ ] client developement
- [ ] client-server integration
- [ ] full stack deployment
- [ ] finish this to-do section (maybe make this list in the app as an example)
