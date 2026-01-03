# Bric-à-brac

## As a user I want to ...

### Auth

- [ ] (un-auth user) sign up using gmail
- [ ] (un-auth user) sign up using reddit
- [ ] (un-auth user) sign in using gmail
- [ ] (un-auth user) sign in using reddit
- [ ] sign out from my account

### Builder view

- [ ] create a graph
- [ ] (if admin) update the title of a graph
- [ ] (if admin) update the description of a graph
- [ ] (if admin) make a graph public and attach one subreddit
- [ ] (if admin) make a graph private
- [ ] (if admin) delete a graph
- [ ] (if admin) share a graph to another user, selecting the role
- [ ] (if admin) edit the role of a another user of a graph
- [ ] (if admin) delete the role of a another user of a graph
- [ ] (if admin or writer) add a node to the schema of a graph
- [ ] (if admin or writer) update a node of the schema of a graph
- [ ] (if admin or writer) delete a node of the schema of a graph
- [ ] (if admin or writer) add a relationship to the schema of a graph
- [ ] (if admin or writer) update a relationship of the schema of a graph
- [ ] (if admin or writer) delete a relationship of the schema of a graph
- [ ] (if admin or writer) add a node to a graph
- [ ] (if admin or writer) update a node of a graph
- [ ] (if admin or writer) delete a node of a graph
- [ ] (if admin or writer) add a relationship to a graph
- [ ] (if admin or writer) update a relationship of a graph
- [ ] (if admin or writer) delete a relationship of a graph
- [ ] list the graphs I'm admin of
- [ ] list the graphs I'm writer of
- [ ] list the graphs I'm reader of
- [ ] access a specific graph I'm admin of
- [ ] access a specific graph I'm writer of
- [ ] access a specific graph I'm reader of

### Explorer view

- [ ] (un-auth user) search for a public graph using a keyword
- [ ] (un-auth user) access a specific public graph
- [ ] (un-auth user) see the subreddit attached to a public graph
- [ ] (un-auth user) join the subreddit attached to a public graph
- [ ] bookmark a public graph
- [ ] un-bookmark a public graph
- [ ] list the public graphs I bookmarked 
- [ ] cheer a public graph
- [ ] un-cheer a public graph
- [ ] list the public graphs I cheered

## Architecture

### Databases

#### Knowledge

Memgraph database...

#### Metadata

PostgreSQL database...

### Micro-services

#### Metadata

> Rest API

**Middlewares**:
- auth          (AUT): retrieve auth credentials, no check
- access        (ACC): check user access to targeted ressource, 401/403/404
- validation    (VAL): check body from schema when inserting data, 400/409

**Endpoints**:
- [ ] POST      ()              /auth/register?service={gmail,reddit}
- [ ] POST      ()              /auth/login?service={gmail,reddit}
- [ ] DELETE    ()              /auth/logout
- [ ] GET       ()              /graphs/search?keyword=""
- [ ] GET       (ACC)           /graphs/:graph_id
- [ ] GET       (ACC)           /graphs/:graph_id/schema
- [ ] GET       (ACC)           /graphs/:graph_id/data
- [ ] GET       (AUT,ACC)       /users
- [ ] DELETE    (AUT,ACC)       /users
- [ ] POST      (AUT,ACC)       /graphs
- [ ] GET       (AUT,ACC)       /graphs/filter?role=[reader,writer,admin]&bookmarked={true,false}&cheered={true,fase}
- [ ] PATCH     (AUT,ACC)       /graphs/:graph_id
- [ ] DELETE    (AUT,ACC)       /graphs/:graph_id
- [ ] POST      (AUT,ACC)       /graphs/:graph_id/schemas/nodes
- [ ] PATCH     (AUT,ACC)       /graphs/:graph_id/schemas/nodes/:node_id
- [ ] DELETE    (AUT,ACC)       /graphs/:graph_id/schemas/nodes/:node_id
- [ ] POST      (AUT,ACC)       /graphs/:graph_id/schemas/relationships
- [ ] PATCH     (AUT,ACC)       /graphs/:graph_id/schemas/relationships/:relationship_id
- [ ] DELETE    (AUT,ACC)       /graphs/:graph_id/schemas/relationships/:relationship_id
- [ ] POST      (AUT,ACC,VAL)   /graphs/:graph_id/data/nodes                             (body: schema_node_id)
- [ ] PATCH     (AUT,ACC,VAL)   /graphs/:graph_id/data/nodes/:node_id                    (body: schema_node_id)
- [ ] DELETE    (AUT,ACC)       /graphs/:graph_id/data/nodes/:node_id
- [ ] POST      (AUT,ACC,VAL)   /graphs/:graph_id/data/relationships                     (body: schema_relationship_id)
- [ ] PATCH     (AUT,ACC,VAL)   /graphs/:graph_id/data/relationships/:relationship_id    (body: schema_relationship_id)
- [ ] DELETE    (AUT,ACC)       /graphs/:graph_id/data/relationships/:relationship_id
- [ ] GET       (AUT,ACC)       /graphs/:graph_id/accesses
- [ ] POST      (AUT,ACC)       /graphs/:graph_id/accesses
- [ ] PATCH     (AUT,ACC)       /graphs/:graph_id/accesses/:access_id
- [ ] DELETE    (AUT,ACC)       /graphs/:graph_id/accesses/:access_id
- [ ] POST      (AUT,ACC)       /graphs/:graph_id/bookmarks
- [ ] DELETE    (AUT,ACC)       /graphs/:graph_id/bookmarks
- [ ] POST      (AUT,ACC)       /graphs/:graph_id/cheers
- [ ] DELETE    (AUT,ACC)       /graphs/:graph_id/cheers

#### Knowledge

> Communicates via RPC with the metadata micro-service
> Un-accessible from the oustide

### User interface

...

## TODO

### Don't forget to ...

- [ ] create reddit account & subreddit (to explain this project and the roadmap)

### and to think about ...

- branches collaboration (like github)
- real time collaboration on same branche
- link subreddit to specific node (comments on graph)
- integration with x
- integration with discord
- integration with linkedin
- integration with medium
- integration with github
- integration with youtube
- integration wih social medias ?
- (if admin) archive a graph
- sort graphs lists by title
- sort graphs lists by cheers
- sort graphs lists by creation date
- sort graphs lists by update date
- sort graphs lists by nb of node
- sort graphs lists by nb of relationships
- sort graphs lists by nb of subreddit discussions
- graph topics
- pay to use features / ratings/limits (plans, pricing, pay)
- analytics
- separate database per project (very very large graphs - enterprise plan)
